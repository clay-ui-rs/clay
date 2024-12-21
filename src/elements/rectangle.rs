use std::mem::MaybeUninit;

use crate::{bindings::*, TypedConfig};

use super::ElementConfigType;

pub struct Rectangle {
    inner: Clay_RectangleElementConfig,
}

impl Rectangle {
    pub fn new() -> Self {
        let inner = MaybeUninit::<Clay_RectangleElementConfig>::zeroed(); // Creates zero-initialized uninitialized memory
        let inner = unsafe { inner.assume_init() };
        Self { inner }
    }

    fn null_id() -> Clay_ElementId {
        let inner = MaybeUninit::<Clay_ElementId>::zeroed(); // Creates zero-initialized uninitialized memory
        unsafe { inner.assume_init() }
    }

    pub fn color(&mut self, color: (f32, f32, f32, f32)) -> &mut Self {
        self.inner.color = Clay_Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3,
        };
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreRectangleElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: Self::null_id(),
            config_type: ElementConfigType::Rectangle as _,
        }
    }
}