use crate::bindings::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Padding {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16,
}

impl Padding {
    pub fn new_rect(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self { left, right, top, bottom }
    }

    pub fn all(value: u16) -> Self {
        Self::new_rect(value, value, value, value)
    }

    pub fn horizontal(value: u16) -> Self {
        Self::new_rect(value, value, 0, 0)
    }

    pub fn vertical(value: u16) -> Self {
        Self::new_rect(0, 0, value, value)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn left(&mut self, value: u16) -> &mut Self {
        self.left = value;
        self
    }

    pub fn right(&mut self, value: u16) -> &mut Self {
        self.right = value;
        self
    }

    pub fn top(&mut self, value: u16) -> &mut Self {
        self.top = value;
        self
    }

    pub fn bottom(&mut self, value: u16) -> &mut Self {
        self.bottom = value;
        self
    }

    pub fn end(&mut self) -> Self {
        *self
    }
}

impl From<Clay_Padding> for Padding {
    fn from(value: Clay_Padding) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}
impl From<Padding> for Clay_Padding {
    fn from(value: Padding) -> Self {
        Self {
            left: value.left,
            right: value.right,
            top: value.top,
            bottom: value.bottom,
        }
    }
}

impl From<(u16, u16, u16, u16)> for Padding {
    fn from(other: (u16, u16, u16, u16)) -> Self {
        Self::new_rect(other.0, other.1, other.2, other.3)
    }
}
