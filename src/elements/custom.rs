use std::{ffi::c_void, marker::PhantomData};

use crate::{bindings::*, id::Id, mem::zeroed_init, TypedConfig};

use super::ElementConfigType;

pub struct Custom<Data> {
    inner: Clay_CustomElementConfig,
    id: Id,
    phantom: PhantomData<Data>,
}

impl<Data> Custom<Data> {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
            id: Id::default(),
            phantom: PhantomData,
        }
    }

    pub fn attach(&mut self, id: Id) -> &mut Self {
        self.id = id;
        self
    }

    pub fn data(&mut self, data: *const Data) -> &mut Self {
        self.inner.customData = data as *mut c_void;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreCustomElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: self.id.into(),
            config_type: ElementConfigType::Image as _,
        }
    }
}
