//! peeknth — An iterator adapter for N-th and double-ended peeking.
//!
//! This crate provides optional modules depending on the enabled feature:
//! - `peekn`: Peek forward multiple steps
//! - `peekdn`: Double-ended peeking with front/back buffers
//! - `peekde`: Lightweight front/back peeking for one item each
#[cfg(feature = "peekn")]
mod peekn;
#[cfg(feature = "peekn")]
pub use peekn::{PeekN, peekn};

#[cfg(feature = "peekdn")]
mod peekdn;
#[cfg(feature = "peekdn")]
pub use peekdn::{PeekDN, peekdn};

#[cfg(feature = "peekde")]
mod peekablede;

#[cfg(feature = "peekde")]
pub use peekablede::{PeekableDE, peekablede};
