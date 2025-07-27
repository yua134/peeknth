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
- 🧩 Implements Iterator, so compatible with .map(), .filter(), etc.

## 🔧 Feature Flags

| Feature     | Description                                   |
|-------------|-----------------------------------------------|
| `peekn`     | Basic N-step forward peek                     |
| `peekdn`    | Double-ended peek with front/back buffers     |
| `peekde`    | Lightweight bidirectional peek (1 element)    |
| `all`       | Enables all of the above                      |

You can control features in `Cargo.toml` like:

```toml
peeknth = { version = "0.2", features = ["peekdn"] }
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

let mut iter = peekn(0..);
let values: Vec<_> = iter.peek_range(1..4).cloned().collect();
assert_eq!(values, vec![1, 2, 3]);
```

## 📦 Crate Info
- License: MIT OR Apache-2.0

- Repository: [GitHub](https://github.com/yua134/peeknth)

## 🔖 License
This project is dual-licensed under either:

- MIT
- Apache-2.0

You may choose the license that best suits your needs.