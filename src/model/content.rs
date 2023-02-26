use crate::Language;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Content is basically the abstraction
/// of a markdown page.
/// All elements here should feel familiar
/// if you know markdown.
/// Crate documentation is written using markdown,
/// so it should be representable as such when parsed.

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TextStyle {
    bold: Option<bool>,
    code: Option<bool>, // inline code, like `markdown`
    italic: Option<bool>,
    strike_through: Option<bool>,
    underline: Option<bool>,
    foreground_rgb: Option<(u8, u8, u8)>,
    background_rgb: Option<(u8, u8, u8)>,
}

impl fmt::Debug for TextStyle {
    // When comaring big structs, TextStyle is really
    // noisy. To noise down, hide fields being None.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut x = f.debug_struct("TextStyle");
        if self.bold.is_some() {
            x.field("bold", &self.bold);
        }
        if self.code.is_some() {
            x.field("code", &self.code);
        }
        if self.italic.is_some() {
            x.field("italic", &self.italic);
        }
        if self.strike_through.is_some() {
            x.field("strike_through", &self.strike_through);
        }
        if self.underline.is_some() {
            x.field("underline", &self.underline);
        }
        if self.foreground_rgb.is_some() {
            x.field("foreground_rgb", &self.foreground_rgb);
        }
        if self.background_rgb.is_some() {
            x.field("background_rgb", &self.background_rgb);
        }

        x.finish()
    }
}

// Example:
// "a <code>b</code> c"
// would consist of three text atomics:
// - "a "
// - "b" with code style
// - " c"
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TextAtomic {
    pub text: String,
    pub style: TextStyle,
    pub url: Option<String>,
}

impl TextAtomic {
    pub fn simple<T: ToString>(text: &T) -> Self {
        Self {
            text: text.to_string(),
            style: TextStyle::default(),
            url: None,
        }
    }
    pub fn with_some_url<T: ToString>(mut self, url: &T) -> Self {
        self.url = Some(url.to_string());
        self
    }
    pub fn with_url(mut self, url: Option<String>) -> Self {
        self.url = url;
        self
    }
}

pub struct TextAtomicBuilder(TextAtomic);

impl TextAtomicBuilder {
    pub fn new<T>(content: &T) -> Self
    where
        T: ToString,
    {
        Self(TextAtomic {
            text: content.to_string(),
            style: TextStyle::default(),
            url: None,
        })
    }

    pub fn set_url<S: ToString>(mut self, url: &S) -> Self {
        self.0.url = Some(url.to_string());
        self
    }

    pub fn bold(mut self, enabled: bool) -> Self {
        self.0.style.bold = Some(enabled);
        self
    }

    pub fn code(mut self, enabled: bool) -> Self {
        self.0.style.code = Some(enabled);
        self
    }

    pub fn italic(mut self, enabled: bool) -> Self {
        self.0.style.italic = Some(enabled);
        self
    }

    pub fn strike_through(mut self, enabled: bool) -> Self {
        self.0.style.strike_through = Some(enabled);
        self
    }

    pub fn underline(mut self, enabled: bool) -> Self {
        self.0.style.underline = Some(enabled);
        self
    }

    pub fn foreground_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.0.style.foreground_rgb = Some((r, g, b));
        self
    }

    pub fn background_rgb(mut self, r: u8, g: u8, b: u8) -> Self {
        self.0.style.background_rgb = Some((r, g, b));
        self
    }

    pub fn build(self) -> TextAtomic {
        self.0
    }
}

impl Into<TextAtomic> for TextAtomicBuilder {
    fn into(self) -> TextAtomic {
        self.build()
    }
}

// Something which can not be embedded inline.
// For example a table, an image or a list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockContainer {
    Heading1(Vec<TextAtomic>),
    Heading2(Vec<TextAtomic>),
    Heading3(Vec<TextAtomic>),
    Heading4(Vec<TextAtomic>),
    Paragraph(Vec<TextAtomic>), // Simple text without newlines
    // Multiline quote
    Quote(Vec<BlockContainer>),
    // Code block, not inline code.
    // Style will be ignored in favor of
    // formatting the entire code at once.
    Code {
        code: String,
        language: Option<Language>,
    },
    BulletPoints {
        points: Vec<BlockContainer>,
        enumerated: bool, // points or 1. 2. 3.
    },
    // Vec of rows.
    // Rows are Vec of Cells.
    // Cells are Text.
    Table(Vec<Vec<Vec<TextAtomic>>>),
    // This one is not renderable in a terminal,
    // but it has to be represented somehow anyways.
    Image {
        url: String,
        alt: Option<String>,
    },
}

fn merge<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().chain(b.into_iter()).collect::<Vec<T>>()
}

pub enum BlockContainerAddition {
    Merged(BlockContainer),
    NotMerged(BlockContainer, BlockContainer),
}

// Allows to merge BlockContainers if possible.
//
// This is ugly as f'ck. I am open to suggestions.
// Also clone sucks.
impl std::ops::Add for BlockContainer {
    type Output = BlockContainerAddition;

    fn add(self, rhs: Self) -> Self::Output {
        match self.clone() {
            BlockContainer::Heading1(c) => match rhs {
                BlockContainer::Heading1(d) => {
                    BlockContainerAddition::Merged(BlockContainer::Heading1(merge(c, d)))
                }
                _ => BlockContainerAddition::NotMerged(self, rhs),
            },
            BlockContainer::Heading2(c) => match rhs {
                BlockContainer::Heading2(d) => {
                    BlockContainerAddition::Merged(BlockContainer::Heading2(merge(c, d)))
                }
                _ => BlockContainerAddition::NotMerged(self, rhs),
            },
            BlockContainer::Heading3(c) => match rhs {
                BlockContainer::Heading3(d) => {
                    BlockContainerAddition::Merged(BlockContainer::Heading3(merge(c, d)))
                }
                _ => BlockContainerAddition::NotMerged(self, rhs),
            },
            BlockContainer::Heading4(c) => match rhs {
                BlockContainer::Heading4(d) => {
                    BlockContainerAddition::Merged(BlockContainer::Heading4(merge(c, d)))
                }
                _ => BlockContainerAddition::NotMerged(self, rhs),
            },
            BlockContainer::Paragraph(c) => match rhs {
                BlockContainer::Paragraph(d) => {
                    BlockContainerAddition::Merged(BlockContainer::Paragraph(merge(c, d)))
                }
                _ => BlockContainerAddition::NotMerged(self, rhs),
            },
            BlockContainer::Quote(c) => match rhs {
                BlockContainer::Quote(d) => {
                    BlockContainerAddition::Merged(BlockContainer::Quote(merge(c, d)))
                }
                _ => BlockContainerAddition::NotMerged(self, rhs),
            },
            _ => BlockContainerAddition::NotMerged(self, rhs),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Content(pub Vec<BlockContainer>);
