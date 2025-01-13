use crate::{bindings::*, color::Color, mem, TypedConfig};

use super::{CornerRadius, ElementConfigType};

#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    pub color: Color,
    pub corner_radius: CornerRadius,
}

impl Rectangle {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    pub fn corner_radius(&mut self, corner_radius: CornerRadius) -> &mut Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreRectangleElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: mem::zeroed_init(),
            config_type: ElementConfigType::Rectangle as _,
        }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            color: Color::rgba(0., 0., 0., 0.),
            corner_radius: CornerRadius::All(0.),
        }
    }
}

impl From<Clay_RectangleElementConfig> for Rectangle {
    fn from(value: Clay_RectangleElementConfig) -> Self {
        Self {
            color: value.color.into(),
            corner_radius: value.cornerRadius.into(),
        }
    }
}
impl From<Rectangle> for Clay_RectangleElementConfig {
    fn from(value: Rectangle) -> Self {
        Self {
            color: value.color.into(),
            cornerRadius: value.corner_radius.into(),
        }
    }
}
