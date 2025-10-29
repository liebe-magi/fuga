# tui-dashboard Specification

## Purpose
TBD - created by archiving change add-tui-dashboard. Update Purpose after archive.
## Requirements
### Requirement: Launch TUI Without Arguments
The CLI SHALL start the dashboard TUI whenever `fuga` executes without subcommand arguments, falling back to existing CLI parsing when arguments are present.

#### Scenario: Invoke fuga with no arguments
- **WHEN** the user runs `fuga` from the shell without any flags or subcommands
- **THEN** the process SHALL initialise the dashboard TUI
- **AND** the TUI SHALL remain active until the user exits via a dashboard action

### Requirement: Dashboard Layout Components
The dashboard TUI SHALL render a primary file browser pane, a mark list pane sourced from `[data].targets`, and a status bar that advertises essential key bindings.

#### Scenario: Render core panes
- **GIVEN** the configured mark list contains at least one entry
- **WHEN** the TUI launches
- **THEN** the main pane SHALL display the current working directory contents with a header showing the absolute path
- **AND** the mark list pane SHALL list the absolute targets from configuration
- **AND** the status bar SHALL display shortcuts such as `q`, `m`, `c`, `v`, `s`

#### Scenario: Balanced pane widths
- **WHEN** the TUI renders the file browser and mark list panes side-by-side
- **THEN** the file browser pane SHALL consume approximately 70% of the horizontal space
- **AND** the mark list pane SHALL consume the remaining ~30% so long paths remain readable

#### Scenario: Alternate mark list styling
- **GIVEN** the mark list contains more than one entry
- **WHEN** the TUI renders the pane
- **THEN** odd and even rows SHALL use contrasting background fills so adjacent items remain visually distinct even on dark terminals
- **AND** the darker band SHALL reuse the file browser cursor colour to keep the palette consistent

### Requirement: File Browser Navigation and Filtering
The file browser pane SHALL support cursor navigation, directory traversal, hidden file toggling, and incremental filtering triggered by `/` input.

#### Scenario: Navigate directories
- **WHEN** the user presses `↓`/`j` or `↑`/`k`
- **THEN** the selection cursor SHALL move accordingly
- **AND** pressing `→`/`l`/`Enter` on a directory SHALL change the browsed directory to that child
- **AND** pressing `←`/`h`/`Backspace` SHALL return to the parent directory

#### Scenario: Toggle hidden entries
- **GIVEN** the current directory contains files beginning with `.`
- **WHEN** the user presses `Ctrl+h` or `.`
- **THEN** the file list SHALL toggle between including and hiding dot-prefixed entries

#### Scenario: Filter by query
- **WHEN** the user presses `/`
- **THEN** the TUI SHALL enter filtering mode
- **AND** as the user types, the file list SHALL update incrementally using fuzzy matching semantics similar to `fzf`

#### Scenario: Clear active filter
- **GIVEN** a filter query is currently limiting the file list
- **WHEN** the user presses `Ctrl+l`
- **THEN** the filter input SHALL be cleared
- **AND** the file browser SHALL revert to showing all visible entries

### Requirement: Mark Management Integration
The TUI SHALL orchestrate existing `fuga mark` flows to add, remove, and reset marks while keeping the mark list pane in sync with configuration.

#### Scenario: Toggle mark on untracked item
- **GIVEN** the highlighted path is not present in `[data].targets`
- **WHEN** the user presses `Space` or `m`
- **THEN** the TUI SHALL invoke `fuga mark --add <path>`
- **AND** the mark list pane SHALL refresh to include the new absolute path

#### Scenario: Toggle mark off tracked item
- **GIVEN** the highlighted path already exists in `[data].targets`
- **WHEN** the user presses `Space` or `m`
- **THEN** the TUI SHALL issue `fuga mark --reset`
- **AND** the TUI SHALL reapply the remaining marks by calling `fuga mark --add` with every target except the removed path
- **AND** the mark list pane SHALL refresh to exclude the removed path

#### Scenario: Reset mark list from dashboard
- **WHEN** the user presses `Ctrl+r` or `R`
- **THEN** the TUI SHALL prompt `"Reset marks? [y/N]"`
- **AND** on confirmation with `y` the TUI SHALL invoke `fuga mark --reset`
- **AND** the mark list pane SHALL display an empty state

### Requirement: Dashboard File Operations
The TUI SHALL exit and delegate to existing copy, move, and link commands using the dashboard's currently browsed directory as the implicit destination when no explicit path is provided.

#### Scenario: Copy marked targets into browsed directory
- **GIVEN** the dashboard is focused on `/tui/dest`
- **WHEN** the user presses `c`
- **THEN** the TUI SHALL terminate
- **AND** it SHALL run `fuga copy` targeting `/tui/dest`
- **AND** the copy command SHALL operate on the current mark list

#### Scenario: Move and link actions
- **WHEN** the user presses `v` or `s`
- **THEN** the TUI SHALL terminate
- **AND** it SHALL run `fuga move` or `fuga link` respectively using the same browsed directory path observed at exit
- **AND** the existing CLI commands SHALL process the mark list without additional prompts

### Requirement: Dashboard Exit and Help
The TUI SHALL provide consistent exit shortcuts and an in-app help overlay enumerating available key bindings.

#### Scenario: Quit without side effects
- **WHEN** the user presses `q` or `Ctrl+c`
- **THEN** the TUI SHALL terminate without invoking file operations or modifying marks

#### Scenario: Help overlay
- **WHEN** the user presses `?` or `F1`
- **THEN** the TUI SHALL display a help view listing all dashboard key bindings
- **AND** exiting the help view SHALL return to the dashboard without changing state

