# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

FUGA is a Rust CLI tool for two-step file operations - mark files/directories first, then perform copy/move/link operations from another location. It's designed as an alternative to traditional `mv`, `cp`, and `ln` commands.

## Build and Development Commands

### Building and Running
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the application
cargo run -- <command>

# Install from source
cargo install --path .

# Run tests
cargo test
```

### Development Tools
```bash
# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run clippy lints
cargo clippy

# Check for unused dependencies
cargo machete
```

## Architecture

### Core Components

- **main.rs**: CLI interface using clap for command parsing with subcommands (mark, copy, move, link, completion, version)
- **fuga.rs**: Core business logic containing file operations, configuration management, and utility functions

### Key Design Patterns

1. **Two-Step Operations**: The tool stores marked file paths in a config file (~/.config/fuga/fuga.toml) and performs operations later
2. **Configuration Management**: Uses `confy` crate for TOML-based configuration with automatic path resolution
3. **Progress Tracking**: Implements progress bars using `indicatif` for long-running file operations
4. **Error Handling**: Comprehensive error handling with user-friendly error messages and emojis

### File Operation Flow

1. **Mark Phase**: `fuga mark <path>` stores absolute path in config
2. **Operation Phase**: `fuga copy/move/link [destination]` performs the operation
3. **State Management**: Move operations automatically reset the mark; copy/link operations preserve it

### Dependencies

- `clap` (4.5.4): Command-line argument parsing with derive macros
- `fs_extra` (1.3.0): Enhanced file operations with progress tracking
- `confy` (1.0.0): Configuration file management
- `indicatif` (0.17.8): Progress bars and spinners
- `termion` (4.0.0): Terminal formatting and colors
- `emojis` (0.7.0): Emoji display for user feedback

## Testing

The project currently has minimal test coverage. When adding tests:
```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture
```

## Configuration

The tool automatically creates configuration at `~/.config/fuga/fuga.toml` with:
- `user_config.box_path`: Path to config directory
- `data.target`: Currently marked file/directory path