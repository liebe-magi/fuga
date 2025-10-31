## ADDED Requirements
### Requirement: Preset Mark Management CLI
The CLI SHALL provide `fuga preset` subcommands that persist and restore named mark lists using absolute target paths in the configuration file.

#### Scenario: Save current marks into preset
- **GIVEN** `[data].targets` contains absolute paths `[/abs/A, /abs/B]`
- **WHEN** the user runs `fuga preset save template`
- **THEN** the CLI SHALL write `[presets].template` equal to `[/abs/A, /abs/B]`
- **AND** the command SHALL confirm the preset name and number of targets saved

#### Scenario: Load preset into mark list
- **GIVEN** `[presets].template` contains absolute paths `[/abs/A, /abs/B]`
- **WHEN** the user runs `fuga preset load template`
- **THEN** the CLI SHALL replace `[data].targets` with `[/abs/A, /abs/B]`
- **AND** the command SHALL report the preset name and target count now tracked

#### Scenario: List available presets
- **GIVEN** the configuration contains presets `template` and `collect`
- **WHEN** the user runs `fuga preset list`
- **THEN** the CLI SHALL print each preset name on its own line beneath a heading indicating saved presets

#### Scenario: Show preset contents
- **GIVEN** `[presets].template` contains absolute paths
- **WHEN** the user runs `fuga preset show template`
- **THEN** the CLI SHALL print each stored path with clear formatting so users can review the preset members

#### Scenario: Delete named preset
- **GIVEN** `[presets].template` exists
- **WHEN** the user runs `fuga preset delete template`
- **THEN** the CLI SHALL remove the `template` key from `[presets]`
- **AND** the command SHALL confirm deletion of the preset
