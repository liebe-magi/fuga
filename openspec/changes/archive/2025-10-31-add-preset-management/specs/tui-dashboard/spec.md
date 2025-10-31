## ADDED Requirements
### Requirement: Dashboard Preset Management
The dashboard TUI SHALL expose preset list, load, save, overwrite, and delete flows that mirror the `fuga preset` subcommands while keeping the mark pane in sync.

#### Scenario: Load preset from dashboard popup
- **GIVEN** presets `template` and `logs` exist in configuration
- **WHEN** the user presses `P`
- **AND** selects `template` from the popup list (optionally filtered via `/` search)
- **THEN** the TUI SHALL invoke the preset load operation for `template`
- **AND** the mark list pane SHALL refresh to show the preset targets immediately

#### Scenario: Create new preset from dashboard
- **GIVEN** `[data].targets` currently tracks absolute paths
- **WHEN** the user presses `S`
- **AND** chooses `[ Create New Preset... ]`
- **AND** enters the name `daily`
- **THEN** the TUI SHALL invoke the preset save operation with name `daily`
- **AND** the preset list SHALL include `daily` after the save completes

#### Scenario: Overwrite existing preset from dashboard
- **GIVEN** preset `template` exists with prior contents
- **WHEN** the user presses `S`
- **AND** selects `template`
- **AND** confirms the overwrite prompt with `y`
- **THEN** the TUI SHALL invoke the preset save operation for `template`
- **AND** the overwrite SHALL persist the current mark list under that preset

#### Scenario: Delete preset from dashboard
- **GIVEN** preset `logs` exists
- **WHEN** the user opens the preset list via `P`
- **AND** highlights `logs`
- **AND** presses `D`
- **AND** confirms with `y`
- **THEN** the TUI SHALL invoke the preset delete operation for `logs`
- **AND** the popup list SHALL remove `logs` without restarting the dashboard
