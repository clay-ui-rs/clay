use crate::{
    bindings::*,
    color::Color,
    elements::{CornerRadius, ElementConfigType},
    id::Id,
    mem::zeroed_init,
    TypedConfig,
};

pub struct BorderContainer {
    inner: Clay_BorderElementConfig,
    id: Id,
}

impl BorderContainer {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
            id: Id::default(),
        }
    }

    pub fn attach(&mut self, id: Id) -> &mut Self {
        self.id = id;
        self
    }

    fn into_clay_border(width: u32, color: Color) -> Clay_Border {
        Clay_Border {
            width,
            color: color.into(),
        }
    }

    pub fn left(&mut self, width: u32, color: Color) -> &mut Self {
        self.inner.left = Self::into_clay_border(width, color);
        self
    }
    pub fn right(&mut self, width: u32, color: Color) -> &mut Self {
        self.inner.right = Self::into_clay_border(width, color);
        self
    }
    pub fn top(&mut self, width: u32, color: Color) -> &mut Self {
        self.inner.top = Self::into_clay_border(width, color);
        self
    }
    pub fn bottom(&mut self, width: u32, color: Color) -> &mut Self {
        self.inner.bottom = Self::into_clay_border(width, color);
        self
    }
    pub fn between_childs(&mut self, width: u32, color: Color) -> &mut Self {
        self.inner.betweenChildren = Self::into_clay_border(width, color);
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

    pub fn corner_radius(&mut self, radius: CornerRadius) -> &mut Self {
        self.inner.cornerRadius = radius.into();
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreBorderElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: self.id.into(),
            config_type: ElementConfigType::BorderContainer as _,
        }
    }
}
