use crate::bindings::*;

/// Represents padding with values for left, right, top, and bottom.
#[derive(Debug, Copy, Clone, Default)]
pub struct Padding {
    /// Padding value for the left side.
    pub left: u16,
    /// Padding value for the right side.
    pub right: u16,
    /// Padding value for the top side.
    pub top: u16,
    /// Padding value for the bottom side.
    pub bottom: u16,
}

impl Padding {
    /// Creates a new `Padding` instance with specific values for all sides.
    pub fn new_rect(left: u16, right: u16, top: u16, bottom: u16) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }

    /// Creates a new `Padding` instance with the same value for all sides.
    pub fn all(value: u16) -> Self {
        Self::new_rect(value, value, value, value)
    }

    /// Creates a new `Padding` instance with the same value for left and right sides.
    pub fn horizontal(value: u16) -> Self {
        Self::new_rect(value, value, 0, 0)
    }

    /// Creates a new `Padding` instance with the same value for top and bottom sides.
    pub fn vertical(value: u16) -> Self {
        Self::new_rect(0, 0, value, value)
    }

    /// Creates a new `Padding` instance with default values (all zero).
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the padding value for the left side.
    pub fn left(&mut self, value: u16) -> &mut Self {
        self.left = value;
        self
    }

    /// Sets the padding value for the right side.
    pub fn right(&mut self, value: u16) -> &mut Self {
        self.right = value;
        self
    }

    /// Sets the padding value for the top side.
    pub fn top(&mut self, value: u16) -> &mut Self {
        self.top = value;
        self
    }

    /// Sets the padding value for the bottom side.
    pub fn bottom(&mut self, value: u16) -> &mut Self {
        self.bottom = value;
        self
    }

    /// Finalizes the modifications and returns the updated `Padding` instance.
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
