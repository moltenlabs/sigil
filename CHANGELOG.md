# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2024-12-14

### Added

- Initial release
- `Color` enum with ANSI, 256-color, and RGB support
- `Modifier` enum for text styling (bold, italic, etc.)
- `style()` function for fluent styling API
- `parse()` function to parse ANSI sequences
- `strip_ansi()` to remove ANSI codes
- `visible_len()` to get string length without ANSI codes
- Cursor movement helpers (`cursor::up`, `cursor::goto`, etc.)
- Common sequences (`sequences::CLEAR_SCREEN`, etc.)
- Optional `brand` feature for Molten brand colors
- Human-readable descriptions for parsed sequences
