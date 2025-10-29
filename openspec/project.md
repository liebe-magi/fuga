# Project Context

## Purpose
`fuga` is a cross-platform Rust CLI that streamlines copy, move, and symlink workflows by splitting them into two deliberate steps: mark a source once, then execute the operation from the destination. The goal is to reduce shell typos, keep file operations predictable, and provide helpful terminal feedback.

## Tech Stack
- Rust 1.78+ (edition 2021) with Cargo for builds, testing, and distribution
- clap 4.x for argument parsing plus clap_complete for shell completions
- confy + serde for persisting application state in the user config directory
- fs_extra and indicatif for long-running filesystem operations with progress bars
- termion and emojis crate for terminal-friendly UI feedback

## Project Conventions

### Code Style
- Enforce `cargo fmt` with default Rustfmt settings and `cargo clippy -- -D warnings` before committing (mirrored in CI and `.pre-commit-config.yaml`).
- Modules and functions follow `snake_case`; types and traits use `UpperCamelCase`; constants use `SCREAMING_SNAKE_CASE`.
- Prefer expressive error variants in `error/` and surface user-facing text via the UI service to keep command code lean.

### Architecture Patterns
- Command pattern: each subcommand implements `commands::Command` and receives only the services it needs.
- Trait-based dependency injection: `traits.rs` defines service interfaces backed by `services::*` implementations, assembled in `main.rs::ServiceContainer`.
- Config access flows through `ConfigRepository` (currently `FileConfigRepository`) so alternative storage backends remain possible.
- UI concerns stay inside `ui::TerminalUIService`, keeping emojis, colors, and formatting isolated from business logic.

### Testing Strategy
- Primary automation uses `cargo test`; unit coverage exists today for logic that can run without touching the real filesystem (e.g., completion command).
- CI (`.github/workflows/build.yml`) runs `cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, and `cargo test` on every PR and pushes to `main`.
- Manual testing is still expected for end-to-end filesystem flows; prefer adding focused unit tests when bugs are fixed or new command behaviors are introduced.

### Git Workflow
- Default branch is `develop`; cut feature branches from it and open pull requests back into `develop`.
- Merge from `develop` into `main` when preparing releases—GitHub Actions release job triggers on pushes to `main` and publishes multi-target binaries.
- Keep commits small, imperative, and scoped to one logical change; rebase local branches before merging to maintain a linear history.

## Domain Context
- Users mark a single source path that persists via `confy`; subsequent `copy`, `move`, or `link` commands operate relative to the current working directory or an explicit destination argument.
- Operations must tolerate both files and directories, emitting clear emoji-prefixed status lines, and should no-op gracefully if the mark is missing.
- The tool targets macOS and Linux primarily, with Windows support in progress (link creation uses platform-specific APIs when available).

## Important Constraints
- Never perform destructive operations when source and destination resolve to the same absolute path—commands return `DuplicatePath` errors instead.
- All filesystem paths should be resolved to absolute paths before acting to avoid surprises from relative directories.
- Keep output readable on non-emoji terminals; fallback glyphs should remain ASCII-friendly.
- Avoid introducing dependencies that break the lightweight single-binary distribution story.

## External Dependencies
- GitHub Actions (`.github/workflows/build.yml`) handles CI, cross-compilation, and draft release creation.
- `confy` writes configuration under the OS-specific config directory discovered via the `dirs` crate.
- `fs_extra` performs recursive copy/move with progress callbacks; ensure its API remains stable before upgrading.
- `indicatif` drives progress bars and must be kept in sync with terminal capabilities expected by `termion`.
