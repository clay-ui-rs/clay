use core::{ffi::c_void, marker::PhantomData};

use crate::{bindings::*, id::Id, math::Dimensions, mem::zeroed_init, TypedConfig};

use super::ElementConfigType;

pub struct Image<Data> {
    inner: Clay_ImageElementConfig,
    id: Id,
    phantom: PhantomData<Data>,
}

impl<Data> Image<Data> {
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

    pub fn image_data(&mut self, data: *const Data) -> &mut Self {
        self.inner.imageData = data as *mut c_void;
        self
    }

    pub fn source_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.inner.sourceDimensions = dimensions.into();
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreImageElementConfig(self.inner) };

        TypedConfig {
            config_memory: memory as _,
            id: self.id.into(),
            config_type: ElementConfigType::Image as _,
        }
    }
}
