use alignment::{Alignment, LayoutAlignmentX, LayoutAlignmentY};
use padding::Padding;
use sizing::Sizing;

use crate::{bindings::*, elements::ElementConfigType, mem::zeroed_init, TypedConfig};

pub mod alignment;
pub mod padding;
pub mod sizing;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutDirection {
    LeftToRight = Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT,
    TopToBottom = Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
}

#[derive(Debug, Copy, Clone)]
pub struct Layout {
    pub width: Sizing,
    pub height: Sizing,
    pub padding: Padding,
    pub child_gap: u16,
    pub child_alignment: Alignment,
    pub direction: LayoutDirection,
}

impl Layout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn width(&mut self, width: Sizing) -> &mut Self {
        self.width = width;
        self
    }

    pub fn height(&mut self, height: Sizing) -> &mut Self {
        self.height = height;
        self
    }

    pub fn padding(&mut self, padding: Padding) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreLayoutConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: zeroed_init(),
            config_type: ElementConfigType::Layout as _,
        }
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            width: Sizing::Fit(0., f32::MAX),
            height: Sizing::Fit(0., f32::MAX),
            padding: Padding::default(),
            child_gap: 0,
            child_alignment: Alignment::new(LayoutAlignmentX::Left, LayoutAlignmentY::Top),
            direction: LayoutDirection::LeftToRight,
        }
    }
}

impl From<Clay_LayoutConfig> for Layout {
    fn from(value: Clay_LayoutConfig) -> Self {
        Self {
            width: value.sizing.width.into(),
            height: value.sizing.height.into(),
            padding: value.padding.into(),
            child_gap: value.childGap,
            child_alignment: value.childAlignment.into(),
            direction: unsafe {
                core::mem::transmute::<u8, LayoutDirection>(value.layoutDirection)
            },
        }
    }
}
impl From<Layout> for Clay_LayoutConfig {
    fn from(value: Layout) -> Self {
        Self {
            sizing: Clay_Sizing {
                width: value.width.into(),
                height: value.height.into(),
            },
            padding: value.padding.into(),
            childGap: value.child_gap,
            childAlignment: value.child_alignment.into(),
            layoutDirection: value.direction as _,
        }
    }
}
