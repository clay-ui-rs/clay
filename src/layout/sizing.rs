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
    /// Fit the layout [width](Layout::width)/[height](Layout::height) to a min and max constraint
    Fit(f32, f32),
    /// Grows to fill the parent, while keeping the layout [width](Layout::width)/[height](Layout::height)
    /// between a min and max constraint
    Grow(f32, f32),
    /// Fix the layout [width](Layout::width)/[height](Layout::height) to a certain value
    Fixed(f32),
    /// Fix the layout [width](Layout::width)/[height](Layout::height) to a certain percentage of
    /// it's parent
    Percent(f32),
}

impl From<Clay_SizingAxis> for Sizing {
    fn from(value: Clay_SizingAxis) -> Self {
        match unsafe { core::mem::transmute::<u8, SizingType>(value.type_) } {
            SizingType::Fit => {
                let min_max = unsafe { value.size.minMax };
                Self::Fit(min_max.min, min_max.max)
            }
            SizingType::Grow => {
                let min_max = unsafe { value.size.minMax };
                Self::Grow(min_max.min, min_max.max)
            }
            SizingType::Fixed => {
                let min_max = unsafe { value.size.minMax };
                Self::Fixed(min_max.min)
            }
            SizingType::Percent => {
                let percent = unsafe { value.size.percent };
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
                size: Clay_SizingAxis__bindgen_ty_1 {
                    minMax: Clay_SizingMinMax { min, max },
                },
            },
            Sizing::Grow(min, max) => Self {
                type_: SizingType::Grow as _,
                size: Clay_SizingAxis__bindgen_ty_1 {
                    minMax: Clay_SizingMinMax { min, max },
                },
            },
            Sizing::Fixed(size) => Self {
                type_: SizingType::Fixed as _,
                size: Clay_SizingAxis__bindgen_ty_1 {
                    minMax: Clay_SizingMinMax {
                        min: size,
                        max: size,
                    },
                },
            },
            Sizing::Percent(percent) => Self {
                type_: SizingType::Percent as _,
                size: Clay_SizingAxis__bindgen_ty_1 { percent },
            },
        }
    }
}

/// Shorthand to create a [`Sizing::Fit`] value. Excluding the `$max` value sets it to `f32::MAX`.
#[macro_export]
macro_rules! fit {
    ($min:expr, $max:expr) => {
        $crate::layout::sizing::Sizing::Fit($min, $max)
    };

    ($min:expr) => {
        fit!($min, f32::MAX)
    };

    () => {
        fit!(0.0)
    };
}

/// Shorthand to create a [`Sizing::Grow`] value. Excluding the `$max` value sets it to `f32::MAX`.
#[macro_export]
macro_rules! grow {
    ($min:expr, $max:expr) => {
        $crate::layout::sizing::Sizing::Grow($min, $max)
    };

    ($min:expr) => {
        grow!($min, f32::MAX)
    };

    () => {
        grow!(0.0)
    };
}

/// Shorthand to create a [`Sizing::Fixed`] value.
#[macro_export]
macro_rules! fixed {
    ($val:expr) => {
        $crate::layout::sizing::Sizing::Fixed($val)
    };
}

/// Shorthand to create a [`Sizing::Percent`] value.
#[macro_export]
macro_rules! percent {
    ($percent:expr) => {
        $crate::layout::sizing::Sizing::Percent($percent)
    };
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{fit, fixed, grow, percent};

    #[test]
    fn fit_macro() {
        let both_args = fit!(12.0, 34.0);
        assert!(matches!(both_args, Sizing::Fit(12.0, 34.0)));

        let one_arg = fit!(12.0);
        assert!(matches!(one_arg, Sizing::Fit(12.0, f32::MAX)));

        let zero_args = fit!();
        assert!(matches!(zero_args, Sizing::Fit(0.0, f32::MAX)));
    }

    #[test]
    fn grow_macro() {
        let both_args = grow!(12.0, 34.0);
        assert!(matches!(both_args, Sizing::Grow(12.0, 34.0)));

        let one_arg = grow!(12.0);
        assert!(matches!(one_arg, Sizing::Grow(12.0, f32::MAX)));

        let zero_args = grow!();
        assert!(matches!(zero_args, Sizing::Grow(0.0, f32::MAX)));
    }

    #[test]
    fn fixed_macro() {
        let value = fixed!(123.0);
        assert!(matches!(value, Sizing::Fixed(123.0)));
    }

    #[test]
    fn percent_macro() {
        let value = percent!(0.5);
        assert!(matches!(value, Sizing::Percent(0.5)));
    }
}
