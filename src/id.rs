use crate::bindings::*;

#[derive(Debug, Copy, Clone)]
pub struct Id {
    pub id: Clay_ElementId,
}

impl Id {
    /// Creates a clay id using the `label`
    #[inline]
    pub(crate) fn new(label: &'static str) -> Id {
        Self::new_index(label, 0)
    }

    /// Creates a clay id using the `label` and the `index`
    #[inline]
    pub(crate) fn new_index(label: &'static str, index: u32) -> Id {
        Self::new_index_internal(label, index)
    }

    #[inline]
    pub(crate) fn new_index_internal(label: &'static str, index: u32) -> Id {
        let id = unsafe { Clay__HashStringWithOffset(label.into(), index, 0) };
        Id { id }
    }

    #[inline]
    pub(crate) fn new_index_local(label: &'static str, index: u32) -> Id {
        let id =
            unsafe { Clay__HashStringWithOffset(label.into(), index, Clay__GetParentElementId()) };
        Id { id }
    }
}
