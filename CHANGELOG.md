# Changelog

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
