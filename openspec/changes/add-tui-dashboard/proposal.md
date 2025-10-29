## Why
Users want an interactive dashboard that makes multi-target marking and follow-up file operations achievable without memorising subcommand flags. A TUI lowers friction and aligns with expectations set by tools like `htop` and `fzf`.

## What Changes
- Add a TUI mode that launches when `fuga` runs without arguments and renders a dashboard with file browser, mark list, and status bar.
- Implement navigation, filtering, and mark toggling inside the dashboard by orchestrating existing CLI subcommands (`mark --add`, `mark --reset`, `copy`, `move`, `link`).
- Provide TUI affordances for quitting, showing help, and invoking copy/move/link actions that exit back to the shell and run against the caller's current directory.

## Impact
- Affected specs: `tui-dashboard`
- Affected code: `src/main.rs`, `src/ui/*`, new TUI module(s), command dispatch integration
