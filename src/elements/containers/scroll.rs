use crate::{bindings::*, elements::ElementConfigType, id::Id, mem::zeroed_init, TypedConfig};

pub struct ScrollContainer {
    inner: Clay_ScrollElementConfig,
    id: Id,
}

impl ScrollContainer {
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

    pub fn horizontal(&mut self) -> &mut Self {
        self.inner.horizontal = true;
        self
    }

    pub fn vertical(&mut self) -> &mut Self {
        self.inner.vertical = true;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreScrollElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: self.id.into(),
            config_type: ElementConfigType::ScrollContainer as _,
        }
    }
}
