//! Text styling API.

use crate::color::Color;
use crate::modifier::{Modifier, ModifierSet};
use crate::{RESET, CSI, SGR_SUFFIX};
use std::fmt;

/// Create a styled string.
///
/// # Example
///
/// ```rust
/// use sigil::{style, Color};
///
/// let s = style("Hello")
///     .fg(Color::Red)
///     .bold()
///     .to_string();
/// ```
#[must_use]
pub fn style<S: Into<String>>(text: S) -> Styled {
    Styled::new(text.into())
}

/// A styled string with colors and modifiers.
#[derive(Debug, Clone)]
pub struct Styled {
    text: String,
    style: Style,
}

impl Styled {
    /// Create a new styled string.
    #[must_use]
    pub fn new(text: String) -> Self {
        Self {
            text,
            style: Style::default(),
        }
    }

    /// Set the foreground color.
    #[must_use]
    pub fn fg(mut self, color: Color) -> Self {
        self.style.foreground = Some(color);
        self
    }

    /// Set the background color.
    #[must_use]
    pub fn bg(mut self, color: Color) -> Self {
        self.style.background = Some(color);
        self
    }

    /// Make the text bold.
    #[must_use]
    pub fn bold(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Bold);
        self
    }

    /// Make the text dim.
    #[must_use]
    pub fn dim(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Dim);
        self
    }

    /// Make the text italic.
    #[must_use]
    pub fn italic(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Italic);
        self
    }

    /// Underline the text.
    #[must_use]
    pub fn underline(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Underline);
        self
    }

    /// Make the text blink.
    #[must_use]
    pub fn blink(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Blink);
        self
    }

    /// Reverse/invert the colors.
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Reverse);
        self
    }

    /// Hide the text.
    #[must_use]
    pub fn hidden(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Hidden);
        self
    }

    /// Strikethrough the text.
    #[must_use]
    pub fn strikethrough(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Strikethrough);
        self
    }

    /// Add an overline.
    #[must_use]
    pub fn overline(mut self) -> Self {
        self.style.modifiers = self.style.modifiers.with(Modifier::Overline);
        self
    }

    /// Get the underlying text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Get the style.
    #[must_use]
    pub const fn get_style(&self) -> &Style {
        &self.style
    }

    /// Render to a string with ANSI codes.
    #[must_use]
    pub fn render(&self) -> String {
        let codes = self.style.codes();
        if codes.is_empty() {
            return self.text.clone();
        }

        let codes_str = codes
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(";");

        format!("{CSI}{codes_str}{SGR_SUFFIX}{}{RESET}", self.text)
    }
}

impl fmt::Display for Styled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render())
    }
}

/// A style definition (without text).
#[derive(Debug, Clone, Default)]
pub struct Style {
    /// Foreground color.
    pub foreground: Option<Color>,
    /// Background color.
    pub background: Option<Color>,
    /// Text modifiers.
    pub modifiers: ModifierSet,
}

impl Style {
    /// Create a new empty style.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            foreground: None,
            background: None,
            modifiers: ModifierSet::empty(),
        }
    }

    /// Set the foreground color.
    #[must_use]
    pub const fn fg(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    /// Set the background color.
    #[must_use]
    pub const fn bg(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Add a modifier.
    #[must_use]
    pub const fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers = self.modifiers.with(modifier);
        self
    }

    /// Get the ANSI codes for this style.
    #[must_use]
    pub fn codes(&self) -> Vec<String> {
        let mut codes = Vec::new();

        // Add modifier codes
        for modifier in self.modifiers.modifiers() {
            codes.push(modifier.on_code().to_string());
        }

        // Add foreground color
        if let Some(fg) = &self.foreground {
            codes.push(fg.fg_code());
        }

        // Add background color
        if let Some(bg) = &self.background {
            codes.push(bg.bg_code());
        }

        codes
    }

    /// Apply this style to a string.
    #[must_use]
    pub fn apply(&self, text: &str) -> String {
        let codes = self.codes();
        if codes.is_empty() {
            return text.to_string();
        }

        let codes_str = codes.join(";");
        format!("{CSI}{codes_str}{SGR_SUFFIX}{text}{RESET}")
    }

    /// Get a human-readable description of this style.
    #[must_use]
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();

        for modifier in self.modifiers.modifiers() {
            parts.push(modifier.name().to_string());
        }

        if let Some(fg) = &self.foreground {
            parts.push(format!("fg: {}", fg.name()));
        }

        if let Some(bg) = &self.background {
            parts.push(format!("bg: {}", bg.name()));
        }

        if parts.is_empty() {
            "no style".to_string()
        } else {
            parts.join(", ")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_rendering() {
        let s = style("test").fg(Color::Red).to_string();
        assert!(s.starts_with("\x1b["));
        assert!(s.contains("31")); // Red foreground
        assert!(s.contains("test"));
        assert!(s.ends_with("\x1b[0m"));
    }

    #[test]
    fn test_style_with_modifiers() {
        let s = style("test").bold().underline().to_string();
        assert!(s.contains("1")); // Bold
        assert!(s.contains("4")); // Underline
    }

    #[test]
    fn test_style_description() {
        let style = Style::new()
            .fg(Color::Red)
            .modifier(Modifier::Bold);
        
        let desc = style.describe();
        assert!(desc.contains("bold"));
        assert!(desc.contains("red"));
    }
}
