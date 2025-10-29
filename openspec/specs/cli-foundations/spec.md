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

