## Why
Recent code review flagged multiple reliability gaps: mark operations mask errors, filesystem helpers silently coerce problematic paths, Windows output is blocked by the Unix-only UI stack, and there are no regression tests to guard these behaviors. We need a coordinated plan to stabilize core flows before expanding features.

## What Changes
- Fix command error handling so missing or inaccessible marks surface actionable failures
- Rework path utilities to avoid lossy string conversions and supply correct platform semantics
- Replace the Unix-only terminal formatting with a cross-platform solution and provide emoji fallbacks
- Back critical flows with automated tests covering success and failure scenarios

## Impact
- Affected specs: cli-foundations
- Affected code: `src/commands/mark.rs`, `src/services/filesystem.rs`, `src/services/path.rs`, `src/ui/formatting.rs`, new/updated tests
