# Sigil ✨

> Human-readable ANSI escape sequences for terminal styling.

[![Crates.io](https://img.shields.io/crates/v/sigil.svg)](https://crates.io/crates/sigil)
[![Documentation](https://docs.rs/sigil/badge.svg)](https://docs.rs/sigil)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)

Sigil is a Rust library for working with ANSI escape sequences in a type-safe, human-readable way. It's the Rust equivalent of [sequin](https://github.com/charmbracelet/sequin) from Charmbracelet.

## Features

- 🎨 **Type-safe colors** - RGB, 256-color, and standard ANSI colors
- ✍️ **Text modifiers** - Bold, italic, underline, and more
- 🔍 **Parse sequences** - Convert raw ANSI codes to readable descriptions
- 🧹 **Strip sequences** - Remove ANSI codes from strings
- 🔥 **Brand integration** - Optional Molten brand colors

## Installation

```bash
cargo add sigil
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
sigil = "0.1"
```

## Quick Start

### Styling Text

```rust
use sigil::{style, Color};

// Basic styling
let red = style("Error!").fg(Color::Red).bold().to_string();
println!("{}", red);

// RGB colors
let orange = style("Molten")
    .fg(Color::rgb(249, 115, 22))
    .to_string();

// Hex colors
let purple = style("Goblin")
    .fg(Color::from_hex("#7C3AED"))
    .bold()
    .to_string();

// Multiple modifiers
let fancy = style("Important")
    .fg(Color::Yellow)
    .bg(Color::Blue)
    .bold()
    .underline()
    .to_string();
```

### Parsing ANSI Sequences

```rust
use sigil::{parse, ParsedSequence};

let input = "\x1b[1;31mError:\x1b[0m Something went wrong";

for segment in parse(input) {
    match segment {
        ParsedSequence::Text(text) => {
            println!("Text: {}", text);
        }
        ParsedSequence::Escape(escape) => {
            println!("Escape: {}", escape.human_readable());
            // Outputs: "[SGR (style)] bold, red fg"
        }
    }
}
```

### Stripping ANSI Codes

```rust
use sigil::parser::{strip_ansi, visible_len};

let styled = "\x1b[1;31mHello\x1b[0m World";

let plain = strip_ansi(styled);
assert_eq!(plain, "Hello World");

let len = visible_len(styled);
assert_eq!(len, 11); // "Hello World".len()
```

### Cursor Movement

```rust
use sigil::cursor;

// Move cursor
print!("{}", cursor::up(5));
print!("{}", cursor::goto(10, 20));

// Save/restore position
print!("{}", sigil::sequences::CURSOR_SAVE);
// ... do stuff ...
print!("{}", sigil::sequences::CURSOR_RESTORE);
```

### Screen Control

```rust
use sigil::sequences;

// Clear screen
print!("{}", sequences::CLEAR_SCREEN);

// Alternate screen buffer
print!("{}", sequences::ALT_SCREEN_ENTER);
// ... your TUI ...
print!("{}", sequences::ALT_SCREEN_EXIT);
```

## With Molten Brand Colors

Enable the `brand` feature for Molten Labs brand colors:

```toml
[dependencies]
sigil = { version = "0.1", features = ["brand"] }
```

```rust
use sigil::{style, color::brand};

let molten = style("Molten Labs").fg(brand::MOLTEN).bold();
let goblin = style("Goblins").fg(brand::GOBLIN);
let success = style("Success!").fg(brand::SUCCESS);
```

## Color Types

### Standard ANSI Colors

```rust
use sigil::Color;

Color::Black
Color::Red
Color::Green
Color::Yellow
Color::Blue
Color::Magenta
Color::Cyan
Color::White

// Bright variants
Color::BrightRed
// ... etc
```

### 256-Color Palette

```rust
use sigil::Color;

let color = Color::ansi256(42);
```

### True Color (RGB)

```rust
use sigil::Color;

let rgb = Color::rgb(255, 128, 0);
let hex = Color::from_hex("#FF8000");
```

## Modifiers

```rust
use sigil::Modifier;

Modifier::Bold
Modifier::Dim
Modifier::Italic
Modifier::Underline
Modifier::Blink
Modifier::Reverse
Modifier::Hidden
Modifier::Strikethrough
Modifier::DoubleUnderline
Modifier::Overline
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

---

*Part of the [Molten Labs](https://github.com/moltenlabs) ecosystem. "Let them cook." 🔥*
