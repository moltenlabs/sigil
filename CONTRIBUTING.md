# Contributing to Molten Labs Open Source

Thank you for your interest in contributing to Molten Labs projects! 🔥

## Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and constructive in all interactions.

## Ways to Contribute

### 🐛 Reporting Bugs

- Check existing issues first to avoid duplicates
- Use the bug report template
- Include reproduction steps, expected behavior, and actual behavior
- Include your Rust version (`rustc --version`) and OS

### 💡 Suggesting Features

- Open an issue with the feature request template
- Explain the use case and why it would be valuable
- Consider if it fits the project's goals

### 📝 Documentation

- Fix typos, clarify explanations
- Add examples
- Improve API documentation

### 🔧 Code Contributions

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test --all-features`)
5. Run lints (`cargo clippy`)
6. Format code (`cargo fmt`)
7. Commit with a descriptive message
8. Push and open a Pull Request

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Git

### Getting Started

```bash
# Clone the repo
git clone https://github.com/moltenlabs/molten.git
cd molten

# Build all crates
cargo build --workspace

# Run all tests
cargo test --workspace --all-features

# Check lints
cargo clippy --workspace --all-features

# Format code
cargo fmt --all
```

### Project Structure

```
molten/
├── crates/
│   ├── molten_brand/    # Design tokens (colors, typography, etc.)
│   ├── sigil/           # ANSI escape sequences
│   ├── lacquer/         # Terminal styling
│   ├── cauldron/        # TUI framework
│   └── ...
├── apps/
│   └── lair/            # Main terminal application
└── scripts/
    └── release-crate.sh # Release automation
```

## Commit Messages

We use conventional commits:

```
feat: add new color parsing utility
fix: correct RGB to hex conversion
docs: improve sigil README examples
chore: update dependencies
refactor: simplify style builder API
test: add tests for modifier combinations
```

## Pull Request Guidelines

- Keep PRs focused on a single change
- Include tests for new functionality
- Update documentation if needed
- Ensure all CI checks pass
- Request review from maintainers

## Code Style

- Follow `rustfmt` defaults
- Use `clippy` and address all warnings
- Write doc comments for public APIs
- Prefer explicit types over inference in public APIs
- Use meaningful variable names

## Testing

```bash
# Run all tests
cargo test --workspace --all-features

# Run tests for a specific crate
cargo test -p sigil --all-features

# Run tests with output
cargo test --workspace -- --nocapture
```

## Documentation

- Use `///` for public API documentation
- Include examples in doc comments
- Run `cargo doc --open` to preview

```rust
/// Create a styled string with the given text.
///
/// # Example
///
/// ```rust
/// use sigil::style;
///
/// let styled = style("Hello").bold().to_string();
/// ```
pub fn style(text: &str) -> Styled {
    // ...
}
```

## Release Process

Releases are handled by maintainers using:

```bash
./scripts/release-crate.sh <crate_name> <version> "Release message"
```

This triggers GitHub Actions to:
1. Sync to the individual repo
2. Publish to crates.io
3. Create a GitHub release

## Questions?

- Open a GitHub Discussion for general questions
- Tag issues with `question` for specific questions
- Join our Discord (coming soon)

## License

By contributing, you agree that your contributions will be licensed under the same MIT/Apache-2.0 dual license as the project.

---

*Thank you for helping make Molten Labs better! 🔥*
