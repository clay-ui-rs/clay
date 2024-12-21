use crate::bindings::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
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
