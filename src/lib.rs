#![cfg_attr(not(feature = "std"), no_std)]

pub mod bindings;

pub mod color;
pub mod elements;
pub mod id;
pub mod layout;
pub mod math;
pub mod render_commands;

mod mem;

use elements::{text::TextElementConfig, ElementConfigType};
use math::{Dimensions, Vector2};
use render_commands::RenderCommand;

use crate::bindings::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TypedConfig {
    pub config_memory: *const u8,
    pub id: Clay_ElementId,
    pub config_type: ElementConfigType,
}

pub type MeasureTextFunction = fn(text: &str, config: TextElementConfig) -> Dimensions;

// Is used to store the current callback for measuring text
static mut MEASURE_TEXT_HANDLER: Option<MeasureTextFunction> = None;

// Converts the args and calls `MEASURE_TEXT_HANDLER`. Passed to clay with `Clay_SetMeasureTextFunction`
unsafe extern "C" fn measure_text_handle(
    str: *mut Clay_String,
    config: *mut Clay_TextElementConfig,
) -> Clay_Dimensions {
    match MEASURE_TEXT_HANDLER {
        Some(func) => func((*str.as_ref().unwrap()).into(), config.into()).into(),
        None => Clay_Dimensions {
            width: 0.0,
            height: 0.0,
        },
    }
}

pub struct Clay {
    // Memory used internally by clay
    #[cfg(feature = "std")]
    _memory: Vec<u8>,
    // Memory used internally by clay. The caller is responsible for managing this memory in
    // no_std case.
    #[cfg(not(feature = "std"))]
    _memory: *const core::ffi::c_void,
}

impl Clay {
    #[cfg(feature = "std")]
    pub fn new(dimensions: Dimensions) -> Self {
        let memory_size = unsafe { Clay_MinMemorySize() };
        let memory = vec![0; memory_size as usize];
        unsafe {
            let arena =
                Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory.as_ptr() as _);
            Clay_Initialize(arena, dimensions.into());
        }

        Self { _memory: memory }
    }

    #[cfg(not(feature = "std"))]
    pub unsafe fn new_with_memory(dimensions: Dimensions, memory: *mut core::ffi::c_void) -> Self {
        let memory_size = Clay_MinMemorySize();
        let arena = Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory);
        Clay_Initialize(arena, dimensions.into());

        Self { _memory: memory }
    }

    pub fn required_memory_size() -> usize {
        unsafe { Clay_MinMemorySize() as usize }
    }

    pub fn measure_text_function(&self, func: MeasureTextFunction) {
        unsafe {
            MEASURE_TEXT_HANDLER = Some(func);
            Clay_SetMeasureTextFunction(Some(measure_text_handle));
        }
    }

    pub fn layout_dimensions(&self, dimensions: Dimensions) {
        unsafe {
            Clay_SetLayoutDimensions(dimensions.into());
        }
    }
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

    // TODO: Uncomment once `clay.h` adds the declaration of `Clay_PointerOver`
    // pub fn pointer_over(&self, id: Id) -> bool {
    //     unsafe { Clay_PointerOver(id.into()) }
    // }

    pub fn begin(&self) {
        unsafe { Clay_BeginLayout() };
    }

    pub fn end(&self) -> impl Iterator<Item = RenderCommand> {
        let array = unsafe { Clay_EndLayout() };
        let slice = unsafe { core::slice::from_raw_parts(array.internalArray, array.length as _) };
        slice.iter().map(|command| RenderCommand::from(*command))
    }

    pub fn with<F: FnOnce(&Clay), const N: usize>(&self, configs: [TypedConfig; N], f: F) {
        unsafe { Clay__OpenElement() };

        for config in configs {
            if config.config_type == ElementConfigType::Id as _ {
                unsafe { Clay__AttachId(config.id) };
            } else if config.config_type == ElementConfigType::Layout as _ {
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

    pub fn text(&self, text: &str, config: TextElementConfig) {
        unsafe { Clay__OpenTextElement(text.into(), config.into()) };
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
    use std::mem;

    use color::Color;
    use elements::{
        containers::border::BorderContainer, rectangle::Rectangle, text::Text, CornerRadius,
    };
    use id::Id;
    use layout::{padding::Padding, sizing::Sizing, Layout};

    use super::*;

    /*
    #[test]
    fn test_create() {
        let _clay = Clay::new(800.0, 600.0);
    }
    */

    #[test]
    fn test_begin() {
        let clay = Clay::new(Dimensions::new(800.0, 600.0));

        clay.begin();

        clay.with(
            [
                Layout::new()
                    .width(Sizing::Fixed(100.0))
                    .height(Sizing::Fixed(100.0))
                    .padding(Padding::new(10, 10))
                    .end(),
                Rectangle::new()
                    .color(Color::rgb(255., 255., 255.))
                    .end(Id::new("parent_rect")),
            ],
            |clay| {
                clay.with(
                    [
                        Layout::new()
                            .width(Sizing::Fixed(100.0))
                            .height(Sizing::Fixed(100.0))
                            .padding(Padding::new(10, 10))
                            .end(),
                        Rectangle::new()
                            .color(Color::rgb(255., 255., 255.))
                            .end(Id::new("rect_under_rect")),
                    ],
                    |_clay| {},
                );
                // THIS FAILS
                // clay.text("test", Text::new().color(Color::rgb(255., 255., 255.)).font_size(24).end());
            },
        );
        clay.with(
            [
                Layout::new().padding(Padding::new(16, 16)).end(),
                BorderContainer::new()
                    .all_directions(2, Color::rgb(255., 255., 0.))
                    .corner_radius(CornerRadius::All(25.))
                    .end(Id::new_index("Border_container", 1)),
            ],
            |clay| {
                clay.with(
                    [
                        Layout::new()
                            .width(Sizing::Fixed(50.0))
                            .height(Sizing::Fixed(50.0))
                            .end(),
                        Rectangle::new()
                            .color(Color::rgb(0., 255., 255.))
                            .end(Id::new("rect_under_border")),
                    ],
                    |_clay| {},
                );
            },
        );

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
            mem::size_of::<Clay_SizingAxis__bindgen_ty_1>(),
            mem::size_of::<Clay_SizingAxis__bindgen_ty_1>()
        )
    }
}
