use crate::bindings::*;

pub mod containers;
pub mod custom;
pub mod image;
pub mod rectangle;
pub mod text;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElementConfigType {
    Rectangle = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_RECTANGLE,
    BorderContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_BORDER_CONTAINER,
    FloatingContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_FLOATING_CONTAINER,
    ScrollContainer = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_SCROLL_CONTAINER,
    Image = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_IMAGE,
    Text = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_TEXT,
    Custom = Clay__ElementConfigType_CLAY__ELEMENT_CONFIG_TYPE_CUSTOM,
    // Rust specific enum types (uses the same approach as Odin bindings)
    Id = 65,
    Layout = 66,
}

pub enum CornerRadius {
    All(f32),
    Individual {
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    },
}

impl From<CornerRadius> for Clay_CornerRadius {
    fn from(value: CornerRadius) -> Self {
        match value {
            CornerRadius::All(radius) => Self {
                topLeft: radius,
                topRight: radius,
                bottomLeft: radius,
                bottomRight: radius,
            },
            CornerRadius::Individual {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
            } => Self {
                topLeft: top_left,
                topRight: top_right,
                bottomLeft: bottom_left,
                bottomRight: bottom_right,
            },
        }
    }
}
impl From<Clay_CornerRadius> for CornerRadius {
    fn from(value: Clay_CornerRadius) -> Self {
        if value.topLeft == value.topRight
            && value.topRight == value.bottomLeft
            && value.bottomLeft == value.bottomRight
        {
            Self::All(value.topLeft)
        } else {
            Self::Individual {
                top_left: value.topLeft,
                top_right: value.topRight,
                bottom_left: value.bottomLeft,
                bottom_right: value.bottomRight,
            }
        }
    }
}
