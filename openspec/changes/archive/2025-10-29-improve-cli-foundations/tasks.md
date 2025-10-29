## 1. Error Handling
- [x] 1.1 Bubble up a structured error when mark operations target missing or inaccessible paths
- [x] 1.2 Ensure filesystem service distinguishes permission errors from missing files

## 2. Path Safety
- [x] 2.1 Update absolute-path resolution to preserve non-UTF-8 paths or fail loudly
- [x] 2.2 Replace string concatenation with `PathBuf` operations in destination-name logic

## 3. Cross-Platform Output
- [x] 3.1 Swap `termion` for `crossterm` (or equivalent) to unblock Windows builds
- [x] 3.2 Add configurable ASCII fallbacks when emoji glyphs are unavailable

## 4. Reliability Tests
- [x] 4.1 Add integration-style tests for mark/copy/move/link happy paths
- [x] 4.2 Add tests covering error cases (missing mark, permission denial, duplicate path)
