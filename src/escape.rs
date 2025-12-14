//! Escape sequence types.

use std::fmt;

/// Kind of escape sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeKind {
    /// SGR (Select Graphic Rendition) - colors and text styling.
    Sgr,
    /// Cursor movement.
    Cursor,
    /// Screen/line clearing.
    Erase,
    /// Mode setting (e.g., alternate screen).
    Mode,
    /// OSC (Operating System Command).
    Osc,
    /// Unknown/other escape.
    Unknown,
}

impl fmt::Display for EscapeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sgr => write!(f, "SGR (style)"),
            Self::Cursor => write!(f, "cursor"),
            Self::Erase => write!(f, "erase"),
            Self::Mode => write!(f, "mode"),
            Self::Osc => write!(f, "OSC"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

/// A parsed escape sequence.
#[derive(Debug, Clone)]
pub struct Escape {
    /// The raw sequence string.
    pub raw: String,
    /// The kind of escape.
    pub kind: EscapeKind,
    /// Human-readable description.
    pub description: String,
    /// Parameters (if any).
    pub params: Vec<u16>,
}

impl Escape {
    /// Create a new escape sequence.
    #[must_use]
    pub fn new(raw: String, kind: EscapeKind, description: String) -> Self {
        Self {
            raw,
            kind,
            description,
            params: Vec::new(),
        }
    }

    /// Create with parameters.
    #[must_use]
    pub fn with_params(mut self, params: Vec<u16>) -> Self {
        self.params = params;
        self
    }

    /// Get a human-readable representation.
    #[must_use]
    pub fn human_readable(&self) -> String {
        format!("[{}] {}", self.kind, self.description)
    }
}

impl fmt::Display for Escape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.human_readable())
    }
}

/// Parse SGR (style) parameters into a description.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn describe_sgr(params: &[u16]) -> String {
    if params.is_empty() || params == [0] {
        return "reset".to_string();
    }

    let mut descriptions = Vec::new();
    let mut i = 0;

    while i < params.len() {
        let desc = match params[i] {
            0 => "reset",
            1 => "bold",
            2 => "dim",
            3 => "italic",
            4 => "underline",
            5 => "blink",
            7 => "reverse",
            8 => "hidden",
            9 => "strikethrough",
            22 => "normal intensity",
            23 => "not italic",
            24 => "not underlined",
            25 => "not blinking",
            27 => "not reversed",
            28 => "not hidden",
            29 => "not strikethrough",
            30 => "black fg",
            31 => "red fg",
            32 => "green fg",
            33 => "yellow fg",
            34 => "blue fg",
            35 => "magenta fg",
            36 => "cyan fg",
            37 => "white fg",
            38 => {
                // Extended foreground color
                if params.len() > i + 2 && params[i + 1] == 5 {
                    let code = params[i + 2];
                    i += 2;
                    descriptions.push(format!("fg: color {code}"));
                    i += 1;
                    continue;
                } else if params.len() > i + 4 && params[i + 1] == 2 {
                    let r = params[i + 2];
                    let g = params[i + 3];
                    let b = params[i + 4];
                    i += 4;
                    descriptions.push(format!("fg: rgb({r}, {g}, {b})"));
                    i += 1;
                    continue;
                }
                "extended fg"
            }
            39 => "default fg",
            40 => "black bg",
            41 => "red bg",
            42 => "green bg",
            43 => "yellow bg",
            44 => "blue bg",
            45 => "magenta bg",
            46 => "cyan bg",
            47 => "white bg",
            48 => {
                // Extended background color
                if params.len() > i + 2 && params[i + 1] == 5 {
                    let code = params[i + 2];
                    i += 2;
                    descriptions.push(format!("bg: color {code}"));
                    i += 1;
                    continue;
                } else if params.len() > i + 4 && params[i + 1] == 2 {
                    let r = params[i + 2];
                    let g = params[i + 3];
                    let b = params[i + 4];
                    i += 4;
                    descriptions.push(format!("bg: rgb({r}, {g}, {b})"));
                    i += 1;
                    continue;
                }
                "extended bg"
            }
            49 => "default bg",
            90..=97 => {
                let colors = ["bright black", "bright red", "bright green", "bright yellow",
                              "bright blue", "bright magenta", "bright cyan", "bright white"];
                let color = colors[(params[i] - 90) as usize];
                descriptions.push(format!("{color} fg"));
                i += 1;
                continue;
            }
            100..=107 => {
                let colors = ["bright black", "bright red", "bright green", "bright yellow",
                              "bright blue", "bright magenta", "bright cyan", "bright white"];
                let color = colors[(params[i] - 100) as usize];
                descriptions.push(format!("{color} bg"));
                i += 1;
                continue;
            }
            _ => {
                descriptions.push(format!("code {}", params[i]));
                i += 1;
                continue;
            }
        };
        descriptions.push(desc.to_string());
        i += 1;
    }

    descriptions.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe_sgr() {
        assert_eq!(describe_sgr(&[0]), "reset");
        assert_eq!(describe_sgr(&[1]), "bold");
        assert_eq!(describe_sgr(&[31]), "red fg");
        assert_eq!(describe_sgr(&[1, 31]), "bold, red fg");
        assert_eq!(describe_sgr(&[38, 2, 255, 128, 0]), "fg: rgb(255, 128, 0)");
    }
}
