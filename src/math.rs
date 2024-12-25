use crate::bindings::*;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct Dimensions {
    width: f32,
    height: f32,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[repr(C)]
pub struct BoundingBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl BoundingBox {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<Clay_BoundingBox> for BoundingBox {
    fn from(value: Clay_BoundingBox) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
impl From<BoundingBox> for Clay_BoundingBox {
    fn from(value: BoundingBox) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}
