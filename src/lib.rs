#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
pub mod bindings;

pub mod layout;
pub mod elements;
pub mod commands;

use elements::ElementConfigType;

use crate::bindings::*;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TypedConfig {
    pub config_memory: *const u8,
    pub id: Clay_ElementId,
    pub config_type: ElementConfigType,
}

#[derive(Debug, Clone)]
pub struct Clay {
    // memory owned by clay, destroyed at exit
    mem_raw: *const u8,
}

impl Clay {
    pub fn new(width: f32, height: f32) -> Self {
        let memory_size = unsafe { Clay_MinMemorySize() };
        let memory = vec![0; memory_size as usize];
        let mem_raw = Box::into_raw(memory.into_boxed_slice()) as *const u8;
        unsafe {
            let arena = Clay_CreateArenaWithCapacityAndMemory(memory_size as _, mem_raw as _);
            Clay_Initialize(arena, Clay_Dimensions { width, height });
        }

        Self { mem_raw }
    }

    pub fn begin(&self) {
        unsafe { Clay_BeginLayout() };
    }

    pub fn end(&self) -> Clay_RenderCommandArray {
        unsafe { Clay_EndLayout() }
    }

    pub fn with<F: FnOnce(), const N: usize>(&self, configs: [TypedConfig; N], f: F) {
        unsafe { Clay__OpenElement() };

        for config in configs {
            if config.config_type == ElementConfigType::Id as _ {
                unsafe { Clay__AttachId(config.id) };
            } else if config.config_type == ElementConfigType::Layout as _ {
                unsafe { Clay__AttachLayoutConfig(config.config_memory as _) };
            } else {
                unsafe {
                    Clay__AttachElementConfig(
                        // This isn't strictcly correct, but as this is a union of pointers
                        // we can cast to any of them.
                        Clay_ElementConfigUnion {
                            rectangleElementConfig: config.config_memory as _,
                        },
                        config.config_type as _,
                    )
                };
            }
        }

        unsafe { Clay__ElementPostConfiguration() };

        f();

        unsafe { Clay__CloseElement(); }
    }
}

impl Drop for Clay {
    fn drop(&mut self) {
        unsafe {
            // drops the memory
            let _memory = Box::from_raw(self.mem_raw as *mut u8);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

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
                elements::rectangle::Rectangle::new().color((255.0, 255.0, 255.0, 0.0)).end(),
            ],
            || {

                // do nothing
            },
        );

        // TODO: Cleanup
        let render_array = clay.end();
        let items = unsafe {
            std::slice::from_raw_parts(render_array.internalArray, render_array.length as _)
        };

        for item in items {
            if item.commandType == commands::RenderCommandType::Rectangle as _ {
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
