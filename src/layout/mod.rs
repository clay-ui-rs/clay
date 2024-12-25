use crate::{bindings::*, elements::ElementConfigType, mem::zeroed_init, TypedConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutDirection {
    LeftToRight = Clay_LayoutDirection_CLAY_LEFT_TO_RIGHT,
    TopToBottom = Clay_LayoutDirection_CLAY_TOP_TO_BOTTOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutAlignmentX {
    Left = Clay_LayoutAlignmentX_CLAY_ALIGN_X_LEFT,
    Center = Clay_LayoutAlignmentX_CLAY_ALIGN_X_CENTER,
    Right = Clay_LayoutAlignmentX_CLAY_ALIGN_X_RIGHT,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum LayoutAlignmentY {
    Top = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_TOP,
    Center = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_CENTER,
    Bottom = Clay_LayoutAlignmentY_CLAY_ALIGN_Y_BOTTOM,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum SizingType {
    Fit = Clay__SizingType_CLAY__SIZING_TYPE_FIT,
    Grow = Clay__SizingType_CLAY__SIZING_TYPE_GROW,
    Percent = Clay__SizingType_CLAY__SIZING_TYPE_PERCENT,
    Fixed = Clay__SizingType_CLAY__SIZING_TYPE_FIXED,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Sizing {
    Fit(f32, f32),
    Grow(f32, f32),
    Fixed(f32),
    Percent(f32),
}

impl From<Sizing> for Clay_SizingAxis {
    fn from(value: Sizing) -> Self {
        match value {
            Sizing::Fit(min, max) => Self {
                type_: SizingType::Fit as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizeMinMax: Clay_SizingMinMax { min, max },
                },
            },

            Sizing::Grow(min, max) => Self {
                type_: SizingType::Grow as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizeMinMax: Clay_SizingMinMax { min, max },
                },
            },

            Sizing::Fixed(size) => Self {
                type_: SizingType::Fixed as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizeMinMax: Clay_SizingMinMax {
                        min: size,
                        max: size,
                    },
                },
            },

            Sizing::Percent(percent) => Self {
                type_: SizingType::Percent as _,
                __bindgen_anon_1: Clay_SizingAxis__bindgen_ty_1 {
                    sizePercent: percent,
                },
            },
        }
    }
}

pub struct Layout {
    inner: Clay_LayoutConfig,
}

impl Layout {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }

    pub fn sizing_width(&mut self, sizing: Sizing) -> &mut Self {
        self.inner.sizing.width = sizing.into();
        self
    }

    pub fn sizing_height(&mut self, sizing: Sizing) -> &mut Self {
        self.inner.sizing.height = sizing.into();
        self
    }

    pub fn padding(&mut self, padding: (u16, u16)) -> &mut Self {
        self.inner.padding.x = padding.0;
        self.inner.padding.y = padding.1;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreLayoutConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: zeroed_init(),
            config_type: ElementConfigType::Layout as _,
        }
    }
}
