#![cfg_attr(not(feature = "std"), no_std)]

pub mod bindings;

pub mod color;
pub mod elements;
pub mod errors;
pub mod id;
pub mod layout;
pub mod math;
pub mod render_commands;

mod mem;

use crate::elements::text::Text;
use elements::{text::TextElementConfig, ElementConfigType};
use errors::Error;
use math::{BoundingBox, Dimensions, Vector2};
use render_commands::RenderCommand;

use crate::bindings::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TypedConfig {
    pub config_memory: *const u8,
    pub id: Clay_ElementId,
    pub config_type: ElementConfigType,
}

#[cfg(feature = "std")]
unsafe extern "C" fn measure_text_trampoline_user_data<'a, F, T>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: usize,
) -> Clay_Dimensions
where
    F: Fn(&str, &Text, &'a mut T) -> Dimensions + 'a,
    T: 'a,
{
    let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        text_slice.chars as *const u8,
        text_slice.length as _,
    ));

    let closure_and_data: &mut (F, T) = &mut *(user_data as *mut (F, T));
    let text_config = Text::from(*config);
    let (callback, data) = closure_and_data;
    callback(text, &text_config, data).into()
}

#[cfg(feature = "std")]
unsafe extern "C" fn measure_text_trampoline<'a, F>(
    text_slice: Clay_StringSlice,
    config: *mut Clay_TextElementConfig,
    user_data: usize,
) -> Clay_Dimensions
where
    F: Fn(&str, &Text) -> Dimensions + 'a,
{
    let text = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        text_slice.chars as *const u8,
        text_slice.length as _,
    ));

    let callback: &mut F = &mut *(user_data as *mut F);
    let text_config = Text::from(*config);
    callback(text, &text_config).into()
}

unsafe extern "C" fn error_handler(error_data: Clay_ErrorData) {
    let error: Error = error_data.into();
    panic!("Clay Error: (type: {:?}) {}", error.type_, error.text);
}

pub struct DataRef<'a> {
    pub(crate) ptr: *const core::ffi::c_void,
    _phantom: core::marker::PhantomData<&'a ()>,
}

#[allow(dead_code)]
pub struct Clay<'a> {
    /// Memory used internally by clay
    #[cfg(feature = "std")]
    _memory: Vec<u8>,
    context: *mut Clay_Context,
    /// Memory used internally by clay. The caller is responsible for managing this memory in
    /// no_std case.
    #[cfg(not(feature = "std"))]
    _memory: *const core::ffi::c_void,
    /// Phantom data to keep the lifetime of the memory
    _phantom: core::marker::PhantomData<&'a ()>,
    /// Stores the raw pointer to the callback data for later cleanup
    text_measure_callback: Option<*const core::ffi::c_void>,
}

impl<'a> Clay<'a> {
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
                    userData: 0,
                },
            );
        }

        Self {
            _memory: memory,
            context,
            _phantom: core::marker::PhantomData,
            text_measure_callback: None,
        }
    }

    /// Get a reference to the data to pass to clay or the builders. This is to ensure that the
    /// data is not dropped before clay is done with it.
    pub fn data<T>(&self, data: &T) -> DataRef<'a> {
        DataRef {
            ptr: data as *const T as *const core::ffi::c_void,
            _phantom: core::marker::PhantomData,
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
                userData: 0,
            },
        );

        Self {
            _memory: memory,
            context,
            _phantom: core::marker::PhantomData,
            text_measure_callback: None,
        }
    }

    /// Wrapper for `Clay_MinMemorySize`, returns the minimum required memory by clay
    pub fn required_memory_size() -> usize {
        unsafe { Clay_MinMemorySize() as usize }
    }

    /// Set the callback for text measurement with user data
    #[cfg(feature = "std")]
    pub fn set_measure_text_function_user_data<F, T>(&mut self, userdata: T, callback: F)
    where
        F: Fn(&str, &Text, &'a mut T) -> Dimensions + 'static,
        T: 'a,
    {
        // Box the callback and userdata together
        let boxed = Box::new((callback, userdata));

        // Get a raw pointer to the boxed data
        let user_data_ptr = Box::into_raw(boxed) as usize;

        // Register the callback with the external C function
        unsafe {
            Clay_SetMeasureTextFunction(
                Some(measure_text_trampoline_user_data::<F, T>),
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
        F: Fn(&str, &Text) -> Dimensions + 'static,
    {
        // Box the callback and userdata together
        let boxed = Box::new(callback);

        // Get a raw pointer to the boxed data
        let user_data_ptr = Box::into_raw(boxed) as usize;

        // Register the callback with the external C function
        unsafe {
            Clay_SetMeasureTextFunction(Some(measure_text_trampoline::<F>), user_data_ptr);
        }

        // Store the raw pointer for later cleanup
        self.text_measure_callback = Some(user_data_ptr as *const core::ffi::c_void);
    }

    /// Sets the maximum number of element that clay supports
    /// **Use only if you know what you are doing or your getting errors from clay**
    pub fn max_element_count(&self, max_element_count: u32) {
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
    pub fn enable_debug_mode(&self, enable: bool) {
        unsafe {
            Clay_SetDebugModeEnabled(enable);
        }
    }

    /// Sets the dimensions of the global layout, use if, for example the window size you render to
    /// changed
    pub fn layout_dimensions(&self, dimensions: Dimensions) {
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

    /// Returns if the current element you are creating is hovered
    pub fn hovered(&self) -> bool {
        unsafe { Clay_Hovered() }
    }

    pub fn pointer_over(&self, cfg: TypedConfig) -> bool {
        unsafe { Clay_PointerOver(cfg.id) }
    }

    fn get_element_data(id: TypedConfig) -> Clay_ElementData {
        unsafe { Clay_GetElementData(id.id) }
    }

    pub fn get_bounding_box(&self, id: TypedConfig) -> Option<BoundingBox> {
        let element_data = Self::get_element_data(id);

        if element_data.found {
            Some(element_data.boundingBox.into())
        } else {
            None
        }
    }

    pub fn begin(&self) {
        unsafe { Clay_BeginLayout() };
    }

    pub fn end(&self) -> impl Iterator<Item = RenderCommand> {
        let array = unsafe { Clay_EndLayout() };
        let slice = unsafe { core::slice::from_raw_parts(array.internalArray, array.length as _) };
        slice.iter().map(|command| RenderCommand::from(*command))
    }

    /// Create an element, passing it's config and a function to add childrens
    /// ```
    /// // TODO: Add Example
    /// ```
    pub fn with<F: FnOnce(&Clay), const N: usize>(
        &self,
        id: Option<&'a str>,
        configs: [TypedConfig; N],
        f: F,
    ) {
        // Mapping `id: Option<&str>` to `Option<(&str, u32)>` with index being zero
        let id: Option<(&str, u32)> = id.map(|name| (name, 0));
        self.with_id_index(id, configs, f)
    }

    /// Create an element, passing it's config and a function to add childrens
    /// ```
    /// // TODO: Add Example
    /// ```
    pub fn with_id_index<F: FnOnce(&Clay), const N: usize>(
        &self,
        id: Option<(&'a str, u32)>,
        configs: [TypedConfig; N],
        f: F,
    ) {
        unsafe {
            Clay_SetCurrentContext(self.context);
            Clay__OpenElement()
        };

        if let Some((id_name, index)) = id {
            unsafe {
                Clay__AttachId(Clay__HashString(id_name.into(), index, 0));
            }
        }

        for config in configs {
            if config.config_type == ElementConfigType::Layout as _ {
                unsafe { Clay__AttachLayoutConfig(config.config_memory as _) };
            } else {
                unsafe {
                    Clay__AttachElementConfig(
                        core::mem::transmute::<*const u8, bindings::Clay_ElementConfigUnion>(
                            config.config_memory,
                        ),
                        config.config_type as _,
                    )
                };
            }
        }

        unsafe { Clay__ElementPostConfiguration() };

        f(self);

        unsafe {
            Clay__CloseElement();
        }
    }

    /// Adds a text element to the current open element or to the root layout
    pub fn text(&self, text: &str, config: TextElementConfig) {
        unsafe { Clay__OpenTextElement(text.into(), config.into()) };
    }
}

#[cfg(feature = "std")]
impl Drop for Clay<'_> {
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

#[cfg(test)]
mod tests {
    use color::Color;
    use elements::{
        containers::border::BorderContainer, rectangle::Rectangle, text::Text, CornerRadius,
    };
    use layout::{padding::Padding, sizing::Sizing, Layout};

    use super::*;

    /*
    #[test]
    fn test_create() {
        let _clay = Clay::new(800.0, 600.0);
    }
    */

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

        clay.begin();

        clay.with(Some("parent_rect"), [
            Layout::new()
                .width(Sizing::Fixed(100.0))
                .height(Sizing::Fixed(100.0))
                .padding(Padding::all(10))
                .end(),
            Rectangle::new().color(Color::rgb(255., 255., 255.)).end()], |clay| 
        {
            clay.with(None, [
                Layout::new()
                    .width(Sizing::Fixed(100.0))
                    .height(Sizing::Fixed(100.0))
                    .padding(Padding::all(10))
                    .end(),
                Rectangle::new().color(Color::rgb(255., 255., 255.)).end()], |clay| 
            {
                clay.with(Some("rect_under_rect"), [
                    Layout::new()
                        .width(Sizing::Fixed(100.0))
                        .height(Sizing::Fixed(100.0))
                        .padding(Padding::all(10))
                        .end(),
                    Rectangle::new().color(Color::rgb(255., 255., 255.)).end()], |clay| 
                    {
                        clay.text("test", Text::new()
                            .color(Color::rgb(255., 255., 255.))
                            .font_size(24)
                            .end());
                    },
                );
            });
        });

        clay.with_id_index(Some(("Border_container", 1)), [
            Layout::new().padding(Padding::all(16)).end(),
            BorderContainer::new()
                .all_directions(2, Color::rgb(255., 255., 0.))
                .corner_radius(CornerRadius::All(25.))
                .end()], |clay| 
        {
            clay.with(Some("rect_under_border"), [
                Layout::new()
                    .width(Sizing::Fixed(50.0))
                    .height(Sizing::Fixed(50.0))
                    .end(),
                Rectangle::new().color(Color::rgb(0., 255., 255.)).end()], |_clay| {},
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

    /*
    #[test]
    fn clay_floating_attach_point_type() {
        assert_eq!(FloatingAttachPointType::LeftTop as c_uchar, 0 as c_uchar);
    }

    #[test]
    fn clay_element_config_type() {
        assert_eq!(ElementConfigType::BorderContainer as c_uchar, 2 as c_uchar);
    }

    */

    #[test]
    fn size_of_union() {
        assert_eq!(
            core::mem::size_of::<Clay_SizingAxis__bindgen_ty_1>(),
            core::mem::size_of::<Clay_SizingAxis__bindgen_ty_1>()
        )
    }
}
