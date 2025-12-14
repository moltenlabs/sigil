//! ANSI sequence parser.

use crate::escape::{describe_sgr, Escape, EscapeKind};

/// A parsed segment of text (either plain text or an escape sequence).
#[derive(Debug, Clone)]
pub enum ParsedSequence {
    /// Plain text without escape sequences.
    Text(String),
    /// An escape sequence.
    Escape(Escape),
}

impl ParsedSequence {
    /// Get as text, or None if this is an escape.
    #[must_use]
    pub fn as_text(&self) -> Option<&str> {
        match self {
            Self::Text(t) => Some(t),
            Self::Escape(_) => None,
        }
    }

    /// Get as escape, or None if this is text.
    #[must_use]
    pub const fn as_escape(&self) -> Option<&Escape> {
        match self {
            Self::Text(_) => None,
            Self::Escape(e) => Some(e),
        }
    }
}

/// Parse a string containing ANSI escape sequences.
///
/// Returns a vector of parsed segments, alternating between plain text and escapes.
///
/// # Example
///
/// ```rust
/// use glyphs::parse;
///
/// let segments = parse("\x1b[31mRed\x1b[0m text");
/// for segment in segments {
///     match segment {
///         glyphs::ParsedSequence::Text(t) => println!("Text: {}", t),
///         glyphs::ParsedSequence::Escape(e) => println!("Escape: {}", e),
///     }
/// }
/// ```
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn parse(input: &str) -> Vec<ParsedSequence> {
    let mut result = Vec::new();
    let mut current_text = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Flush any accumulated text
            if !current_text.is_empty() {
                result.push(ParsedSequence::Text(std::mem::take(&mut current_text)));
            }

            // Parse escape sequence
            let mut seq = String::from('\x1b');
            
            if let Some(&next) = chars.peek() {
                seq.push(chars.next().unwrap());

                match next {
                    '[' => {
                        // CSI sequence
                        let mut params = String::new();
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_digit() || c == ';' {
                                params.push(chars.next().unwrap());
                                seq.push(c);
                            } else {
                                break;
                            }
                        }

                        // Get final character
                        if let Some(final_char) = chars.next() {
                            seq.push(final_char);
                            let escape = parse_csi(&params, final_char, &seq);
                            result.push(ParsedSequence::Escape(escape));
                        }
                    }
                    ']' => {
                        // OSC sequence
                        while let Some(&c) = chars.peek() {
                            seq.push(chars.next().unwrap());
                            if c == '\x07' || (c == '\\' && seq.ends_with('\x1b')) {
                                break;
                            }
                        }
                        let escape = Escape::new(
                            seq,
                            EscapeKind::Osc,
                            "operating system command".to_string(),
                        );
                        result.push(ParsedSequence::Escape(escape));
                    }
                    _ => {
                        // Unknown escape
                        let escape = Escape::new(seq, EscapeKind::Unknown, "unknown".to_string());
                        result.push(ParsedSequence::Escape(escape));
                    }
                }
            }
        } else {
            current_text.push(c);
        }
    }

    // Flush remaining text
    if !current_text.is_empty() {
        result.push(ParsedSequence::Text(current_text));
    }

    result
}

fn parse_csi(params: &str, final_char: char, raw: &str) -> Escape {
    let param_values: Vec<u16> = params
        .split(';')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();

    let (kind, description) = match final_char {
        'm' => (EscapeKind::Sgr, describe_sgr(&param_values)),
        'A' => (EscapeKind::Cursor, format!("cursor up {}", param_values.first().unwrap_or(&1))),
        'B' => (EscapeKind::Cursor, format!("cursor down {}", param_values.first().unwrap_or(&1))),
        'C' => (EscapeKind::Cursor, format!("cursor right {}", param_values.first().unwrap_or(&1))),
        'D' => (EscapeKind::Cursor, format!("cursor left {}", param_values.first().unwrap_or(&1))),
        'H' | 'f' => {
            let row = param_values.first().unwrap_or(&1);
            let col = param_values.get(1).unwrap_or(&1);
            (EscapeKind::Cursor, format!("cursor to ({row}, {col})"))
        }
        'J' => {
            let mode = param_values.first().unwrap_or(&0);
            let desc = match mode {
                0 => "clear to end of screen",
                1 => "clear to start of screen",
                2 => "clear entire screen",
                3 => "clear screen and scrollback",
                _ => "clear screen (unknown mode)",
            };
            (EscapeKind::Erase, desc.to_string())
        }
        'K' => {
            let mode = param_values.first().unwrap_or(&0);
            let desc = match mode {
                0 => "clear to end of line",
                1 => "clear to start of line",
                2 => "clear entire line",
                _ => "clear line (unknown mode)",
            };
            (EscapeKind::Erase, desc.to_string())
        }
        'h' | 'l' => {
            let enabled = final_char == 'h';
            let action = if enabled { "enable" } else { "disable" };
            if params.starts_with('?') {
                let mode = params.trim_start_matches('?');
                let desc = match mode {
                    "25" => format!("{action} cursor visibility"),
                    "1049" => format!("{action} alternate screen"),
                    "1000" => format!("{action} mouse tracking"),
                    "2004" => format!("{action} bracketed paste"),
                    _ => format!("{action} mode {mode}"),
                };
                (EscapeKind::Mode, desc)
            } else {
                (EscapeKind::Mode, format!("{action} mode {params}"))
            }
        }
        's' => (EscapeKind::Cursor, "save cursor position".to_string()),
        'u' => (EscapeKind::Cursor, "restore cursor position".to_string()),
        _ => (EscapeKind::Unknown, format!("CSI sequence ending with '{final_char}'")),
    };

    Escape::new(raw.to_string(), kind, description).with_params(param_values)
}

/// Strip all ANSI escape sequences from a string.
#[must_use]
pub fn strip_ansi(input: &str) -> String {
    parse(input)
        .iter()
        .filter_map(|seg| seg.as_text())
        .collect()
}

/// Get the visible length of a string (excluding ANSI codes).
#[must_use]
pub fn visible_len(input: &str) -> usize {
    strip_ansi(input).chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let segments = parse("\x1b[31mRed\x1b[0m");
        assert_eq!(segments.len(), 3);
        
        assert!(matches!(&segments[0], ParsedSequence::Escape(e) if e.kind == EscapeKind::Sgr));
        assert!(matches!(&segments[1], ParsedSequence::Text(t) if t == "Red"));
        assert!(matches!(&segments[2], ParsedSequence::Escape(e) if e.kind == EscapeKind::Sgr));
    }

    #[test]
    fn test_strip_ansi() {
        let stripped = strip_ansi("\x1b[1;31mBold Red\x1b[0m Text");
        assert_eq!(stripped, "Bold Red Text");
    }

    #[test]
    fn test_visible_len() {
        let len = visible_len("\x1b[31mHello\x1b[0m");
        assert_eq!(len, 5);
    }
}
