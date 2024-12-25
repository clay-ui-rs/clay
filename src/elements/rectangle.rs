use crate::{bindings::*, color::Color, id::Id, mem::zeroed_init, TypedConfig};

use super::{CornerRadius, ElementConfigType};

pub struct Rectangle {
    inner: Clay_RectangleElementConfig,
    id: Id,
}

impl Rectangle {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
            id: Id::default(),
        }
    }

    pub fn attach(&mut self, id: Id) -> &mut Self {
        self.id = id;
        self
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.inner.color = color.into();
        self
    }

    pub fn corner_radius(&mut self, radius: CornerRadius) -> &mut Self {
        self.inner.cornerRadius = radius.into();
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreRectangleElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: self.id.into(),
            config_type: ElementConfigType::Rectangle as _,
        }
    }
}
