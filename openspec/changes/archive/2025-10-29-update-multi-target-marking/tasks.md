## 1. Specification
- [x] 1.1 Update cli-foundations spec with multi-target requirements
- [x] 1.2 Validate the change with `openspec validate update-multi-target-marking --strict`

## 2. Configuration Layer
- [x] 2.1 Introduce `[data].targets` storage, keeping absolute path normalization
- [x] 2.2 Implement legacy `[data].target` migration and persist upgraded files when writable

## 3. CLI Command Updates
- [x] 3.1 Extend `fuga mark` to overwrite, append without duplicates, reset, and show targets
- [x] 3.2 Ensure `copy`, `move`, and `link` iterate all targets with directory/file destination semantics and detailed logging
- [x] 3.3 Reset the mark list only after successful `move` operations

## 4. Testing & Tooling
- [x] 4.1 Expand CLI flow tests to cover multi-target scenarios, including destination edge cases
- [x] 4.2 Add regression tests for migration and empty-target error handling
