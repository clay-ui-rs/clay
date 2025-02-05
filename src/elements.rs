use crate::{Declaration, bindings::*, color::Color, DataRef, Vector2, Dimensions};
use core::ffi::c_void;

/// Builder for configuring border properties of a `Declaration`.
pub struct BorderBuilder<'a> {
    parent: &'a mut Declaration,
}

impl BorderBuilder<'_> {
    /// Creates a new `BorderBuilder` with the given parent `Declaration`.
    pub fn new(parent: &mut Declaration) -> BorderBuilder {
        BorderBuilder { parent }
    }

    /// Sets the left border width.
    pub fn left(&mut self, width: u16) -> &mut Self {
        self.parent.inner.border.width.left = width;
        self
    }

    /// Sets the right border width.
    pub fn right(&mut self, width: u16) -> &mut Self {
        self.parent.inner.border.width.right = width;
        self
    }

    /// Sets the top border width.
    pub fn top(&mut self, width: u16) -> &mut Self {
        self.parent.inner.border.width.top = width;
        self
    }

    /// Sets the bottom border width.
    pub fn bottom(&mut self, width: u16) -> &mut Self {
        self.parent.inner.border.width.bottom = width;
        self
    }

    /// Sets the spacing between child elements.
    pub fn between_children(&mut self, width: u16) -> &mut Self {
        self.parent.inner.border.width.betweenChildren = width;
        self
    }

    /// Sets the border color.
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.parent.inner.border.color = color.into();
        self
    }

    /// Returns the modified `Declaration`.
    pub fn end(&mut self) -> &mut Declaration {
        self.parent
    }
}

/// Builder for configuring image properties in a `Declaration`.
pub struct ImageBuilder<'a> {
    parent: &'a mut Declaration,
}

impl ImageBuilder<'_> {
    /// Creates a new `ImageBuilder` with the given parent `Declaration`.
    pub fn new(parent: &mut Declaration) -> ImageBuilder {
        ImageBuilder { parent }
    }

    /// Sets the source directory of the image.
    pub fn source_dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.parent.inner.image.sourceDimensions = dimensions.into();
        self
    }

    /// Sets the image data.
    /// The data must be created using [`Clay::data`].
    pub fn data(&mut self, data: DataRef) -> &mut Self {
        self.parent.inner.image.imageData = data.ptr as *mut c_void;
        self
    }

    /// Sets the image data using a raw pointer.
    /// 
    /// # Safety
    /// This function is unsafe because it accepts a raw pointer.
    pub unsafe fn data_ptr(&mut self, data: *const c_void) -> &mut Self {
        self.parent.inner.image.imageData = data as _;
        self
    }

    /// Returns the modified `Declaration`.
    pub fn end(&mut self) -> &mut Declaration {
        self.parent
    }
}

/// Represents different attachment points for floating elements.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum FloatingAttachPointType {
    /// Attaches to the top-left of the parent.
    LeftTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_TOP,
    /// Attaches to the center-left of the parent.
    LeftCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_CENTER,
    /// Attaches to the bottom-left of the parent.
    LeftBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_LEFT_BOTTOM,
    /// Attaches to the top-center of the parent.
    CenterTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_TOP,
    /// Attaches to the center of the parent.
    CenterCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_CENTER,
    /// Attaches to the bottom-center of the parent.
    CenterBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_CENTER_BOTTOM,
    /// Attaches to the top-right of the parent.
    RightTop = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_TOP,
    /// Attaches to the center-right of the parent.
    RightCenter = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_CENTER,
    /// Attaches to the bottom-right of the parent.
    RightBottom = Clay_FloatingAttachPointType_CLAY_ATTACH_POINT_RIGHT_BOTTOM,
}

/// Specifies how pointer capture should behave for floating elements.
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum PointerCaptureMode {
    /// Captures all pointer input.
    Capture = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_CAPTURE,
    /// Allows pointer input to pass through.
    Passthrough = Clay_PointerCaptureMode_CLAY_POINTER_CAPTURE_MODE_PASSTHROUGH,
}

/// Defines how a floating element is attached to other elements.
#[derive(Debug, Clone)]
#[repr(u8)]
pub enum FloatingAttachToElement {
    /// The floating element is not attached to any other element.
    None = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_NONE,
    /// The floating element is attached to its parent element.
    Parent = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_PARENT,
    /// The floating element is attached to a specific element identified by an ID.
    ElementWithId = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ELEMENT_WITH_ID,
    /// The floating element is attached to the root of the layout.
    Root = Clay_FloatingAttachToElement_CLAY_ATTACH_TO_ROOT,
}

/// Builder for configuring floating element properties in a `Declaration`.
pub struct FloatingBuilder<'a> {
    parent: &'a mut Declaration,
}

impl FloatingBuilder<'_> {
    /// Creates a new `FloatingBuilder` with the given parent `Declaration`.
    pub fn new(parent: &mut Declaration) -> FloatingBuilder {
        FloatingBuilder { parent }
    }

    /// Sets the floating element's offset.
    pub fn offset(&mut self, offset: Vector2) -> &mut Self {
        self.parent.inner.floating.offset = offset.into();
        self
    }

    /// Sets the floating element's dimensions.
    pub fn dimensions(&mut self, dimensions: Dimensions) -> &mut Self {
        self.parent.inner.floating.expand = dimensions.into();
        self
    }

    /// Sets the floating element's Z-index.
    pub fn z_index(&mut self, z_index: i16) -> &mut Self {
        self.parent.inner.floating.zIndex = z_index;
        self
    }

    /// Sets the parent element ID.
    pub fn parent_id(&mut self, id: u32) -> &mut Self {
        self.parent.inner.floating.parentId = id;
        self
    }

    /// Sets the attachment points of the floating element and its parent.
    pub fn attach_points(
        &mut self,
        element: FloatingAttachPointType,
        parent: FloatingAttachPointType,
    ) -> &mut Self {
        self.parent.inner.floating.attachPoints.element = element as _;
        self.parent.inner.floating.attachPoints.parent = parent as _;
        self
    }

    /// Sets how the floating element is attached to other elements.
    ///
    /// - [`FloatingAttachToElement::None`] - The element is not attached to anything.
    /// - [`FloatingAttachToElement::Parent`] - The element is attached to its parent.
    /// - [`FloatingAttachToElement::ElementWithId`] - The element is attached to a specific element by ID.
    /// - [`FloatingAttachToElement::Root`] - The element is attached to the root of the layout.
    pub fn attach_to(&mut self, attach: FloatingAttachToElement) -> &mut Self {
        self.parent.inner.floating.attachTo = attach as _;
        self
    }

    /// Sets the pointer capture mode.
    pub fn pointer_capture_mode(&mut self, mode: PointerCaptureMode) -> &mut Self {
        self.parent.inner.floating.pointerCaptureMode = mode as _;
        self
    }

    /// Returns the modified `Declaration`.
    pub fn end(&mut self) -> &mut Declaration {
        self.parent
    }
}

/// Builder for configuring corner radius properties in a `Declaration`.
pub struct CornerRadiusBuilder<'a> {
    parent: &'a mut Declaration,
}

impl CornerRadiusBuilder<'_> {
    /// Creates a new `CornerRadiusBuilder` with the given parent `Declaration`.
    pub fn new(parent: &mut Declaration) -> CornerRadiusBuilder {
        CornerRadiusBuilder { parent }
    }

    /// Sets the top-left corner radius.
    pub fn top_left(&mut self, radius: f32) -> &mut Self {
        self.parent.inner.cornerRadius.topLeft = radius;
        self
    }

    /// Sets the top-right corner radius.
    pub fn top_right(&mut self, radius: f32) -> &mut Self {
        self.parent.inner.cornerRadius.topRight = radius;
        self
    }

    /// Sets the bottom-left corner radius.
    pub fn bottom_left(&mut self, radius: f32) -> &mut Self {
        self.parent.inner.cornerRadius.bottomLeft = radius;
        self
    }

    /// Sets the bottom-right corner radius.
    pub fn bottom_right(&mut self, radius: f32) -> &mut Self {
        self.parent.inner.cornerRadius.bottomRight = radius;
        self
    }

    /// Sets all four corner radii to the same value.
    pub fn all(&mut self, radius: f32) -> &mut Self {
        self.parent.inner.cornerRadius.topLeft = radius;
        self.parent.inner.cornerRadius.topRight = radius;
        self.parent.inner.cornerRadius.bottomLeft = radius;
        self.parent.inner.cornerRadius.bottomRight = radius;
        self
    }

    /// Returns the modified `Declaration`.
    pub fn end(&mut self) -> &mut Declaration {
        self.parent
    }
}
