use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 255.0 }
    }
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Allows using hex values to build colors
    pub fn u_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgb(r as _, g as _, b as _)
    }
    /// Allows using hex values to build colors
    pub fn u_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::rgba(r as _, g as _, b as _, a as _)
    }
}

impl From<Clay_Color> for Color {
    fn from(value: Clay_Color) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<Color> for Clay_Color {
    fn from(value: Color) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
