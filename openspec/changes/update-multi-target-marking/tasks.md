## 1. Specification
- [ ] 1.1 Update cli-foundations spec with multi-target requirements
- [ ] 1.2 Validate the change with `openspec validate update-multi-target-marking --strict`

## 2. Configuration Layer
- [ ] 2.1 Introduce `[data].targets` storage, keeping absolute path normalization
- [ ] 2.2 Implement legacy `[data].target` migration and persist upgraded files when writable

## 3. CLI Command Updates
- [ ] 3.1 Extend `fuga mark` to overwrite, append without duplicates, reset, and show targets
- [ ] 3.2 Ensure `copy`, `move`, and `link` iterate all targets with directory/file destination semantics and detailed logging
- [ ] 3.3 Reset the mark list only after successful `move` operations

## 4. Testing & Tooling
- [ ] 4.1 Expand CLI flow tests to cover multi-target scenarios, including destination edge cases
- [ ] 4.2 Add regression tests for migration and empty-target error handling
