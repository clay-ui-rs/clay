use crate::{bindings::*, elements::ElementConfigType, id::Id, TypedConfig};

#[derive(Debug, Copy, Clone, Default)]
pub struct ScrollContainer {
    pub horizontal: bool,
    pub vertical: bool,
}

impl ScrollContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn horizontal(&mut self) -> &mut Self {
        self.horizontal = true;
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        self.vertical = true;
        self
    }

    pub fn end(&self, id: Id) -> TypedConfig {
        let memory = unsafe { Clay__StoreScrollElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: id.into(),
            config_type: ElementConfigType::ScrollContainer as _,
        }
    }
}

impl From<Clay_ScrollElementConfig> for ScrollContainer {
    fn from(value: Clay_ScrollElementConfig) -> Self {
        Self {
            horizontal: value.horizontal,
            vertical: value.vertical,
        }
    }
}
impl From<ScrollContainer> for Clay_ScrollElementConfig {
    fn from(value: ScrollContainer) -> Self {
        Self {
            horizontal: value.horizontal,
            vertical: value.vertical,
        }
    }
}
