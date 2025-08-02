#[cfg(feature = "alloc")]
mod core;

mod sizedpeekn;

#[cfg(feature = "alloc")]
pub use core::{PeekN, peekn};

pub use sizedpeekn::{SizedPeekN, sizedpeekn};
