use crate::bindings::*;

pub struct Id {
    pub(crate) id: Clay_ElementId,
}

impl Id {
    /// Creates a clay id using the `label`
    #[allow(clippy::new_ret_no_self)]
    pub(crate) fn new(label: &str) -> Id {
        Self::new_index(label, 0)
    }

    /// Creates a clay id using the `label` and the `index`
    pub(crate) fn new_index(label: &str, index: u32) -> Id {
        Self::new_index_internal(label, index)
    }

    pub(crate) fn new_index_internal(label: &str, index: u32) -> Id {
        let id = unsafe { Clay__HashString(label.into(), index, 0) };
        Id { id }
    }

    pub(crate) fn new_index_local(label: &str, index: u32) -> Id {
        let id = unsafe { Clay__HashString(label.into(), index, Clay__GetParentElementId()) };
        Id { id }
    }
}
