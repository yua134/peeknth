#[cfg(feature = "alloc")]
mod core;

mod sizedpeekdn;

#[cfg(feature = "alloc")]
pub use core::{PeekDN, peekdn};

pub use sizedpeekdn::{SizedPeekDN, sizedpeekdn};
