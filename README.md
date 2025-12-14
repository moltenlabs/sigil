<p align="center">
  <img src="https://raw.githubusercontent.com/moltenlabs/glyphs/main/.github/assets/banner.png" alt="Glyphs" width="100%" />
</p>

<h1 align="center">✨ Glyphs</h1>

<p align="center">
  <strong>Beautiful ANSI escape sequences for Rust.</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/glyphs"><img src="https://img.shields.io/crates/v/glyphs.svg?style=flat-square&logo=rust" alt="Crates.io"></a>
  <a href="https://docs.rs/glyphs"><img src="https://img.shields.io/docsrs/glyphs?style=flat-square&logo=docs.rs" alt="Documentation"></a>
  <a href="https://github.com/moltenlabs/glyphs/actions"><img src="https://img.shields.io/github/actions/workflow/status/moltenlabs/glyphs/ci.yml?style=flat-square&logo=github" alt="CI"></a>
  <a href="#license"><img src="https://img.shields.io/badge/license-MIT%2FApache--2.0-blue?style=flat-square" alt="License"></a>
</p>

<p align="center">
  <a href="#features">Features</a> •
  <a href="#installation">Installation</a> •
  <a href="#quick-start">Quick Start</a> •
  <a href="#parsing">Parsing</a> •
  <a href="#ecosystem">Ecosystem</a>
</p>

---

## What is Glyphs?

**Glyphs** is a Rust library for working with ANSI escape sequences. It lets you:

1. **Style terminal output** with a fluent, type-safe API
2. **Parse existing sequences** into human-readable descriptions
3. **Strip ANSI codes** from strings
4. **Control cursors and screens** with named constants

Think of it as the Rust equivalent of [sequin](https://github.com/charmbracelet/sequin) from Charmbracelet.

```rust
use glyphs::{style, Color, parse};

// Style text beautifully
let output = style("Hello, Terminal!")
    .fg(Color::rgb(249, 115, 22))  // Molten Orange
    .bold()
    .to_string();

println!("{}", output);

// Parse mystery sequences
for segment in parse("\x1b[1;31mError\x1b[0m") {
    println!("{:?}", segment);
    // Escape: [SGR (style)] bold, red fg
    // Text: "Error"
    // Escape: [SGR (style)] reset
}
```

---

## Features

<table>
<tr>
<td width="50%">

### 🎨 Fluent Styling API
```rust
style("text")
    .fg(Color::Red)
    .bg(Color::Black)
    .bold()
    .italic()
    .underline()
```

</td>
<td width="50%">

### 🔍 Sequence Parsing
```rust
parse("\x1b[31mred\x1b[0m")
// → [Escape(red fg), Text("red"), Escape(reset)]
```

</td>
</tr>
<tr>
<td width="50%">

### 🌈 Full Color Support
- Standard 16 colors
- 256-color palette
- True color (24-bit RGB)
- Hex color parsing

</td>
<td width="50%">

### 🧹 String Utilities
```rust
strip_ansi("\x1b[31mHello\x1b[0m")  // "Hello"
visible_len("\x1b[31mHello\x1b[0m") // 5
```

</td>
</tr>
</table>

---

## Installation

```bash
cargo add glyphs
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
glyphs = "0.1"
```

### With Molten Brand Colors

```toml
[dependencies]
glyphs = { version = "0.1", features = ["brand"] }
```

---

## Quick Start

### Basic Styling

```rust
use glyphs::{style, Color};

// Simple colors
let red = style("Error!").fg(Color::Red).to_string();
let warning = style("Warning").fg(Color::Yellow).bold().to_string();

// RGB colors
let custom = style("Custom")
    .fg(Color::rgb(255, 128, 0))
    .to_string();

// Hex colors
let hex = style("Hex")
    .fg(Color::from_hex("#F97316"))
    .to_string();

// 256-color palette
let palette = style("Palette")
    .fg(Color::ansi256(208))
    .to_string();
```

### Text Modifiers

```rust
use glyphs::{style, Color};

let styled = style("Important")
    .bold()           // Bold text
    .italic()         // Italic
    .underline()      // Underlined
    .strikethrough()  // Strikethrough
    .dim()            // Dimmed
    .reverse()        // Inverted colors
    .blink()          // Blinking (if supported)
    .to_string();
```

### Combining Styles

```rust
use glyphs::{style, Color};

let fancy = style("🔥 Molten Labs")
    .fg(Color::rgb(249, 115, 22))
    .bold()
    .underline()
    .to_string();

println!("{}", fancy);
```

---

## Parsing

### Parse ANSI Sequences

Turn cryptic escape codes into readable descriptions:

```rust
use glyphs::{parse, ParsedSequence};

let input = "\x1b[1;38;2;249;115;22mMolten\x1b[0m";

for segment in parse(input) {
    match segment {
        ParsedSequence::Text(text) => {
            println!("Text: {}", text);
        }
        ParsedSequence::Escape(escape) => {
            println!("Escape: {}", escape.human_readable());
            // "bold, fg: rgb(249, 115, 22)"
        }
    }
}
```

### Strip ANSI Codes

```rust
use glyphs::strip_ansi;

let styled = "\x1b[1;31mBold Red\x1b[0m Normal";
let plain = strip_ansi(styled);
assert_eq!(plain, "Bold Red Normal");
```

### Get Visible Length

```rust
use glyphs::visible_len;

let styled = "\x1b[31mHello\x1b[0m";
assert_eq!(visible_len(styled), 5);  // Not 14!
```

---

## Cursor & Screen Control

### Cursor Movement

```rust
use glyphs::cursor;

// Move cursor
print!("{}", cursor::up(5));      // Move up 5 lines
print!("{}", cursor::down(3));    // Move down 3 lines
print!("{}", cursor::left(10));   // Move left 10 columns
print!("{}", cursor::right(10));  // Move right 10 columns

// Position cursor
print!("{}", cursor::goto(10, 20));  // Row 10, Column 20
print!("{}", cursor::column(1));      // Start of line
```

### Screen Control

```rust
use glyphs::sequences;

// Clear screen
print!("{}", sequences::CLEAR_SCREEN);
print!("{}", sequences::CLEAR_LINE);

// Cursor visibility
print!("{}", sequences::CURSOR_HIDE);
// ... do stuff ...
print!("{}", sequences::CURSOR_SHOW);

// Alternate screen buffer (for TUIs)
print!("{}", sequences::ALT_SCREEN_ENTER);
// ... your TUI ...
print!("{}", sequences::ALT_SCREEN_EXIT);
```

---

## Color Reference

### Standard Colors

| Color | Code | Bright Variant |
|-------|------|----------------|
| `Color::Black` | 30 | `Color::BrightBlack` |
| `Color::Red` | 31 | `Color::BrightRed` |
| `Color::Green` | 32 | `Color::BrightGreen` |
| `Color::Yellow` | 33 | `Color::BrightYellow` |
| `Color::Blue` | 34 | `Color::BrightBlue` |
| `Color::Magenta` | 35 | `Color::BrightMagenta` |
| `Color::Cyan` | 36 | `Color::BrightCyan` |
| `Color::White` | 37 | `Color::BrightWhite` |

### RGB & Hex

```rust
Color::rgb(255, 128, 0)      // Orange
Color::from_hex("#F97316")   // Molten Orange
Color::from_hex("7C3AED")    // Goblin Purple (# optional)
```

### 256-Color Palette

```rust
Color::ansi256(0)     // Black
Color::ansi256(196)   // Bright red
Color::ansi256(208)   // Orange
Color::ansi256(255)   // White
```

---

## Modifier Reference

| Modifier | ANSI Code | Description |
|----------|-----------|-------------|
| `.bold()` | 1 | **Bold text** |
| `.dim()` | 2 | Dimmed text |
| `.italic()` | 3 | *Italic text* |
| `.underline()` | 4 | <u>Underlined</u> |
| `.blink()` | 5 | Blinking text |
| `.reverse()` | 7 | Inverted colors |
| `.hidden()` | 8 | Hidden text |
| `.strikethrough()` | 9 | ~~Strikethrough~~ |

---

## Ecosystem

Glyphs is part of the **Molten Labs** open source ecosystem:

| Crate | Description | Status |
|-------|-------------|--------|
| **[molten_brand](https://crates.io/crates/molten_brand)** | Design tokens & colors | ✅ Published |
| **[glyphs](https://crates.io/crates/glyphs)** | ANSI sequences (you are here) | ✅ Published |
| **[lacquer](https://crates.io/crates/lacquer)** | Terminal styling (like lipgloss) | ✅ Published |
| **cauldron** | TUI framework (like bubbletea) | 📋 Planned |

---

## Why "Glyphs"?

A **glyph** is a symbolic mark or character—like the hidden ANSI codes that transform plain text into beautiful terminal output. In the forge, we inscribe glyphs to add magic to our creations. ✨

---

## Performance

- **Zero allocations** for static styling
- **Compile-time** color constants
- **Efficient parsing** with minimal copies
- Suitable for hot paths and TUI rendering loops

---

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

```bash
git clone https://github.com/moltenlabs/glyphs
cd glyphs
cargo test
```

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

<p align="center">
  <sub>Built with ✨ by <a href="https://github.com/moltenlabs">Molten Labs</a></sub>
</p>

<p align="center">
  <sub><i>"Let them cook."</i></sub>
</p>
