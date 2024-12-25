use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutAlignmentX {
    Left = Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
    Center = Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER,
    Right = Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutAlignmentY {
    Top = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP,
    Center = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER,
    Bottom = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
}

#[derive(Debug, Copy, Clone)]
pub struct Alignment {
    pub x: LayoutAlignmentX,
    pub y: LayoutAlignmentY,
}

impl Alignment {
    pub fn new(x: LayoutAlignmentX, y: LayoutAlignmentY) -> Self {
        Self { x, y }
    }
}

impl From<Clay_ChildAlignment> for Alignment {
    fn from(value: Clay_ChildAlignment) -> Self {
        Self {
            x: unsafe { core::mem::transmute::<u8, LayoutAlignmentX>(value.x) },
            y: unsafe { core::mem::transmute::<u8, LayoutAlignmentY>(value.y) },
        }
    }
}
impl From<Alignment> for Clay_ChildAlignment {
    fn from(value: Alignment) -> Self {
        Self {
            x: value.x as _,
            y: value.y as _,
        }
    }
}
