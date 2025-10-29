## Why
Current mark operations only persist a single filesystem path, which prevents copying, moving, or linking a curated set of files in one step. Users want to mark multiple paths and run one command to act on the whole set.

## What Changes
- Introduce a `targets` array in the persisted configuration and migrate the existing single `target` value
- Expand `fuga mark` to manage the mark list, including overwrite, add, reset, and list sub-flows
- Teach `copy`, `move`, and `link` commands to iterate over the mark list with directory/file destination semantics and structured logging
- Reset the mark list after successful `move` executions while leaving it intact for `copy` and `link`
- **BREAKING** Update the mark storage contract so single-target consumers must use the new list semantics

## Impact
- Affected specs: cli-foundations
- Affected code: src/config, src/commands/mark.rs (and related), src/commands/{copy,move,link}.rs, services/filesystem.rs, tests/cli_flows.rs
