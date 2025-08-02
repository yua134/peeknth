# Changelog

## [0.3.0] - 2025-08-02
[Diff](https://github.com/yua134/peeknth/compare/v0.2.0...v0.3.0)

### Added
- `SizedPeekN`: A peekable iterator with a fixed-size front buffer.
- `SizedPeekDN`: A double-ended peekable iterator with fixed-size front and back buffers.
- New feature flag: `alloc` (for heap-using types like `PeekN`, `PeekDN`, etc.)
- New APIs: `while_peek`, `while_next`, `while_peek_front`, `while_next_back`, etc.

### Changed
- Restructured crate into modular files (e.g., `peekn.rs`, `peekdn.rs`, etc.)
- Corrected internal logic of several peek-related methods to ensure expected behavior
- Improved documentation with clarified trait bounds (`Copy`, `Iterator`)
- Updated `README.md` to reflect new features and `no_std` compatibility

### Removed
- Conversion from `PeekDN` to `PeekN` (`From<PeekDN>` impl)

### Notes
- Contains a few breaking changes from v0.2.0:
  - Internal structure and module paths changed
  - API behavior may differ in edge cases
- All tests passing
- Crate is now fully `#![no_std]` compatible (uses `alloc` feature for dynamic types)

## [0.2.0] - 2025-07-27
[Diff](https://github.com/yua134/peeknth/compare/v0.1.1...v0.2.0)
### Added
- `PeekDN`: A double-ended iterator adapter supporting both `next` and `next_back` peeking
- `PeekableDE`: A lightweight bidirectional peekable adapter (like `Peekable` but double-ended)
- Feature flags for modular builds: `peekn`, `peekdn`, `peekde`, `all`
- `peek_front`, `peek_back`, and related methods for bi-directional access

### Changed
- Restructured crate into modular files (`peekn.rs`, `peekdn.rs`, `peekablede.rs`)
- Improved documentation and usage examples
- Updated README to reflect new features

### Removed
- (none)

### Notes
- No breaking changes from 0.1.x
- All tests passing
---

## [0.1.1] - 2025-07-26
[Diff](https://github.com/yua134/peeknth/compare/v0.1.0...v0.1.1)
### Changed
- Improved internal safety and correctness
- Minor refactor of peek buffer logic

### Notes
- No new features, just internal safety updates



---

## [0.1.0] - 2025-07-25

[Source](https://github.com/yua134/peeknth/releases/tag/v0.1.0)
### Added
- Initial release: `PeekN` adapter with `.peek_nth`, `.peek_range`, etc.
