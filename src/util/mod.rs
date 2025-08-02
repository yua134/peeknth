#[cfg(any(feature = "peekdn", feature = "peekn"))]
mod either;
#[cfg(any(feature = "peekdn", feature = "peekn"))]
pub(crate) use either::Either;

#[cfg(any(feature = "peekde", feature = "peekn"))]
mod func;
#[cfg(any(feature = "peekde", feature = "peekn"))]
pub(crate) use func::get_start_end;

#[cfg(any(feature = "peekdn", feature = "peekde"))]
mod peeksource;
#[cfg(any(feature = "peekdn", feature = "peekde"))]
pub(crate) use peeksource::PeekSource;

#[cfg(any(feature = "peekdn", feature = "peekn"))]
mod ringbuffer;
#[cfg(any(feature = "peekdn", feature = "peekn"))]
pub(crate) use ringbuffer::Buffer;
