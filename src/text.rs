use crate::{bindings::*, color::Color};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TextElementConfigWrapMode {
    /// Wraps on whitespaces not breaking words
    Words = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_WORDS,
    /// Only wraps on new line characters
    Newline = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NEWLINES,
    /// Never wraps, can overflow of parent layout
    None = Clay_TextElementConfigWrapMode_CLAY_TEXT_WRAP_NONE,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum TextAlignment {
    /// Aligns the text to the left.
    Left = Clay_TextAlignment_CLAY_TEXT_ALIGN_LEFT,
    /// Aligns the text to the center.
    Center = Clay_TextAlignment_CLAY_TEXT_ALIGN_CENTER,
    /// Aligns the text to the right.
    Right = Clay_TextAlignment_CLAY_TEXT_ALIGN_RIGHT,
}

pub struct TextElementConfig {
    inner: *mut Clay_TextElementConfig,
}

impl From<TextElementConfig> for *mut Clay_TextElementConfig {
    fn from(value: TextElementConfig) -> Self {
        value.inner
    }
}

/// Configuration settings for rendering text elements.
#[derive(Debug, Clone, Copy)]
pub struct TextConfig {
    /// The color of the text.
    pub color: Color,
    /// Clay does not manage fonts. It is up to the user to assign a unique ID to each font
    /// and provide it via the [`font_id`](Text::font_id) field.
    pub font_id: u16,
    /// The font size of the text.
    pub font_size: u16,
    /// The spacing between letters.
    pub letter_spacing: u16,
    /// The height of each line of text.
    pub line_height: u16,
    /// Defines the text wrapping behavior.
    pub wrap_mode: TextElementConfigWrapMode,
    /// The alignment of the text.
    pub alignment: TextAlignment,
}

impl TextConfig {
    /// Creates a new `TextConfig` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text color.
    #[inline]
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets the font ID. The user is responsible for assigning unique font IDs.
    #[inline]
    pub fn font_id(&mut self, id: u16) -> &mut Self {
        self.font_id = id;
        self
    }

    /// Sets the font size.
    #[inline]
    pub fn font_size(&mut self, size: u16) -> &mut Self {
        self.font_size = size;
        self
    }

    /// Sets the letter spacing.
    #[inline]
    pub fn letter_spacing(&mut self, spacing: u16) -> &mut Self {
        self.letter_spacing = spacing;
        self
    }

    /// Sets the line height.
    #[inline]
    pub fn line_height(&mut self, height: u16) -> &mut Self {
        self.line_height = height;
        self
    }

    /// Sets the text wrapping mode.
    #[inline]
    pub fn wrap_mode(&mut self, mode: TextElementConfigWrapMode) -> &mut Self {
        self.wrap_mode = mode;
        self
    }

    /// Sets the text alignment.
    #[inline]
    pub fn alignment(&mut self, alignment: TextAlignment) -> &mut Self {
        self.alignment = alignment;
        self
    }

    /// Finalizes the text configuration and stores it in memory.
    #[inline]
    pub fn end(&self) -> TextElementConfig {
        let memory = unsafe { Clay__StoreTextElementConfig((*self).into()) };
        TextElementConfig { inner: memory }
    }
}

impl Default for TextConfig {
    fn default() -> Self {
        Self {
            color: Color::rgba(0., 0., 0., 0.),
            font_id: 0,
            font_size: 0,
            letter_spacing: 0,
            line_height: 0,
            wrap_mode: TextElementConfigWrapMode::Words,
            alignment: TextAlignment::Left,
        }
    }
}

impl From<TextConfig> for Clay_TextElementConfig {
    fn from(value: TextConfig) -> Self {
        Self {
            userData: core::ptr::null_mut(),
            textColor: value.color.into(),
            fontId: value.font_id,
            fontSize: value.font_size,
            letterSpacing: value.letter_spacing,
            lineHeight: value.line_height,
            wrapMode: value.wrap_mode as _,
            textAlignment: value.alignment as _,
        }
    }
}

impl From<Clay_TextElementConfig> for TextConfig {
    fn from(value: Clay_TextElementConfig) -> Self {
        Self {
            color: value.textColor.into(),
            font_id: value.fontId,
            font_size: value.fontSize,
            letter_spacing: value.letterSpacing,
            line_height: value.lineHeight,
            wrap_mode: unsafe {
                core::mem::transmute::<u8, TextElementConfigWrapMode>(value.wrapMode)
            },
            alignment: unsafe { core::mem::transmute::<u8, TextAlignment>(value.textAlignment) },
        }
    }
}
