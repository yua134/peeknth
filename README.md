[![Crates.io](https://img.shields.io/crates/v/peeknth.svg)](https://crates.io/crates/peeknth)
[![Docs.rs](https://docs.rs/peeknth/badge.svg)](https://docs.rs/peeknth)

# peeknth

An iterator adapter for peeking multiple elements ahead in a Rust iterator.

## ✨ Features

- Peek at any N-th future element (`.peek_nth(n)`)
- Peek over ranges (`.peek_range(start..end)`)
- Lightweight and generic
- Iterator-compatible (supports `.map()`, `.filter()`, etc.)


## 🚀 Usage
```rust
use peeknth::peekn;

fn main() {
    let mut iter = peekn(1..);

    assert_eq!(iter.peek(), Some(&1));
    assert_eq!(iter.peek_nth(2), Some(&3));

    // Consume elements
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.peek(), Some(&2));
}
```
You can also peek a range:

```rust
use peeknth::peekn;

let mut iter = peekn(0..);
let slice: Vec<_> = iter.peek_range(1..4).cloned().collect();
assert_eq!(slice, vec![1, 2, 3]);
```

## 📘 Documentation
Run this to view local documentation:

```bash
cargo doc --open
```
Or view online at docs.rs/peeknth (once published).

## 🧪 Testing
```bash
cargo test
```
Unit tests and doc tests are included.

## 📦 Crate info (optional)
License: MIT OR Apache-2.0

Minimum Rust version: 1.60+


## 🔖 License
This project is dual-licensed under MIT or Apache-2.0.
You can choose either license.
