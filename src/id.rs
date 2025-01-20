use crate::{bindings::*, ElementConfigType, TypedConfig};

pub struct Id;

impl Id {
    /// Creates a clay id using the `label`
    #[allow(clippy::new_ret_no_self)]
    pub fn new(label: &str) -> TypedConfig {
        Self::new_index(label, 0)
    }

    /// Creates a clay id using the `label` and the `index`
    pub fn new_index(label: &str, index: u32) -> TypedConfig {
        Self::new_index_internal(label, index)
    }

    fn new_index_internal(label: &str, index: u32) -> TypedConfig {
        let id = unsafe { Clay__HashString(label.into(), index, 0) };
        TypedConfig {
            //
            config_memory: core::ptr::null_mut(),
            id,
            config_type: ElementConfigType::Id as _,
        }
    }
}
