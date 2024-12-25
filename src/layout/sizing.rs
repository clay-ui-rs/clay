use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SizingType {
    Fit = Clay__SizingType_CLAY__SIZING_TYPE_FIT,
    Grow = Clay__SizingType_CLAY__SIZING_TYPE_GROW,
    Percent = Clay__SizingType_CLAY__SIZING_TYPE_PERCENT,
    Fixed = Clay__SizingType_CLAY__SIZING_TYPE_FIXED,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizing {
    Fit(f32, f32),
    Grow(f32, f32),
    Fixed(f32),
    Percent(f32),
}

impl From<Clay_SizingAxis> for Sizing {
    fn from(value: Clay_SizingAxis) -> Self {
        match unsafe { core::mem::transmute::<u8, SizingType>(value.type_) } {
            SizingType::Fit => {
                let min_max = unsafe { value.__bindgen_anon_1.sizeMinMax };
                Self::Fit(min_max.min, min_max.max)
            }
            SizingType::Grow => {
                let min_max = unsafe { value.__bindgen_anon_1.sizeMinMax };
                Self::Grow(min_max.min, min_max.max)
            }
            SizingType::Fixed => {
                let min_max = unsafe { value.__bindgen_anon_1.sizeMinMax };
                Self::Fixed(min_max.min)
            }
            SizingType::Percent => {
                let percent = unsafe { value.__bindgen_anon_1.sizePercent };
                Self::Percent(percent)
            }
        }
    }
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
