use crate::{
    bindings::*,
    color::Color,
    math::{BoundingBox, Dimensions},
};

/// Represents a rectangle with a specified color and corner radii.
#[derive(Debug, Clone)]
pub struct Rectangle {
    /// The fill color of the rectangle.
    pub color: Color,
    /// The corner radii for rounded edges.
    pub corner_radii: CornerRadii,
}

/// Represents a text element with styling attributes.
#[derive(Debug, Clone)]
pub struct Text<'a> {
    /// The text content.
    pub text: &'a str,
    /// The color of the text.
    pub color: Color,
    /// The ID of the font used.
    pub font_id: u16,
    /// The font size.
    pub font_size: u16,
    /// The spacing between letters.
    pub letter_spacing: u16,
    /// The line height.
    pub line_height: u16,
}

/// Defines individual corner radii for an element.
#[derive(Debug, Clone)]
pub struct CornerRadii {
    /// The radius for the top-left corner.
    pub top_left: f32,
    /// The radius for the top-right corner.
    pub top_right: f32,
    /// The radius for the bottom-left corner.
    pub bottom_left: f32,
    /// The radius for the bottom-right corner.
    pub bottom_right: f32,
}

/// Defines the border width for each side of an element.
#[derive(Debug, Clone)]
pub struct BorderWidth {
    /// Border width on the left side.
    pub left: u16,
    /// Border width on the right side.
    pub right: u16,
    /// Border width on the top side.
    pub top: u16,
    /// Border width on the bottom side.
    pub bottom: u16,
    /// Border width between child elements.
    pub between_children: u16,
}

/// Represents a border with a specified color, width, and corner radii.
#[derive(Debug, Clone)]
pub struct Border {
    /// The border color.
    pub color: Color,
    /// The corner radii for rounded border edges.
    pub corner_radii: CornerRadii,
    /// The width of the border on each side.
    pub width: BorderWidth,
}

/// Represents an image with defined dimensions and data.
#[derive(Debug, Clone)]
pub struct Image<'a, ImageElementData> {
    /// Background color
    pub background_color: Color,
    /// The dimensions of the image.
    pub dimensions: Dimensions,
    /// A pointer to the image data.
    pub data: &'a ImageElementData,
}

/// Represents a custom element with a background color, corner radii, and associated data.
#[derive(Debug, Clone)]
pub struct Custom<'a, CustomElementData> {
    /// The background color of the custom element.
    pub background_color: Color,
    /// The corner radii for rounded edges.
    pub corner_radii: CornerRadii,
    /// A pointer to additional custom data.
    pub data: &'a CustomElementData,
}

impl From<Clay_RectangleRenderData> for Rectangle {
    fn from(value: Clay_RectangleRenderData) -> Self {
        Self {
            color: value.backgroundColor.into(),
            corner_radii: value.cornerRadius.into(),
        }
    }
}

impl From<Clay_TextRenderData> for Text<'_> {
    fn from(value: Clay_TextRenderData) -> Self {
        let text = unsafe {
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                value.stringContents.chars as *const u8,
                value.stringContents.length as _,
            ))
        };

        Self {
            text,
            color: value.textColor.into(),
            font_id: value.fontId,
            font_size: value.fontSize,
            letter_spacing: value.letterSpacing,
            line_height: value.lineHeight,
        }
    }
}

impl<ImageElementData> Image<'_, ImageElementData> {
    pub(crate) unsafe fn from_clay_image_render_data(value: Clay_ImageRenderData) -> Self {
        Self {
            dimensions: value.sourceDimensions.into(),
            data: unsafe { &*value.imageData.cast() },
            background_color: value.backgroundColor.into(),
        }
    }
}

impl From<Clay_CornerRadius> for CornerRadii {
    fn from(value: Clay_CornerRadius) -> Self {
        Self {
            top_left: value.topLeft,
            top_right: value.topRight,
            bottom_left: value.bottomLeft,
            bottom_right: value.bottomRight,
        }
    }
}

impl From<Clay_BorderRenderData> for Border {
    fn from(value: Clay_BorderRenderData) -> Self {
        Self {
            color: value.color.into(),
            corner_radii: value.cornerRadius.into(),

            width: BorderWidth {
                left: value.width.left,
                right: value.width.right,
                top: value.width.top,
                bottom: value.width.bottom,
                between_children: value.width.betweenChildren,
            },
        }
    }
}

impl<CustomElementData> Custom<'_, CustomElementData> {
    pub(crate) unsafe fn from_clay_custom_element_data(value: Clay_CustomRenderData) -> Self {
        Self {
            background_color: value.backgroundColor.into(),
            corner_radii: value.cornerRadius.into(),
            data: unsafe { &*value.customData.cast() },
        }
    }
}

#[derive(Debug, Clone)]
pub enum RenderCommandConfig<'a, ImageElementData, CustomElementData> {
    None(),
    Rectangle(Rectangle),
    Border(Border),
    Text(Text<'a>),
    Image(Image<'a, ImageElementData>),
    ScissorStart(),
    ScissorEnd(),
    Custom(Custom<'a, CustomElementData>),
}

impl<ImageElementData, CustomElementData>
    RenderCommandConfig<'_, ImageElementData, CustomElementData>
{
    #[allow(non_upper_case_globals)]
    pub(crate) unsafe fn from_clay_render_command(value: &Clay_RenderCommand) -> Self {
        match value.commandType {
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE => Self::None(),
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE => {
                Self::Rectangle(Rectangle::from(*unsafe { &value.renderData.rectangle }))
            }
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT => {
                Self::Text(Text::from(*unsafe { &value.renderData.text }))
            }
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER => {
                Self::Border(Border::from(*unsafe { &value.renderData.border }))
            }
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE => {
                Self::Image(unsafe { Image::from_clay_image_render_data(value.renderData.image) })
            }
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START => Self::ScissorStart(),
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END => Self::ScissorEnd(),
            Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM => Self::Custom(unsafe {
                Custom::from_clay_custom_element_data(value.renderData.custom)
            }),
            _ => unreachable!(),
        }
    }
}

/// Represents a render command for drawing an element on the screen.
#[derive(Debug, Clone)]
pub struct RenderCommand<'a, ImageElementData, CustomElementData> {
    /// The bounding box defining the area occupied by the element.
    pub bounding_box: BoundingBox,
    /// The specific configuration for rendering this command.
    pub config: RenderCommandConfig<'a, ImageElementData, CustomElementData>,
    /// A unique identifier for the render command.
    pub id: u32,
    /// The z-index determines the stacking order of elements.
    /// Higher values are drawn above lower values.
    pub z_index: i16,
}

impl<ImageElementData, CustomElementData> RenderCommand<'_, ImageElementData, CustomElementData> {
    pub(crate) unsafe fn from_clay_render_command(value: Clay_RenderCommand) -> Self {
        Self {
            id: value.id,
            z_index: value.zIndex,
            bounding_box: value.boundingBox.into(),
            config: unsafe { RenderCommandConfig::from_clay_render_command(&value) },
        }
    }
}
