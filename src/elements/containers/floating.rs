use crate::{
    bindings::*,
    elements::ElementConfigType,
    id::Id,
    math::{Dimensions, Vector2},
    TypedConfig,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FloatingAttachPointType {
    LeftTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_TOP,
    LeftCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_CENTER,
    LeftBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_BOTTOM,
    CenterTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_TOP,
    CenterCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_CENTER,
    CenterBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_BOTTOM,
    RightTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_TOP,
    RightCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_CENTER,
    RightBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_BOTTOM,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum PointerCaptureMode {
    Capture = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE,
    Passthrough = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH,
}

#[derive(Debug, Copy, Clone)]
pub struct FloatingContainer {
    pub offset: Vector2,
    pub expand: Dimensions,
    pub z_index: u16,
    pub parent: u32,
    pub parent_attachment: FloatingAttachPointType,
    pub element_attachment: FloatingAttachPointType,
    pub pointer_capture_mode: PointerCaptureMode,
}

impl FloatingContainer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn offset(&mut self, offset: Vector2) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.expand = dimensions;
        self
    }

    pub fn z_index(&mut self, z_index: u16) -> &mut Self {
        self.z_index = z_index;
        self
    }

    pub fn parent_id(&mut self, id: u32) -> &mut Self {
        self.parent = id;
        self
    }

    pub fn attachment(
        &mut self,
        element: FloatingAttachPointType,
        parent: FloatingAttachPointType,
    ) -> &mut Self {
        self.element_attachment = element;
        self.parent_attachment = parent;
        self
    }

    pub fn pointer_capture_mode(&mut self, mode: PointerCaptureMode) -> &mut Self {
        self.pointer_capture_mode = mode;
        self
    }

    pub fn end(&self, id: Id) -> TypedConfig {
        let memory = unsafe { Clay__StoreFloatingElementConfig((*self).into()) };

        TypedConfig {
            config_memory: memory as _,
            id: id.into(),
            config_type: ElementConfigType::FloatingContainer as _,
        }
    }
}

impl Default for FloatingContainer {
    fn default() -> Self {
        Self {
            offset: Vector2::default(),
            expand: Dimensions::default(),
            z_index: 0,
            parent: 0,
            parent_attachment: FloatingAttachPointType::LeftTop,
            element_attachment: FloatingAttachPointType::LeftTop,
            pointer_capture_mode: PointerCaptureMode::Capture,
        }
    }
}

impl From<Clay_FloatingElementConfig> for FloatingContainer {
    fn from(value: Clay_FloatingElementConfig) -> Self {
        Self {
            offset: value.offset.into(),
            expand: value.expand.into(),
            z_index: value.zIndex,
            parent: value.parentId,
            element_attachment: unsafe {
                core::mem::transmute::<u8, FloatingAttachPointType>(value.attachment.element)
            },
            parent_attachment: unsafe {
                core::mem::transmute::<u8, FloatingAttachPointType>(value.attachment.parent)
            },
            pointer_capture_mode: unsafe {
                core::mem::transmute::<u32, PointerCaptureMode>(value.pointerCaptureMode)
            },
        }
    }
}
impl From<FloatingContainer> for Clay_FloatingElementConfig {
    fn from(value: FloatingContainer) -> Self {
        Self {
            offset: value.offset.into(),
            expand: value.expand.into(),
            zIndex: value.z_index,
            parentId: value.parent,
            attachment: Clay_FloatingAttachPoints {
                element: value.element_attachment as _,
                parent: value.parent_attachment as _,
            },
            pointerCaptureMode: value.pointer_capture_mode as _,
        }
    }
}
