use core::{ffi::c_void, marker::PhantomData};

use crate::{bindings::*, math::Dimensions, mem::zeroed_init, TypedConfig};

use super::ElementConfigType;

pub struct Image<Data> {
    inner: Clay_ImageElementConfig,
    phantom: PhantomData<Data>,
}

impl<Data> Image<Data> {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
            phantom: PhantomData,
        }
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
            id: zeroed_init(),
            config_type: ElementConfigType::Image as _,
        }
    }
}
