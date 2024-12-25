use core::{ffi::c_void, ptr};

use crate::{bindings::*, id::Id, TypedConfig};

use super::ElementConfigType;

#[derive(Debug, Copy, Clone)]
pub struct Custom {
    pub data: *mut c_void,
}

impl Custom {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn data<Data>(&mut self, data: &mut Data) -> &mut Self {
        self.data = data as *mut _ as *mut c_void;
        self
    }

    pub fn end(&self, id: Id) -> TypedConfig {
        let memory = unsafe { Clay__StoreCustomElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: id.into(),
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
