use crate::{
    bindings::*,
    elements::{
        containers::border::BorderContainer, custom::Custom, image::Image, rectangle::Rectangle,
        text::Text,
    },
    math::BoundingBox,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum RenderCommandType {
    None = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_NONE,
    Rectangle = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_RECTANGLE,
    Border = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_BORDER,
    Text = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_TEXT,
    Image = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_IMAGE,
    ScissorStart = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_START,
    ScissorEnd = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_SCISSOR_END,
    Custom = Clay_RenderCommandType_CLAY_RENDER_COMMAND_TYPE_CUSTOM,
}

#[derive(Debug, Clone)]
pub enum RenderCommandConfig<'a> {
    None(),
    Rectangle(Rectangle),
    Border(BorderContainer),
    Text(&'a str, Text),
    Image(Image),
    ScissorStart(),
    ScissorEnd(),
    Custom(Custom),
}

impl From<&Clay_RenderCommand> for RenderCommandConfig<'_> {
    fn from(value: &Clay_RenderCommand) -> Self {
        match unsafe { core::mem::transmute::<u8, RenderCommandType>(value.commandType) } {
            RenderCommandType::None => Self::None(),
            RenderCommandType::Rectangle => Self::Rectangle(Rectangle::from(*unsafe {
                &mut *(value.config.rectangleElementConfig)
            })),
            RenderCommandType::Border => Self::Border(BorderContainer::from(*unsafe {
                &mut *(value.config.borderElementConfig)
            })),
            RenderCommandType::Text => Self::Text(
                <Clay_StringSlice as Into<&str>>::into(value.text),
                Text::from(*unsafe { &mut *(value.config.textElementConfig) }),
            ),
            RenderCommandType::Image => Self::Image(Image::from(*unsafe {
                &mut *(value.config.imageElementConfig)
            })),
            RenderCommandType::ScissorStart => Self::ScissorStart(),
            RenderCommandType::ScissorEnd => Self::ScissorEnd(),
            RenderCommandType::Custom => Self::Custom(Custom::from(*unsafe {
                &mut *(value.config.customElementConfig)
            })),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderCommand<'a> {
    pub id: u32,
    pub z_index: i32,
    pub bounding_box: BoundingBox,
    pub config: RenderCommandConfig<'a>,
}

impl From<Clay_RenderCommand> for RenderCommand<'_> {
    fn from(value: Clay_RenderCommand) -> Self {
        Self {
            id: value.id,
            z_index: value.zIndex,
            bounding_box: value.boundingBox.into(),
            config: (&value).into(),
        }
    }
}
