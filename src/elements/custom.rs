use core::{ffi::c_void, ptr};

use crate::{bindings::*, mem, DataRef, TypedConfig};

use super::ElementConfigType;

#[derive(Debug, Copy, Clone)]
pub struct Custom {
    pub data: *mut c_void,
}

impl Custom {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the data for custom. The data has to be created by using [Clay::data].
    pub fn data(&mut self, data: DataRef) -> &mut Self {
        self.data = data.ptr as *mut c_void;
        self
    }

    pub fn end(&self) -> TypedConfig {
        let memory = unsafe { Clay__StoreCustomElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: mem::zeroed_init(),
            config_type: ElementConfigType::Image as _,
        }
    }
}

impl Default for Custom {
    fn default() -> Self {
        Self {
            data: ptr::null_mut(),
        }
    }
}

impl From<Clay_CustomElementConfig> for Custom {
    fn from(value: Clay_CustomElementConfig) -> Self {
        Self {
            data: value.customData,
        }
    }
}

impl From<Custom> for Clay_CustomElementConfig {
    fn from(value: Custom) -> Self {
        Self {
            customData: value.data,
        }
    }
}
