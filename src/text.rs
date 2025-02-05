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
    /// If `true`, the string contents are hashed to prevent unnecessary updates.
    /// Set this to `false` if the text element is updated frequently.
    pub hash_string_contents: bool,
}

impl TextConfig {
    /// Creates a new `TextConfig` instance with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text color.
    pub fn color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    /// Sets the font ID. The user is responsible for assigning unique font IDs.
    pub fn font_id(&mut self, id: u16) -> &mut Self {
        self.font_id = id;
        self
    }

    /// Sets the font size.
    pub fn font_size(&mut self, size: u16) -> &mut Self {
        self.font_size = size;
        self
    }

    /// Sets the letter spacing.
    pub fn letter_spacing(&mut self, spacing: u16) -> &mut Self {
        self.letter_spacing = spacing;
        self
    }

    /// Sets the line height.
    pub fn line_height(&mut self, height: u16) -> &mut Self {
        self.line_height = height;
        self
    }

    /// Enables or disables hashing of string contents.
    ///
    /// If the text content changes frequently, set this to `true` otherwise Clay will cache this
    /// and not rehash the content. 
    pub fn hash_string_contents(&mut self, do_hash: bool) -> &mut Self {
        self.hash_string_contents = do_hash;
        self
    }

    /// Sets the text wrapping mode.
    pub fn wrap_mode(&mut self, mode: TextElementConfigWrapMode) -> &mut Self {
        self.wrap_mode = mode;
        self
    }

    /// Finalizes the text configuration and stores it in memory.
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
            hash_string_contents: true,
        }
    }
}

impl From<TextConfig> for Clay_TextElementConfig {
    fn from(value: TextConfig) -> Self {
        Self {
            textColor: value.color.into(),
            fontId: value.font_id,
            fontSize: value.font_size,
            letterSpacing: value.letter_spacing,
            lineHeight: value.line_height,
            wrapMode: value.wrap_mode as _,
            hashStringContents: value.hash_string_contents,
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
            wrap_mode: unsafe { core::mem::transmute(value.wrapMode) }, 
            hash_string_contents: value.hashStringContents,
        }
    }
}
