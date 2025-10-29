## ADDED Requirements
### Requirement: Multi-Target Mark Configuration
The CLI SHALL persist mark state as a list of absolute filesystem paths and migrate legacy single-target configuration entries.

#### Scenario: Persist multiple absolute targets
- **WHEN** the user runs `fuga mark <path1> <path2>`
- **THEN** the configuration SHALL store `[data].targets` as the absolute forms of the provided paths in order

#### Scenario: Migrate legacy single target
- **GIVEN** the configuration file only contains `[data].target = "/old/path"`
- **WHEN** `fuga` loads mark state
- **THEN** the runtime SHALL surface `[data].targets` containing the canonical absolute path for `/old/path`
- **AND** if the configuration is writable the CLI SHALL persist the migrated array back to disk

### Requirement: Mark List Management Commands
`fuga mark` SHALL manage the mark list through overwrite, additive, reset, and list flows while avoiding duplicate entries.

#### Scenario: Overwrite mark list
- **WHEN** the user executes `fuga mark one two`
- **THEN** the stored `[data].targets` SHALL be replaced with only the absolute versions of `one` and `two`

#### Scenario: Add unique entries
- **GIVEN** `[data].targets` already contains `/abs/one`
- **WHEN** the user executes `fuga mark --add one two`
- **THEN** the CLI SHALL resolve both paths to absolute form
- **AND** the resulting list SHALL contain `/abs/one` and `/abs/two` with no duplicates

#### Scenario: Reset mark list
- **WHEN** the user executes `fuga mark --reset`
- **THEN** the stored `[data].targets` SHALL become an empty array

- **WHEN** the user executes `fuga mark --list`
- **THEN** the CLI SHALL print each target on its own line with status messaging
- **AND** if the list is empty the CLI SHALL state that no targets are marked

### Requirement: Batch Operations from Marks
File operation commands SHALL iterate the mark list, reject invalid destinations, and emit per-target feedback.

#### Scenario: Error on empty mark list
- **WHEN** the user executes `fuga copy` while `[data].targets` is empty
- **THEN** the command SHALL exit with a non-zero status
- **AND** the CLI SHALL state that no targets are marked

#### Scenario: Default destination is current directory
- **GIVEN** `[data].targets` contains absolute paths to `a` and `b`
- **WHEN** the user executes `fuga copy` without a destination argument
- **THEN** both targets SHALL be copied into the current working directory

#### Scenario: Copy into destination directory
- **WHEN** the user executes `fuga link dest/` and `dest/` resolves to an existing directory
- **THEN** each marked path SHALL be linked under `dest/` using the source file names

#### Scenario: Reject multiple targets to single file
- **GIVEN** `[data].targets` contains two entries
- **WHEN** the user executes `fuga move file.txt`
- **THEN** the CLI SHALL refuse the operation with a clear error explaining that multiple targets cannot map to a single destination

### Requirement: Move Clears Marks After Success
The CLI SHALL clear the mark list after successful move operations while preserving it on failure.

#### Scenario: Clear marks on successful move
- **GIVEN** `[data].targets` contains entries
- **WHEN** `fuga move dest/` completes without error
- **THEN** `[data].targets` SHALL be reset to an empty array

#### Scenario: Preserve marks on move failure
- **GIVEN** `[data].targets` contains entries
- **WHEN** `fuga move dest/` fails for any target
- **THEN** the CLI SHALL leave `[data].targets` unchanged
