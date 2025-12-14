//! # Sigil ✨
//!
//! Human-readable ANSI escape sequences for terminal styling.
//!
//! Sigil makes it easy to work with ANSI escape codes in a type-safe,
//! readable way. It's the Rust equivalent of [sequin](https://github.com/charmbracelet/sequin)
//! from Charmbracelet.
//!
//! ## Quick Start
//!
//! ```rust
//! use sigil::{style, Color, Modifier};
//!
//! // Style some text
//! let styled = style("Hello, World!")
//!     .fg(Color::Red)
//!     .bold()
//!     .to_string();
//!
//! println!("{}", styled);
//!
//! // Use RGB colors
//! let rgb = style("Molten Orange")
//!     .fg(Color::rgb(249, 115, 22))
//!     .to_string();
//!
//! // Parse existing ANSI sequences
//! let parsed = sigil::parse("\x1b[31mRed text\x1b[0m");
//! ```
//!
//! ## Features
//!
//! - **Type-safe** - No string manipulation, just types
//! - **Human-readable** - Parse ANSI codes to readable descriptions
//! - **Zero-copy** - Efficient string handling where possible
//! - **Brand integration** - Optional Molten brand colors via `brand` feature

#![deny(missing_docs)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod color;
mod escape;
mod modifier;
mod parser;
mod sequence;
mod style;

pub use color::Color;
#[cfg(feature = "brand")]
pub use color::brand;
pub use escape::{Escape, EscapeKind};
pub use modifier::Modifier;
pub use parser::{parse, strip_ansi, visible_len, ParsedSequence};
pub use sequence::{Sequence, SequenceBuilder};
pub use style::{style, Style, Styled};

/// CSI (Control Sequence Introducer) prefix.
pub const CSI: &str = "\x1b[";

/// OSC (Operating System Command) prefix.
pub const OSC: &str = "\x1b]";

/// SGR (Select Graphic Rendition) suffix.
pub const SGR_SUFFIX: &str = "m";

/// Reset all attributes.
pub const RESET: &str = "\x1b[0m";

/// Common escape sequences.
pub mod sequences {
    /// Clear the entire screen.
    pub const CLEAR_SCREEN: &str = "\x1b[2J";

    /// Clear from cursor to end of screen.
    pub const CLEAR_TO_END: &str = "\x1b[0J";

    /// Clear from cursor to start of screen.
    pub const CLEAR_TO_START: &str = "\x1b[1J";

    /// Clear the entire line.
    pub const CLEAR_LINE: &str = "\x1b[2K";

    /// Clear from cursor to end of line.
    pub const CLEAR_LINE_TO_END: &str = "\x1b[0K";

    /// Clear from cursor to start of line.
    pub const CLEAR_LINE_TO_START: &str = "\x1b[1K";

    /// Move cursor to home position (0, 0).
    pub const CURSOR_HOME: &str = "\x1b[H";

    /// Hide cursor.
    pub const CURSOR_HIDE: &str = "\x1b[?25l";

    /// Show cursor.
    pub const CURSOR_SHOW: &str = "\x1b[?25h";

    /// Save cursor position.
    pub const CURSOR_SAVE: &str = "\x1b[s";

    /// Restore cursor position.
    pub const CURSOR_RESTORE: &str = "\x1b[u";

    /// Enter alternate screen buffer.
    pub const ALT_SCREEN_ENTER: &str = "\x1b[?1049h";

    /// Exit alternate screen buffer.
    pub const ALT_SCREEN_EXIT: &str = "\x1b[?1049l";

    /// Enable mouse tracking.
    pub const MOUSE_ENABLE: &str = "\x1b[?1000h";

    /// Disable mouse tracking.
    pub const MOUSE_DISABLE: &str = "\x1b[?1000l";

    /// Enable bracketed paste mode.
    pub const BRACKETED_PASTE_ENABLE: &str = "\x1b[?2004h";

    /// Disable bracketed paste mode.
    pub const BRACKETED_PASTE_DISABLE: &str = "\x1b[?2004l";
}

/// Cursor movement helpers.
pub mod cursor {
    /// Move cursor up by n rows.
    #[must_use]
    pub fn up(n: u16) -> String {
        format!("\x1b[{n}A")
    }

    /// Move cursor down by n rows.
    #[must_use]
    pub fn down(n: u16) -> String {
        format!("\x1b[{n}B")
    }

    /// Move cursor right by n columns.
    #[must_use]
    pub fn right(n: u16) -> String {
        format!("\x1b[{n}C")
    }

    /// Move cursor left by n columns.
    #[must_use]
    pub fn left(n: u16) -> String {
        format!("\x1b[{n}D")
    }

    /// Move cursor to specific position (1-indexed).
    #[must_use]
    pub fn goto(row: u16, col: u16) -> String {
        format!("\x1b[{row};{col}H")
    }

    /// Move cursor to specific column (1-indexed).
    #[must_use]
    pub fn column(col: u16) -> String {
        format!("\x1b[{col}G")
    }
}

/// Prelude for convenient imports.
pub mod prelude {
    pub use crate::color::Color;
    pub use crate::modifier::Modifier;
    pub use crate::style::{style, Style, Styled};
    pub use crate::{cursor, sequences, RESET};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_basic() {
        let s = style("test").fg(Color::Red).to_string();
        assert!(s.contains("\x1b["));
        assert!(s.contains("test"));
        assert!(s.ends_with(RESET));
    }

    #[test]
    fn test_cursor_movement() {
        assert_eq!(cursor::up(5), "\x1b[5A");
        assert_eq!(cursor::goto(10, 20), "\x1b[10;20H");
    }
}
