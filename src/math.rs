use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl From<Clay_Vector2> for Vector2 {
    fn from(value: Clay_Vector2) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<Vector2> for Clay_Vector2 {
    fn from(value: Vector2) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Dimensions {
    width: f32,
    height: f32,
}

impl From<Clay_Dimensions> for Dimensions {
    fn from(value: Clay_Dimensions) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<Dimensions> for Clay_Dimensions {
    fn from(value: Dimensions) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
