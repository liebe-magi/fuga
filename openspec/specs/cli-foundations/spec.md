# cli-foundations Specification

## Purpose
TBD - created by archiving change improve-cli-foundations. Update Purpose after archive.
## Requirements
### Requirement: Reliable Mark Error Reporting
The CLI SHALL surface filesystem failures during mark operations by returning a non-zero exit code and propagating a clear error message sourced from the underlying `FugaError`.

#### Scenario: Missing target path
- **GIVEN** no file or directory exists at the provided mark path
- **WHEN** the user executes `fuga mark <missing-path>`
- **THEN** the command SHALL exit with a non-zero status
- **AND** the UI SHALL print that the mark failed because the path was not found

#### Scenario: Permission denied target
- **GIVEN** the mark path exists but cannot be read due to permissions
- **WHEN** the user executes `fuga mark <restricted-path>`
- **THEN** the command SHALL exit with a non-zero status
- **AND** the UI SHALL print the permission error reported by the filesystem service

### Requirement: Path Integrity for File Operations
File operations SHALL preserve exact filesystem paths when resolving sources and destinations, avoiding lossy Unicode conversions and using platform-appropriate separators.

#### Scenario: Preserve non-UTF-8 paths
- **GIVEN** a marked path containing non-UTF-8 bytes
- **WHEN** copy, move, or link commands resolve absolute paths
- **THEN** the services SHALL either operate on the original bytes or emit a specific error explaining the path cannot be represented, without silently modifying the value

#### Scenario: Assemble destination under directory argument
- **GIVEN** the destination argument resolves to an existing directory
- **WHEN** copy, move, or link determine the final destination path
- **THEN** the services SHALL combine paths using `PathBuf` semantics so separators and casing follow the host platform

### Requirement: Cross-Platform Terminal Feedback
Terminal feedback SHALL rely on a cross-platform styling library and offer configurable fallbacks when emoji glyphs are unavailable.

#### Scenario: Windows build compatibility
- **GIVEN** the project is built on Windows
- **WHEN** the terminal UI component links against its dependencies
- **THEN** the build SHALL succeed because the styling library supports Windows consoles

#### Scenario: ASCII fallback mode
- **GIVEN** the runtime cannot render emoji glyphs
- **WHEN** `fuga` emits status icons
- **THEN** the UI SHALL provide ASCII-safe replacements that convey the same meaning

### Requirement: Regression Coverage for Core Commands
Automated tests SHALL cover core command flows, validating both successful operations and expected failure modes.

#### Scenario: Happy-path integration tests
- **GIVEN** temporary test directories and files
- **WHEN** mark, copy, move, and link flows are executed in tests
- **THEN** the tests SHALL assert that the filesystem results and UI messages match the documented behavior

#### Scenario: Error-path regression tests
- **GIVEN** scenarios such as missing marks, duplicate destination paths, or permission errors
- **WHEN** the commands under test encounter those conditions
- **THEN** the tests SHALL verify that non-zero exits and error messages align with the error-handling requirement

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

#### Scenario: List mark targets
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

