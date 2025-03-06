#![cfg_attr(not(feature = "std"), no_std)]

pub mod bindings;
pub mod color;
pub mod elements;
pub mod errors;
pub mod id;
pub mod layout;
pub mod math;
pub mod render_commands;
pub mod text;

mod mem;
pub mod renderers;

use core::marker::PhantomData;

pub use crate::bindings::*;
use errors::Error;
use id::Id;
use math::{BoundingBox, Dimensions, Vector2};
use render_commands::RenderCommand;

pub use color::Color;

#[cfg(feature = "std")]
use text::TextConfig;

use text::TextElementConfig;
#[derive(Copy, Clone)]
pub struct Declaration<'render, ImageElementData: 'render, CustomElementData: 'render> {
    inner: Clay_ElementDeclaration,
    _phantom: PhantomData<(&'render CustomElementData, &'render ImageElementData)>,
}

impl<'render, ImageElementData: 'render, CustomElementData: 'render>
    Declaration<'render, ImageElementData, CustomElementData>
{
    #[inline]
    pub fn new() -> Self {
        crate::mem::zeroed_init()
    }

    #[inline]
    pub fn background_color(&mut self, color: Color) -> &mut Self {
        self.inner.backgroundColor = color.into();
        self
    }

    #[inline]
    pub fn scroll(&mut self, horizontal: bool, vertical: bool) -> &mut Self {
        self.inner.scroll.horizontal = horizontal;
        self.inner.scroll.vertical = vertical;
        self
    }

    #[inline]
    pub fn id(&mut self, id: Id) -> &mut Self {
        self.inner.id = id.id;
        self
    }

    #[inline]
    pub fn custom_element(&mut self, data: &'render CustomElementData) -> &mut Self {
        self.inner.custom.customData = data as *const CustomElementData as _;
        self
    }

    #[inline]
    pub fn layout(
        &mut self,
    ) -> layout::LayoutBuilder<'_, 'render, ImageElementData, CustomElementData> {
        layout::LayoutBuilder::new(self)
    }

    #[inline]
    pub fn image(
        &mut self,
    ) -> elements::ImageBuilder<'_, 'render, ImageElementData, CustomElementData> {
        elements::ImageBuilder::new(self)
    }

    #[inline]
    pub fn floating(
        &mut self,
    ) -> elements::FloatingBuilder<'_, 'render, ImageElementData, CustomElementData> {
        elements::FloatingBuilder::new(self)
    }

    #[inline]
    pub fn border(
        &mut self,
    ) -> elements::BorderBuilder<'_, 'render, ImageElementData, CustomElementData> {
        elements::BorderBuilder::new(self)
    }

    #[inline]
    pub fn corner_radius(
        &mut self,
    ) -> elements::CornerRadiusBuilder<'_, 'render, ImageElementData, CustomElementData> {
        elements::CornerRadiusBuilder::new(self)
    }
}

impl<ImageElementData, CustomElementData> Default
    for Declaration<'_, ImageElementData, CustomElementData>
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
unsafe extern "C" fn measure_text_trampoline_user_data<'a, F, T>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: *mut core::ffi::c_void,
) -> Clay_Dimensions
where
    F: Fn(&str, &TextConfig, &'a mut T) -> Dimensions + 'a,
    T: 'a,
{
    let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        text_slice.chars as *const u8,
        text_slice.length as _,
    ));

    let closure_and_data: &mut (F, T) = &mut *(user_data as *mut (F, T));
    let text_config = TextConfig::from(*config);
    let (callback, data) = closure_and_data;
    callback(text, &text_config, data).into()
}

#[cfg(feature = "std")]
unsafe extern "C" fn measure_text_trampoline<'a, F>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: *mut core::ffi::c_void,
) -> Clay_Dimensions
where
    F: Fn(&str, &TextConfig) -> Dimensions + 'a,
{
    let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        text_slice.chars as *const u8,
        text_slice.length as _,
    ));

    let callback: &mut F = &mut *(user_data as *mut F);
    let text_config = TextConfig::from(*config);
    callback(text, &text_config).into()
}

unsafe extern "C" fn error_handler(error_data: Clay_ErrorData) {
    let error: Error = error_data.into();
    panic!("Clay Error: (type: {:?}) {}", error.type_, error.text);
}

#[allow(dead_code)]
pub struct Clay {
    /// Memory used internally by clay
    #[cfg(feature = "std")]
    _memory: Vec<u8>,
    context: *mut Clay_Context,
    /// Memory used internally by clay. The caller is responsible for managing this memory in
    /// no_std case.
    #[cfg(not(feature = "std"))]
    _memory: *const core::ffi::c_void,
    /// Stores the raw pointer to the callback data for later cleanup
    text_measure_callback: Option<*const core::ffi::c_void>,
}

pub struct ClayLayoutScope<'clay, 'render, ImageElementData, CustomElementData> {
    clay: &'clay mut Clay,
    _phantom: core::marker::PhantomData<(&'render ImageElementData, &'render CustomElementData)>,
    dropped: bool,
}

impl<'render, 'clay: 'render, ImageElementData: 'render, CustomElementData: 'render>
    ClayLayoutScope<'clay, 'render, ImageElementData, CustomElementData>
{
    /// Create an element, passing its config and a function to add childrens
    pub fn with<
        F: FnOnce(&mut ClayLayoutScope<'clay, 'render, ImageElementData, CustomElementData>),
    >(
        &mut self,
        declaration: &Declaration<'render, ImageElementData, CustomElementData>,
        f: F,
    ) {
        unsafe {
            Clay_SetCurrentContext(self.clay.context);
            Clay__OpenElement();
            Clay__ConfigureOpenElement(declaration.inner);
        }

        f(self);

        unsafe {
            Clay__CloseElement();
        }
    }

    pub fn with_styling<
        G: FnOnce(
            &ClayLayoutScope<'clay, 'render, ImageElementData, CustomElementData>,
        ) -> Declaration<'render, ImageElementData, CustomElementData>,
        F: FnOnce(&ClayLayoutScope<'clay, 'render, ImageElementData, CustomElementData>),
    >(
        &self,
        g: G,
        f: F,
    ) {
        unsafe {
            Clay_SetCurrentContext(self.clay.context);
            Clay__OpenElement();
        }

        let declaration = g(self);

        unsafe {
            Clay__ConfigureOpenElement(declaration.inner);
        }

        f(self);

        unsafe {
            Clay__CloseElement();
        }
    }

    pub fn end(&mut self,
    ) -> impl Iterator<Item = RenderCommand<'render, ImageElementData, CustomElementData>> {
        let array = unsafe { Clay_EndLayout() };
        self.dropped = true;
        let slice = unsafe { core::slice::from_raw_parts(array.internalArray, array.length as _) };
        slice
            .iter()
            .map(|command| unsafe { RenderCommand::from_clay_render_command(*command) })
    }

    /// Generates a unique ID based on the given `label`.
    ///
    /// This ID is global and must be unique across the entire scope.
    #[inline]
    pub fn id(&self, label: &'render str) -> id::Id {
        id::Id::new(label)
    }

    /// Generates a unique indexed ID based on the given `label` and `index`.
    ///
    /// This is useful when multiple elements share the same label but need distinct IDs.
    #[inline]
    pub fn id_index(&self, label: &'render str, index: u32) -> id::Id {
        id::Id::new_index(label, index)
    }

    /// Generates a locally unique ID based on the given `label`.
    ///
    /// The ID is unique within a specific local scope but not globally.
    #[inline]
    pub fn id_local(&self, label: &'render str) -> id::Id {
        id::Id::new_index_local(label, 0)
    }

    /// Generates a locally unique indexed ID based on the given `label` and `index`.
    ///
    /// This is useful for differentiating elements within a local scope while keeping their labels consistent.
    #[inline]
    pub fn id_index_local(&self, label: &'render str, index: u32) -> id::Id {
        id::Id::new_index_local(label, index)
    }

    /// Adds a text element to the current open element or to the root layout
    pub fn text(&self, text: &'render str, config: TextElementConfig) {
        unsafe { Clay__OpenTextElement(text.into(), config.into()) };
    }

    pub fn hovered(&self) -> bool {
        unsafe { Clay_Hovered() }
    }

    pub fn pointer_over(&self, cfg: Id) -> bool {
        unsafe { Clay_PointerOver(cfg.id) }
    }
}

impl<ImageElementData, CustomElementData> Drop
    for ClayLayoutScope<'_, '_, ImageElementData, CustomElementData>
{
    fn drop(&mut self) {
        if !self.dropped {
            unsafe {
                Clay_EndLayout();
            }
        }
    }
}

impl Clay {
    pub fn begin<'render, ImageElementData: 'render, CustomElementData: 'render>(
        &mut self,
    ) -> ClayLayoutScope<'_, 'render, ImageElementData, CustomElementData> {
        unsafe { Clay_BeginLayout() };
        ClayLayoutScope {
            clay: self,
            _phantom: core::marker::PhantomData,
            dropped: false,
        }
    }

    #[cfg(feature = "std")]
    pub fn new(dimensions: Dimensions) -> Self {
        let memory_size = Self::required_memory_size();
        let memory = vec![0; memory_size];
        let context;

        unsafe {
            let arena =
                Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory.as_ptr() as _);

            context = Clay_Initialize(
                arena,
                dimensions.into(),
                Clay_ErrorHandler {
                    errorHandlerFunction: Some(error_handler),
                    userData: std::ptr::null_mut(),
                },
            );
        }

        Self {
            _memory: memory,
            context,
            text_measure_callback: None,
        }
    }

    #[cfg(not(feature = "std"))]
    pub unsafe fn new_with_memory(dimensions: Dimensions, memory: *mut core::ffi::c_void) -> Self {
        let memory_size = Self::required_memory_size();
        let arena = Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory);

        let context = Clay_Initialize(
            arena,
            dimensions.into(),
            Clay_ErrorHandler {
                errorHandlerFunction: Some(error_handler),
                userData: core::ptr::null_mut(),
            },
        );

        Self {
            _memory: memory,
            context,
            text_measure_callback: None,
        }
    }

    /// Wrapper for `Clay_MinMemorySize`, returns the minimum required memory by clay
    pub fn required_memory_size() -> usize {
        unsafe { Clay_MinMemorySize() as usize }
    }

    /// Set the callback for text measurement with user data
    #[cfg(feature = "std")]
    pub fn set_measure_text_function_user_data<'clay, F, T>(
        &'clay mut self,
        userdata: T,
        callback: F,
    ) where
        F: Fn(&str, &TextConfig, &'clay mut T) -> Dimensions + 'static,
        T: 'clay,
    {
        // Box the callback and userdata together
        let boxed = Box::new((callback, userdata));

        // Get a raw pointer to the boxed data
        let user_data_ptr = Box::into_raw(boxed) as _;

        // Register the callback with the external C function
        unsafe {
            Self::set_measure_text_function_unsafe(
                measure_text_trampoline_user_data::<F, T>,
                user_data_ptr,
            );
        }

        // Store the raw pointer for later cleanup
        self.text_measure_callback = Some(user_data_ptr as *const core::ffi::c_void);
    }

    /// Set the callback for text measurement
    #[cfg(feature = "std")]
    pub fn set_measure_text_function<F>(&mut self, callback: F)
    where
        F: Fn(&str, &TextConfig) -> Dimensions + 'static,
    {
        // Box the callback and userdata together
        let boxed = Box::new(callback);

        // Get a raw pointer to the boxed data
        let user_data_ptr = Box::into_raw(boxed) as *mut core::ffi::c_void;

        // Register the callback with the external C function
        unsafe {
            Self::set_measure_text_function_unsafe(measure_text_trampoline::<F>, user_data_ptr);
        }

        // Store the raw pointer for later cleanup
        self.text_measure_callback = Some(user_data_ptr as *const core::ffi::c_void);
    }

    /// Set the callback for text measurement with user data.
    /// # Safety
    /// This function is unsafe because it sets a callback function without any error checking
    pub unsafe fn set_measure_text_function_unsafe(
        callback: unsafe extern "C" fn(
            Clay_StringSlice,
            *mut Clay_TextElementConfig,
            *mut core::ffi::c_void,
        ) -> Clay_Dimensions,
        user_data: *mut core::ffi::c_void,
    ) {
        Clay_SetMeasureTextFunction(Some(callback), user_data);
    }

    /// Sets the maximum number of element that clay supports
    /// **Use only if you know what you are doing or your getting errors from clay**
    pub fn max_element_count(&mut self, max_element_count: u32) {
        unsafe {
            Clay_SetMaxElementCount(max_element_count as _);
        }
    }
    /// Sets the capacity of the cache used for text in the measure text function
    /// **Use only if you know what you are doing or your getting errors from clay**
    pub fn max_measure_text_cache_word_count(&self, count: u32) {
        unsafe {
            Clay_SetMaxElementCount(count as _);
        }
    }

    /// Enables or disables the debug mode of clay
    pub fn set_debug_mode(&self, enable: bool) {
        unsafe {
            Clay_SetDebugModeEnabled(enable);
        }
    }

    /// Sets the dimensions of the global layout, use if, for example the window size you render to
    /// changed
    pub fn set_layout_dimensions(&self, dimensions: Dimensions) {
        unsafe {
            Clay_SetLayoutDimensions(dimensions.into());
        }
    }
    /// Updates the state of the pointer for clay. Used to update scroll containers and for
    /// interactions functions
    pub fn pointer_state(&self, position: Vector2, is_down: bool) {
        unsafe {
            Clay_SetPointerState(position.into(), is_down);
        }
    }
    pub fn update_scroll_containers(
        &self,
        drag_scrolling_enabled: bool,
        scroll_delta: Vector2,
        delta_time: f32,
    ) {
        unsafe {
            Clay_UpdateScrollContainers(drag_scrolling_enabled, scroll_delta.into(), delta_time);
        }
    }

    pub fn scroll_container_data(&self, id: Id) -> Option<Clay_ScrollContainerData> {
        unsafe {
            Clay_SetCurrentContext(self.context);
            let scroll_container_data = Clay_GetScrollContainerData(id.id);

            if scroll_container_data.found {
                Some(scroll_container_data)
            } else {
                None
            }
        }
    }

    /// Returns if the current element you are creating is hovered
    pub fn hovered(&self) -> bool {
        unsafe { Clay_Hovered() }
    }

    pub fn pointer_over(&self, cfg: Id) -> bool {
        unsafe { Clay_PointerOver(cfg.id) }
    }

    fn element_data(id: Id) -> Clay_ElementData {
        unsafe { Clay_GetElementData(id.id) }
    }

    pub fn bounding_box(&self, id: Id) -> Option<BoundingBox> {
        let element_data = Self::element_data(id);

        if element_data.found {
            Some(element_data.boundingBox.into())
        } else {
            None
        }
    }
}

#[cfg(feature = "std")]
impl Drop for Clay {
    fn drop(&mut self) {
        unsafe {
            if let Some(ptr) = self.text_measure_callback {
                let _ = Box::from_raw(ptr as *mut (usize, usize));
            }
        }
    }
}

impl From<&str> for Clay_String {
    fn from(value: &str) -> Self {
        Self {
            length: value.len() as _,
            chars: value.as_ptr() as _,
        }
    }
}

impl From<Clay_String> for &str {
    fn from(value: Clay_String) -> Self {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                value.chars as *const u8,
                value.length as _,
            ))
        }
    }
}

impl From<Clay_StringSlice> for &str {
    fn from(value: Clay_StringSlice) -> Self {
        unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                value.chars as *const u8,
                value.length as _,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use color::Color;
    use layout::{Padding, Sizing};

    #[test]
    #[rustfmt::skip]
    fn test_begin() {
        let mut callback_data = 0u32;

        let mut clay = Clay::new(Dimensions::new(800.0, 600.0));

        clay.set_measure_text_function_user_data(&mut callback_data, |text, _config, data| {
            println!(
                "set_measure_text_function_user_data {:?} count {:?}",
                text, data
            );
            **data += 1;
            Dimensions::default()
        });

        let clay = clay.begin::<(), ()>();

        clay.with(&Declaration::new()
            .id(clay.id("parent_rect"))
            .layout()
                .width(Sizing::Fixed(100.0))
                .height(Sizing::Fixed(100.0))
                .padding(Padding::all(10))
                .end()
            .background_color(Color::rgb(255., 255., 255.)), |clay|
        {
            clay.with(&Declaration::new()
                .layout()
                    .width(Sizing::Fixed(100.0))
                    .height(Sizing::Fixed(100.0))
                    .padding(Padding::all(10))
                    .end()
                .background_color(Color::rgb(255., 255., 255.)), |clay| 
            {
                clay.with(&Declaration::new()
                    .id(clay.id("rect_under_rect"))
                    .layout()
                        .width(Sizing::Fixed(100.0))
                        .height(Sizing::Fixed(100.0))
                        .padding(Padding::all(10))
                        .end()
                    .background_color(Color::rgb(255., 255., 255.)), |clay| 
                    {
                        clay.text("test", TextConfig::new()
                            .color(Color::rgb(255., 255., 255.))
                            .font_size(24)
                            .end());
                    },
                );
            });
        });

        clay.with(&Declaration::new()
            .id(clay.id_index("border_container", 1))
            .layout()
                .padding(Padding::all(16))
                .end()
            .border()
                .color(Color::rgb(255., 255., 0.))
                .all_directions(2)
                .end()
            .corner_radius().all(10.0).end(), |clay|
        {
            clay.with(&Declaration::new()
                .id(clay.id("rect_under_border"))
                .layout()
                    .width(Sizing::Fixed(50.0))
                    .height(Sizing::Fixed(50.0))
                    .end()
                .background_color(Color::rgb(0., 255., 255.)), |_clay| {},
            );
        });

        let items = clay.end();

        for item in items {
            println!(
                "id: {}\nbbox: {:?}\nconfig: {:?}",
                item.id, item.bounding_box, item.config,
            );
        }
    }
}
