//! Sequence building utilities.

use std::fmt;

/// A builder for constructing ANSI escape sequences.
#[derive(Debug, Clone, Default)]
pub struct SequenceBuilder {
    sequences: Vec<String>,
}

impl SequenceBuilder {
    /// Create a new sequence builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a raw sequence.
    #[must_use]
    pub fn raw(mut self, seq: &str) -> Self {
        self.sequences.push(seq.to_string());
        self
    }

    /// Add a CSI sequence with the given parameters and final character.
    #[must_use]
    pub fn csi(mut self, params: &[u16], final_char: char) -> Self {
        let params_str = params
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(";");
        self.sequences.push(format!("\x1b[{params_str}{final_char}"));
        self
    }

    /// Add an SGR (style) sequence.
    #[must_use]
    pub fn sgr(self, params: &[u16]) -> Self {
        self.csi(params, 'm')
    }

    /// Add a reset sequence.
    #[must_use]
    pub fn reset(self) -> Self {
        self.sgr(&[0])
    }

    /// Clear the screen.
    #[must_use]
    pub fn clear_screen(mut self) -> Self {
        self.sequences.push("\x1b[2J".to_string());
        self
    }

    /// Move cursor to position.
    #[must_use]
    pub fn cursor_to(mut self, row: u16, col: u16) -> Self {
        self.sequences.push(format!("\x1b[{row};{col}H"));
        self
    }

    /// Build the final sequence string.
    #[must_use]
    pub fn build(self) -> String {
        self.sequences.join("")
    }
}

impl fmt::Display for SequenceBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone().build())
    }
}

/// A complete ANSI sequence that can be written to a terminal.
#[derive(Debug, Clone)]
pub struct Sequence {
    data: String,
}

impl Sequence {
    /// Create from a raw string.
    #[must_use]
    pub fn new(data: String) -> Self {
        Self { data }
    }

    /// Create from a builder.
    #[must_use]
    pub fn from_builder(builder: SequenceBuilder) -> Self {
        Self {
            data: builder.build(),
        }
    }

    /// Get the raw sequence data.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.data
    }

    /// Get the byte length of the sequence.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if the sequence is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl AsRef<str> for Sequence {
    fn as_ref(&self) -> &str {
        &self.data
    }
}

impl From<SequenceBuilder> for Sequence {
    fn from(builder: SequenceBuilder) -> Self {
        Self::from_builder(builder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequence_builder() {
        let seq = SequenceBuilder::new()
            .sgr(&[1, 31])
            .build();
        
        assert_eq!(seq, "\x1b[1;31m");
    }

    #[test]
    fn test_sequence_builder_chain() {
        let seq = SequenceBuilder::new()
            .clear_screen()
            .cursor_to(1, 1)
            .sgr(&[32])
            .build();
        
        assert!(seq.contains("\x1b[2J"));
        assert!(seq.contains("\x1b[1;1H"));
        assert!(seq.contains("\x1b[32m"));
    }
}
