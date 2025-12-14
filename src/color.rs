//! Color definitions for terminal styling.

use std::fmt;

/// A terminal color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    /// Default terminal color.
    Default,
    /// Black (ANSI 0).
    Black,
    /// Red (ANSI 1).
    Red,
    /// Green (ANSI 2).
    Green,
    /// Yellow (ANSI 3).
    Yellow,
    /// Blue (ANSI 4).
    Blue,
    /// Magenta (ANSI 5).
    Magenta,
    /// Cyan (ANSI 6).
    Cyan,
    /// White (ANSI 7).
    White,
    /// Bright black (ANSI 8).
    BrightBlack,
    /// Bright red (ANSI 9).
    BrightRed,
    /// Bright green (ANSI 10).
    BrightGreen,
    /// Bright yellow (ANSI 11).
    BrightYellow,
    /// Bright blue (ANSI 12).
    BrightBlue,
    /// Bright magenta (ANSI 13).
    BrightMagenta,
    /// Bright cyan (ANSI 14).
    BrightCyan,
    /// Bright white (ANSI 15).
    BrightWhite,
    /// 256-color palette (0-255).
    Ansi256(u8),
    /// True color RGB.
    Rgb {
        /// Red component (0-255).
        r: u8,
        /// Green component (0-255).
        g: u8,
        /// Blue component (0-255).
        b: u8,
    },
}

impl Color {
    /// Create an RGB color.
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb { r, g, b }
    }

    /// Create a color from a hex string (with or without #).
    ///
    /// # Panics
    ///
    /// Panics if the hex string is invalid.
    #[must_use]
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid hex");
        let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid hex");
        let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid hex");
        Self::Rgb { r, g, b }
    }

    /// Create a 256-color palette color.
    #[must_use]
    pub const fn ansi256(code: u8) -> Self {
        Self::Ansi256(code)
    }

    /// Get the ANSI SGR code for foreground.
    #[must_use]
    pub fn fg_code(&self) -> String {
        match self {
            Self::Default => "39".to_string(),
            Self::Black => "30".to_string(),
            Self::Red => "31".to_string(),
            Self::Green => "32".to_string(),
            Self::Yellow => "33".to_string(),
            Self::Blue => "34".to_string(),
            Self::Magenta => "35".to_string(),
            Self::Cyan => "36".to_string(),
            Self::White => "37".to_string(),
            Self::BrightBlack => "90".to_string(),
            Self::BrightRed => "91".to_string(),
            Self::BrightGreen => "92".to_string(),
            Self::BrightYellow => "93".to_string(),
            Self::BrightBlue => "94".to_string(),
            Self::BrightMagenta => "95".to_string(),
            Self::BrightCyan => "96".to_string(),
            Self::BrightWhite => "97".to_string(),
            Self::Ansi256(code) => format!("38;5;{code}"),
            Self::Rgb { r, g, b } => format!("38;2;{r};{g};{b}"),
        }
    }

    /// Get the ANSI SGR code for background.
    #[must_use]
    pub fn bg_code(&self) -> String {
        match self {
            Self::Default => "49".to_string(),
            Self::Black => "40".to_string(),
            Self::Red => "41".to_string(),
            Self::Green => "42".to_string(),
            Self::Yellow => "43".to_string(),
            Self::Blue => "44".to_string(),
            Self::Magenta => "45".to_string(),
            Self::Cyan => "46".to_string(),
            Self::White => "47".to_string(),
            Self::BrightBlack => "100".to_string(),
            Self::BrightRed => "101".to_string(),
            Self::BrightGreen => "102".to_string(),
            Self::BrightYellow => "103".to_string(),
            Self::BrightBlue => "104".to_string(),
            Self::BrightMagenta => "105".to_string(),
            Self::BrightCyan => "106".to_string(),
            Self::BrightWhite => "107".to_string(),
            Self::Ansi256(code) => format!("48;5;{code}"),
            Self::Rgb { r, g, b } => format!("48;2;{r};{g};{b}"),
        }
    }

    /// Get a human-readable name for the color.
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::Default => "default".to_string(),
            Self::Black => "black".to_string(),
            Self::Red => "red".to_string(),
            Self::Green => "green".to_string(),
            Self::Yellow => "yellow".to_string(),
            Self::Blue => "blue".to_string(),
            Self::Magenta => "magenta".to_string(),
            Self::Cyan => "cyan".to_string(),
            Self::White => "white".to_string(),
            Self::BrightBlack => "bright black".to_string(),
            Self::BrightRed => "bright red".to_string(),
            Self::BrightGreen => "bright green".to_string(),
            Self::BrightYellow => "bright yellow".to_string(),
            Self::BrightBlue => "bright blue".to_string(),
            Self::BrightMagenta => "bright magenta".to_string(),
            Self::BrightCyan => "bright cyan".to_string(),
            Self::BrightWhite => "bright white".to_string(),
            Self::Ansi256(code) => format!("color {code}"),
            Self::Rgb { r, g, b } => format!("rgb({r}, {g}, {b})"),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Convert from `molten_brand` colors when the `brand` feature is enabled.
#[cfg(feature = "brand")]
impl From<molten_brand::Color> for Color {
    fn from(color: molten_brand::Color) -> Self {
        let rgb = color.to_rgb();
        Self::Rgb {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

#[cfg(feature = "brand")]
impl From<molten_brand::Rgb> for Color {
    fn from(rgb: molten_brand::Rgb) -> Self {
        Self::Rgb {
            r: rgb.r,
            g: rgb.g,
            b: rgb.b,
        }
    }
}

/// Molten brand color presets (when `brand` feature is enabled).
#[cfg(feature = "brand")]
pub mod brand {
    use super::Color;

    /// Molten Orange - primary brand color.
    pub const MOLTEN: Color = Color::Rgb {
        r: 249,
        g: 115,
        b: 22,
    };

    /// Goblin Purple - Lair primary color.
    pub const GOBLIN: Color = Color::Rgb {
        r: 124,
        g: 58,
        b: 237,
    };

    /// Iron Blue - Hearth primary color.
    pub const IRON: Color = Color::Rgb {
        r: 59,
        g: 130,
        b: 246,
    };

    /// Forge Black - background color.
    pub const FORGE_BLACK: Color = Color::Rgb { r: 10, g: 10, b: 10 };

    /// Success green.
    pub const SUCCESS: Color = Color::Rgb {
        r: 16,
        g: 185,
        b: 129,
    };

    /// Warning amber.
    pub const WARNING: Color = Color::Rgb {
        r: 245,
        g: 158,
        b: 11,
    };

    /// Error red.
    pub const ERROR: Color = Color::Rgb {
        r: 239,
        g: 68,
        b: 68,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fg_codes() {
        assert_eq!(Color::Red.fg_code(), "31");
        assert_eq!(Color::BrightBlue.fg_code(), "94");
        assert_eq!(Color::Ansi256(42).fg_code(), "38;5;42");
        assert_eq!(Color::rgb(255, 128, 0).fg_code(), "38;2;255;128;0");
    }

    #[test]
    fn test_from_hex() {
        let color = Color::from_hex("#F97316");
        assert_eq!(color, Color::Rgb { r: 249, g: 115, b: 22 });
    }
}
