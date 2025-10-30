## Why
Repeated mark configurations currently require manual re-entry, slowing workflows that reuse the same file sets for templating, log collection, or deliveries. Persisting named presets would let users switch contexts quickly from both the CLI and dashboard TUI.

## What Changes
- Add a `[presets]` table to `fuga.toml` for storing absolute mark lists keyed by preset name.
- Introduce `fuga preset` subcommands to save, load, list, show, and delete presets backed by the new configuration table.
- Extend the dashboard TUI to load, save, and delete presets through interactive popups with filtering and confirmation flows.

## Impact
- Affected specs: `cli-foundations`, `tui-dashboard`
- Affected code: CLI preset command handlers, configuration persistence, dashboard preset UI components
