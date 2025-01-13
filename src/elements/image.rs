use core::ffi::c_void;

use crate::{bindings::*, math::Dimensions, mem, DataRef, TypedConfig};

use super::ElementConfigType;

#[derive(Debug, Copy, Clone)]
pub struct Image {
    pub data: *mut c_void,
    pub source_dimensions: Dimensions,
}

impl Image {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the data of the image. The data has to be created by using [Clay::data].
    pub fn data(&mut self, data: DataRef) -> &mut Self {
        self.data = data.ptr as *mut c_void;
        self
    }

    pub fn source_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.source_dimensions = dimensions;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreImageElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: mem::zeroed_init(),
            config_type: ElementConfigType::Image as _,
        }
    }
}

impl Default for Image {
    fn default() -> Self {
        Self {
            data: core::ptr::null_mut(),
            source_dimensions: Dimensions::default(),
        }
    }
}

impl From<Clay_ImageElementConfig> for Image {
    fn from(value: Clay_ImageElementConfig) -> Self {
        Self {
            data: value.imageData,
            source_dimensions: value.sourceDimensions.into(),
        }
    }
}

impl From<Image> for Clay_ImageElementConfig {
    fn from(value: Image) -> Self {
        Self {
            imageData: value.data,
            sourceDimensions: value.source_dimensions.into(),
        }
    }
}
