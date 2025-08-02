//! # peeknth
//!
//! An iterator adapter for peeking multiple elements ahead or behind.
//!
//! This crate provides flexible, composable tools for lookahead and lookbehind peeking,
//! supporting both heap-allocated and zero-allocation use cases.
//!
//! ## Available Modules (feature-dependent)
//!
//! - **`peekn`** — Forward peeking multiple steps (`PeekN`, `SizedPeekN`)
//! - **`peekdn`** — Double-ended peeking from both front/back (`PeekDn`, `SizedPeekDn`)
//! - **`peekde`** — Lightweight double-ended peek (1 element each, via `PeekableDE`)
//!
//! ## `no_std` Support
//!
//! This crate is fully `#![no_std]` compatible.
//!
//! - `PeekN`, `PeekDn`, `PeekableDE` require the `alloc` crate
//! - `SizedPeekN`, `SizedPeekDn` require no allocation at all

#![no_std]
#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "peekn")]
mod peekn;
#[cfg(feature = "peekn")]
pub use peekn::{PeekN, SizedPeekN, peekn, sizedpeekn};

#[cfg(feature = "peekdn")]
mod peekdn;
#[cfg(feature = "peekdn")]
pub use peekdn::{PeekDN, SizedPeekDN, peekdn, sizedpeekdn};

#[cfg(feature = "peekde")]
mod peekablede;

#[cfg(feature = "peekde")]
pub use peekablede::{PeekableDE, peekablede};

mod util;
#[cfg(any(feature = "peekde", feature = "peekn"))]
pub(crate) use util::get_start_end;
