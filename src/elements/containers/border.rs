use crate::{
    bindings::*,
    color::Color,
    elements::{CornerRadius, ElementConfigType},
    mem, TypedConfig,
};

#[derive(Debug, Clone, Copy)]
pub struct Border {
    width: u32,
    color: Color,
}

impl Border {
    pub fn new(width: u32, color: Color) -> Self {
        Self { width, color }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            width: 0,
            color: Color::rgba(0., 0., 0., 0.),
        }
    }
}

impl From<Clay_Border> for Border {
    fn from(value: Clay_Border) -> Self {
        Self {
            width: value.width,
            color: value.color.into(),
        }
    }
}
impl From<Border> for Clay_Border {
    fn from(value: Border) -> Self {
        Self {
            width: value.width,
            color: value.color.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BorderContainer {
    pub left: Border,
    pub right: Border,
    pub top: Border,
    pub bottom: Border,
    pub between_childs: Border,
    pub corner_radius: CornerRadius,
}

impl BorderContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn left(&mut self, width: u32, color: Color) -> &mut Self {
        self.left = Border::new(width, color);
        self
    }
    pub fn right(&mut self, width: u32, color: Color) -> &mut Self {
        self.right = Border::new(width, color);
        self
    }
    pub fn top(&mut self, width: u32, color: Color) -> &mut Self {
        self.top = Border::new(width, color);
        self
    }
    pub fn bottom(&mut self, width: u32, color: Color) -> &mut Self {
        self.bottom = Border::new(width, color);
        self
    }
    pub fn between_childs(&mut self, width: u32, color: Color) -> &mut Self {
        self.between_childs = Border::new(width, color);
        self
    }
    pub fn all_directions(&mut self, width: u32, color: Color) -> &mut Self {
        self.left(width, color)
            .right(width, color)
            .top(width, color)
            .bottom(width, color)
    }
    pub fn all(&mut self, width: u32, color: Color) -> &mut Self {
        self.all_directions(width, color)
            .between_childs(width, color)
    }

    pub fn corner_radius(&mut self, corner_radius: CornerRadius) -> &mut Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreBorderElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: mem::zeroed_init(),
            config_type: ElementConfigType::BorderContainer as _,
        }
    }
}

impl Default for BorderContainer {
    fn default() -> Self {
        Self {
            left: Border::default(),
            right: Border::default(),
            top: Border::default(),
            bottom: Border::default(),
            between_childs: Border::default(),
            corner_radius: CornerRadius::All(0.),
        }
    }
}

impl From<Clay_BorderElementConfig> for BorderContainer {
    fn from(value: Clay_BorderElementConfig) -> Self {
        Self {
            left: value.left.into(),
            right: value.right.into(),
            top: value.top.into(),
            bottom: value.bottom.into(),
            between_childs: value.betweenChildren.into(),
            corner_radius: value.cornerRadius.into(),
        }
    }
}
impl From<BorderContainer> for Clay_BorderElementConfig {
    fn from(value: BorderContainer) -> Self {
        Self {
            left: value.left.into(),
            right: value.right.into(),
            top: value.top.into(),
            bottom: value.bottom.into(),
            betweenChildren: value.between_childs.into(),
            cornerRadius: value.corner_radius.into(),
        }
    }
}
