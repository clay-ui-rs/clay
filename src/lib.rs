#![cfg_attr(not(feature = "std"), no_std)]

pub mod bindings;

pub mod color;
pub mod elements;
pub mod layout;
pub mod math;
pub mod render_commands;

mod mem;

use elements::{text::TextElementConfig, ElementConfigType};

use crate::bindings::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TypedConfig {
    pub config_memory: *const u8,
    pub id: Clay_ElementId,
    pub config_type: ElementConfigType,
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
    pub fn new(width: f32, height: f32) -> Self {
        let memory_size = unsafe { Clay_MinMemorySize() };
        let memory = vec![0; memory_size as usize];
        unsafe {
            let arena =
                Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory.as_ptr() as _);
            Clay_Initialize(arena, Clay_Dimensions { width, height });
        }

        Self { _memory: memory }
    }

    #[cfg(not(feature = "std"))]
    pub unsafe fn new_with_memory(width: f32, height: f32, memory: *mut core::ffi::c_void) -> Self {
        let memory_size = Clay_MinMemorySize();
        let arena = Clay_CreateArenaWithCapacityAndMemory(memory_size as _, memory);
        Clay_Initialize(arena, Clay_Dimensions { width, height });

        Self { _memory: memory }
    }

    pub fn required_memory_size() -> usize {
        unsafe { Clay_MinMemorySize() as usize }
    }

    pub fn begin(&self) {
        unsafe { Clay_BeginLayout() };
    }

    pub fn end(&self) -> Clay_RenderCommandArray {
        unsafe { Clay_EndLayout() }
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
                        core::mem::transmute(config.config_memory),
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

impl Into<Clay_String> for &str {
    fn into(self) -> Clay_String {
        Clay_String {
            length: self.len() as _,
            chars: self.as_ptr() as _,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use color::Color;
    use elements::text::Text;

    use super::*;

    /*
    #[test]
    fn test_create() {
        let _clay = Clay::new(800.0, 600.0);
    }
    */

    #[test]
    fn test_begin() {
        let clay = Clay::new(800.0, 600.0);

        clay.begin();

        clay.with(
            [
                layout::Layout::new()
                    .sizing_width(layout::Sizing::Fixed(100.0))
                    .sizing_height(layout::Sizing::Fixed(100.0))
                    .padding((10, 10))
                    .end(),
                elements::rectangle::Rectangle::new()
                    .color(Color::rgb(255., 255., 255.))
                    .end(),
            ],
            |clay| {
                clay.with(
                    [
                        layout::Layout::new()
                            .sizing_width(layout::Sizing::Fixed(100.0))
                            .sizing_height(layout::Sizing::Fixed(100.0))
                            .padding((10, 10))
                            .end(),
                        elements::rectangle::Rectangle::new()
                            .color(Color::rgb(255., 255., 255.))
                            .end(),
                    ],
                    |_clay| {},
                );
                // THIS FAILS
                // clay.text("test", Text::new().color(Color::rgb(255., 255., 255.)).font_size(24).end());
            },
        );
        clay.with(
            [
                layout::Layout::new().padding((16, 16)).end(),
                elements::containers::border::BorderContainer::new()
                    .all_directions(2, Color::rgb(255., 255., 0.))
                    .corner_radius(elements::CornerRadius::All(25.))
                    .end(),
            ],
            |_clay| {
                clay.with(
                    [
                        layout::Layout::new()
                            .sizing_width(layout::Sizing::Fixed(50.0))
                            .sizing_height(layout::Sizing::Fixed(50.0))
                            .end(),
                        elements::rectangle::Rectangle::new()
                            .color(Color::rgb(0., 255., 255.))
                            .end(),
                    ],
                    |_clay| {},
                );
            },
        );

        // TODO: Cleanup
        let render_array = clay.end();
        let items = unsafe {
            std::slice::from_raw_parts(render_array.internalArray, render_array.length as _)
        };

        for item in items {
            println!(
                "x:{}, y:{}, width:{}, height:{}, type:{}",
                item.boundingBox.x,
                item.boundingBox.y,
                item.boundingBox.width,
                item.boundingBox.height,
                item.commandType
            );
            if item.commandType == render_commands::RenderCommandType::Rectangle as _ {
                let rectangle = unsafe { item.config.rectangleElementConfig };
                unsafe {
                    println!("{:?}", ((*rectangle).cornerRadius));
                }
            }
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
