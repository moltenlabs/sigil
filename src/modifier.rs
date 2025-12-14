//! Text modifiers (bold, italic, underline, etc.).

use std::fmt;

/// Text style modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Modifier {
    /// Bold text.
    Bold,
    /// Dim/faint text.
    Dim,
    /// Italic text.
    Italic,
    /// Underlined text.
    Underline,
    /// Blinking text (slow).
    Blink,
    /// Rapidly blinking text.
    RapidBlink,
    /// Reversed/inverted colors.
    Reverse,
    /// Hidden/invisible text.
    Hidden,
    /// Strikethrough text.
    Strikethrough,
    /// Double underline.
    DoubleUnderline,
    /// Overlined text.
    Overline,
}

impl Modifier {
    /// Get the ANSI SGR code to enable this modifier.
    #[must_use]
    pub const fn on_code(&self) -> u8 {
        match self {
            Self::Bold => 1,
            Self::Dim => 2,
            Self::Italic => 3,
            Self::Underline => 4,
            Self::Blink => 5,
            Self::RapidBlink => 6,
            Self::Reverse => 7,
            Self::Hidden => 8,
            Self::Strikethrough => 9,
            Self::DoubleUnderline => 21,
            Self::Overline => 53,
        }
    }

    /// Get the ANSI SGR code to disable this modifier.
    #[must_use]
    pub const fn off_code(&self) -> u8 {
        match self {
            Self::Bold | Self::Dim => 22,
            Self::Italic => 23,
            Self::Underline | Self::DoubleUnderline => 24,
            Self::Blink | Self::RapidBlink => 25,
            Self::Reverse => 27,
            Self::Hidden => 28,
            Self::Strikethrough => 29,
            Self::Overline => 55,
        }
    }

    /// Get a human-readable name for this modifier.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Bold => "bold",
            Self::Dim => "dim",
            Self::Italic => "italic",
            Self::Underline => "underline",
            Self::Blink => "blink",
            Self::RapidBlink => "rapid blink",
            Self::Reverse => "reverse",
            Self::Hidden => "hidden",
            Self::Strikethrough => "strikethrough",
            Self::DoubleUnderline => "double underline",
            Self::Overline => "overline",
        }
    }
}

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// A set of modifiers.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct ModifierSet {
    bits: u16,
}

impl ModifierSet {
    // Bit positions for each modifier (0-10, fitting in u16)
    const BOLD_BIT: u16 = 0;
    const DIM_BIT: u16 = 1;
    const ITALIC_BIT: u16 = 2;
    const UNDERLINE_BIT: u16 = 3;
    const BLINK_BIT: u16 = 4;
    const RAPID_BLINK_BIT: u16 = 5;
    const REVERSE_BIT: u16 = 6;
    const HIDDEN_BIT: u16 = 7;
    const STRIKETHROUGH_BIT: u16 = 8;
    const DOUBLE_UNDERLINE_BIT: u16 = 9;
    const OVERLINE_BIT: u16 = 10;

    const fn bit_for(modifier: Modifier) -> u16 {
        match modifier {
            Modifier::Bold => Self::BOLD_BIT,
            Modifier::Dim => Self::DIM_BIT,
            Modifier::Italic => Self::ITALIC_BIT,
            Modifier::Underline => Self::UNDERLINE_BIT,
            Modifier::Blink => Self::BLINK_BIT,
            Modifier::RapidBlink => Self::RAPID_BLINK_BIT,
            Modifier::Reverse => Self::REVERSE_BIT,
            Modifier::Hidden => Self::HIDDEN_BIT,
            Modifier::Strikethrough => Self::STRIKETHROUGH_BIT,
            Modifier::DoubleUnderline => Self::DOUBLE_UNDERLINE_BIT,
            Modifier::Overline => Self::OVERLINE_BIT,
        }
    }

    /// Create an empty modifier set.
    #[must_use]
    pub const fn empty() -> Self {
        Self { bits: 0 }
    }

    /// Add a modifier to the set.
    #[must_use]
    pub const fn with(self, modifier: Modifier) -> Self {
        Self {
            bits: self.bits | (1 << Self::bit_for(modifier)),
        }
    }

    /// Check if a modifier is in the set.
    #[must_use]
    pub const fn contains(self, modifier: Modifier) -> bool {
        (self.bits & (1 << Self::bit_for(modifier))) != 0
    }

    /// Get all enabled modifiers.
    #[must_use]
    pub fn modifiers(self) -> Vec<Modifier> {
        let all = [
            Modifier::Bold,
            Modifier::Dim,
            Modifier::Italic,
            Modifier::Underline,
            Modifier::Blink,
            Modifier::RapidBlink,
            Modifier::Reverse,
            Modifier::Hidden,
            Modifier::Strikethrough,
            Modifier::DoubleUnderline,
            Modifier::Overline,
        ];
        all.into_iter().filter(|m| self.contains(*m)).collect()
    }

    /// Get the ANSI codes for all enabled modifiers.
    #[must_use]
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub fn codes(self) -> Vec<u8> {
        self.modifiers().iter().map(|m| m.on_code()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modifier_codes() {
        assert_eq!(Modifier::Bold.on_code(), 1);
        assert_eq!(Modifier::Bold.off_code(), 22);
        assert_eq!(Modifier::Underline.on_code(), 4);
    }

    #[test]
    fn test_modifier_set() {
        let set = ModifierSet::empty()
            .with(Modifier::Bold)
            .with(Modifier::Italic);
        
        assert!(set.contains(Modifier::Bold));
        assert!(set.contains(Modifier::Italic));
        assert!(!set.contains(Modifier::Underline));
    }
}
