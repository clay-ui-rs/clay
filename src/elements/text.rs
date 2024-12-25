use crate::{bindings::*, color::Color, mem::zeroed_init};

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum TextElementConfigWrapMode {
    Words = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS,
    Newline = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES,
    None = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE,
}

pub struct TextElementConfig {
    inner: *mut Clay_TextElementConfig,
}

impl From<TextElementConfig> for *mut Clay_TextElementConfig {
    fn from(value: TextElementConfig) -> Self {
        value.inner
    }
}
impl From<*mut Clay_TextElementConfig> for TextElementConfig {
    fn from(value: *mut Clay_TextElementConfig) -> Self {
        Self { inner: value }
    }
}

pub struct Text {
    inner: Clay_TextElementConfig,
}

impl Text {
    pub fn new() -> Self {
        Self {
            inner: zeroed_init(),
        }
    }

    pub fn color(&mut self, color: Color) -> &mut Self {
        self.inner.textColor = color.into();
        self
    }

    pub fn font_size(&mut self, size: u16) -> &mut Self {
        self.inner.fontSize = size;
        self
    }

    pub fn font_id(&mut self, id: u16) -> &mut Self {
        self.inner.fontId = id;
        self
    }

    pub fn letter_spacing(&mut self, spacing: u16) -> &mut Self {
        self.inner.letterSpacing = spacing;
        self
    }

    pub fn line_height(&mut self, height: u16) -> &mut Self {
        self.inner.lineHeight = height;
        self
    }

    pub fn wrap_mode(&mut self, mode: TextElementConfigWrapMode) -> &mut Self {
        self.inner.wrapMode = mode as u32;
        self
    }

    pub fn end(&self) -> TextElementConfig {
        let memory = unsafe { Clay__StoreTextElementConfig(self.inner) };

        TextElementConfig { inner: memory }
    }
}
