[![Crates.io](https://img.shields.io/crates/v/peeknth.svg)](https://crates.io/crates/peeknth)
[![Docs.rs](https://docs.rs/peeknth/badge.svg)](https://docs.rs/peeknth)
[![CI](https://github.com/yua134/peeknth/actions/workflows/ci.yml/badge.svg)](https://github.com/yua134/peeknth/actions/workflows/ci.yml)

# peeknth

An iterator adapter for peeking multiple elements ahead or behind in a Rust iterator.

## ✨ Features

- 🔭 Peek N elements ahead (`peek_nth(n)`)
- 🔁 Peek from both ends with double-ended iterators (`peek_front`, `peek_back`)
- 🎯 Range-based peeking (`peek_range(start..end)`)
- ⚡ Lightweight adapters with feature flags (`peekn`, `peekdn`, `peekde`)
- 📐 `SizedPeekN`, `SizedPeekDn` – Zero-allocation, fixed-capacity peek buffers
- 🧩 Implements Iterator, so compatible with .map(), .filter(), etc.

## 🔧 Feature Flags

| Feature     | Description                                                           |
|-------------|-----------------------------------------------------------------------|
| `peekn`     | Enables `PeekN`, `SizedPeekN` (forward peek types)                    |
| `peekdn`    | Enables `PeekDN`, `SizedPeekDN` (double-ended peek types)             |
| `peekde`    | Enables `PeekableDE`, a lightweight double-ended peek wrapper         |
| `alloc`     | Required for types that use dynamic buffers (`PeekN`, `PeekDN`, etc.) |
| `default`   | `["peekn", "alloc"]`                                                  |
| `all`       | Enables all features                                                  |

You can control features in `Cargo.toml` like:

```toml
peeknth = { version = "0.3", features = ["peekdn"] }
```

## 🔒 no_std Compatibility

This crate is 100% `#![no_std]` compatible.

- Types like `SizedPeekN`, `SizedPeekDN` and `PeekableDE` require **no allocation** and run on bare-metal targets.
- Types like `PeekN` and `PeekDN` require the `alloc` crate to support internal buffers (e.g. `VecDeque`).

To use in strict no_std (no alloc), only use the Sized* types:

```toml
peeknth = { version = "0.2", default-features = false, features = ["peekn","peekdn"] }
```

## 🚀 Usage
### Forward peek (peekn)
```rust
use peeknth::peekn;

let mut iter = peekn(1..);
assert_eq!(iter.peek(), Some(&1));
assert_eq!(iter.peek_nth(2), Some(&3));
```
### Double-ended peek (peekdn)
```rust
use peeknth::peekdn;

let mut iter = peekdn(1..=5);
assert_eq!(iter.peek_front(), Some(&1));
assert_eq!(iter.peek_back(), Some(&5));

assert_eq!(iter.next(), Some(1));
assert_eq!(iter.next_back(), Some(5));
```
### Lightweight peek (peekablede)
```rust
use peeknth::peekablede;

let mut iter = peekablede(10..=12);
assert_eq!(iter.peek_front(), Some(&10));
assert_eq!(iter.peek_back(), Some(&12));
```
### Peek a range
```rust
use peeknth::peekn;

let mut iter = peekn(0..5);
let values: Vec<_> = iter.peek_range(1..4).cloned().collect();
assert_eq!(values, vec![1, 2, 3]);
```
### Consume while matching (while_next)
```rust
use peeknth::peekn;

let mut iter = peekn(0..);
let result: Vec<_> = iter.while_next(|&x| x < 5).collect();
assert_eq!(result, vec![0, 1, 2, 3, 4]);
```
### Fixed-capacity peek (SizedPeekN / SizedPeekDN)
```rust
use peeknth::sizedpeekn;

let mut it = sizedpeekn::<_, 4>(0..);
assert_eq!(it.peek_nth(1), Some(&1));
```
```rust
use peeknth::sizedpeekdn;

let mut it = sizedpeekdn::<_, 2, 2>(1..=5);
assert_eq!(it.peek_back_nth(0), Some(&5));
```

## 📦 Crate Info
- License: MIT OR Apache-2.0
- Crate: [peeknth on crates.io](https://crates.io/crates/peeknth)
- Docs: [docs.rs/peeknth](https://docs.rs/peeknth)
- Repository: [GitHub](https://github.com/yua134/peeknth)
- `#![no_std]` compatible
- Requires `alloc` for heap-backed types (`PeekN`, `PeekDn`, etc.)
- Fully usable without `alloc`: `SizedPeekN`, `SizedPeekDN`, `PeekableDE`

## 🔖 License
This project is dual-licensed under either:

- MIT
- Apache-2.0

You may choose the license that best suits your needs.