use crate::util::Buffer;

use core::{
    iter::{FusedIterator, Iterator, Peekable},
    ops::RangeBounds,
};

/// A peekable iterator with a fixed-size front buffer.
///
/// `SizedPeekN<I, const S: usize>` allows peeking up to `S` items ahead in a single direction
/// (the front), without consuming them. The peeked items are stored in a fixed-size buffer,
/// and accessing beyond that limit will result in a panic.
///
/// Compared to `Peekable`, this struct supports arbitrary-indexed peeking (`peek_nth`)
/// and range access (`peek_range`), with predictable memory usage.
///
/// # Type Parameters
/// - `I`: The base iterator.
/// - `I::Item`: Must implement `Copy`, since items are stored by value in the buffer.
/// - `S`: Maximum number of front items that can be peeked.
///
/// # Panics
/// All `peek_nth(n)` and `peek_range`/`peek_range_mut` calls must satisfy `n < S`,
/// or the program will panic.
///
/// # Example
/// ```rust
/// use peeknth::sizedpeekn;
/// let mut peekn = sizedpeekn::<_, 3>(0..);
/// assert_eq!(peekn.peek_nth(2), Some(&2));
/// ```
pub struct SizedPeekN<I, const S: usize>
where
    I: Iterator,
    I::Item: Copy,
{
    pub(crate) iter: I,
    pub(crate) buffer: Buffer<I::Item, S>,
}

impl<I, const S: usize> Iterator for SizedPeekN<I, S>
where
    I: Iterator,
    I::Item: Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.buffer.pop_front() {
            Some(item)
        } else {
            self.iter.next()
        }
    }
}

impl<I, const S: usize> Clone for SizedPeekN<I, S>
where
    I: Iterator + Clone,
    I::Item: Copy + Clone,
{
    fn clone(&self) -> Self {
        SizedPeekN {
            iter: self.iter.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl<I, const S: usize> core::fmt::Debug for SizedPeekN<I, S>
where
    I: Iterator + core::fmt::Debug,
    I::Item: Copy + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SizedPeekN")
            .field("iter", &self.iter)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<I, const S: usize> Eq for SizedPeekN<I, S>
where
    I: Iterator + Eq,
    I::Item: Copy + Eq,
{
}

impl<I, const S: usize> PartialEq for SizedPeekN<I, S>
where
    I: Iterator + PartialEq,
    I::Item: Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter == other.iter && self.buffer == other.buffer
    }
}

impl<I, const S: usize> ExactSizeIterator for SizedPeekN<I, S>
where
    I: ExactSizeIterator,
    I::Item: Copy,
{
    fn len(&self) -> usize {
        self.buffer.len() + self.iter.len()
    }
}

impl<I, const S: usize> From<Peekable<I>> for SizedPeekN<Peekable<I>, S>
where
    I: Iterator,
    I::Item: Clone + Copy,
{
    fn from(mut peekable: Peekable<I>) -> Self {
        let buffer = Buffer::from_iter(peekable.peek().cloned());

        SizedPeekN {
            iter: peekable,
            buffer,
        }
    }
}

impl<I, const S: usize> FusedIterator for SizedPeekN<I, S>
where
    I: FusedIterator,
    I::Item: Copy,
{
}

impl<I, const S: usize> SizedPeekN<I, S>
where
    I: Iterator,
    I::Item: Copy,
{
    /// Creates a new `SizedPeekN` from the given iterator.
    ///
    /// The internal fixed-size peek buffer is initialized empty.
    pub fn new(iter: I) -> Self {
        SizedPeekN {
            iter,
            buffer: Buffer::new(),
        }
    }

    /// Peeks at the `n`-th item without consuming it.
    ///
    /// This method attempts to fill the peek buffer up to index `n`, and returns
    /// a reference to the item at that position if available.
    ///
    /// # Arguments
    /// * `n` - Zero-based index into the peek buffer.
    ///
    /// # Panics
    /// Panics if `n >= self.capacity()`.
    ///
    /// # Returns
    /// `Some(&item)` if available, otherwise `None`.
    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        core::debug_assert!(n < usize::MAX, "peek_nth() with usize::MAX is likely a bug");

        if self.buffer.len() > n {
            return self.buffer.get(n);
        }

        while self.buffer.len() <= n {
            let next_item = self.iter.next()?;
            self.buffer.push_back(next_item);
        }

        self.buffer.get(n)
    }

    /// Mutably peeks at the `n`-th item without consuming it.
    ///
    /// This method attempts to fill the peek buffer up to index `n`, and returns
    /// a mutable reference to the item at that position if available.
    ///
    /// # Arguments
    /// * `n` - Zero-based index into the peek buffer.
    ///
    /// # Panics
    /// Panics if `n >= self.capacity()`.
    ///
    /// # Returns
    /// `Some(&mut item)` if available, otherwise `None`.
    pub fn peek_nth_mut(&mut self, n: usize) -> Option<&mut I::Item> {
        core::debug_assert!(
            n < usize::MAX,
            "peek_nth_mut() with usize::MAX is likely a bug"
        );

        if self.buffer.len() > n {
            return self.buffer.get_mut(n);
        }

        while self.buffer.len() <= n {
            let next_item = self.iter.next()?;
            self.buffer.push_back(next_item);
        }

        self.buffer.get_mut(n)
    }

    /// Peeks at the next item in the iterator without consuming it.
    ///
    /// Equivalent to `peek_nth(0)`.
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    /// Peeks at the next item in the iterator as a mutable reference, without consuming it.
    ///
    /// Equivalent to `peek_nth_mut(0)`.
    ///
    /// This allows in-place mutation of the next item.
    pub fn peek_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_nth_mut(0)
    }

    /// Peeks a range of items without consuming them.
    ///
    /// Attempts to fill the buffer up to the specified range and returns a slice of
    /// items within that range. The range must lie entirely within the fixed capacity.
    ///
    /// # Arguments
    /// * `range` - A range of indices (e.g., `0..3`, `1..=4`) to peek.
    ///
    /// # Panics
    /// Panics if the upper bound of the range exceeds `self.capacity()`.
    ///
    /// # Returns
    /// A slice of peeked items within the specified range, possibly shorter if
    /// the iterator is exhausted.
    pub fn peek_range<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl Iterator<Item = &<I as Iterator>::Item>
    where
        I: ExactSizeIterator,
    {
        use crate::get_start_end;
        let (start, end) = get_start_end(range, self.len());

        core::debug_assert!(
            start < end,
            "peek_range: start ({start}) must be less than end ({end})"
        );
        if start >= end {
            return self.buffer.range(0..0);
        }

        for i in start..end {
            if self.peek_nth(i).is_none() {
                break;
            }
        }

        let safe_end = end.min(self.buffer.len());
        core::debug_assert!(
            end <= self.buffer.len(),
            "peek_range: end out of bounds: {} > {}",
            end,
            self.buffer.len()
        );
        self.buffer.range(start..safe_end)
    }

    /// Mutably peeks a range of items without consuming them.
    ///
    /// Attempts to fill the buffer up to the specified range and returns a mutable slice
    /// of items within that range. The range must lie entirely within the fixed capacity.
    ///
    /// # Arguments
    /// * `range` - A range of indices (e.g., `0..3`, `1..=4`) to peek mutably.
    ///
    /// # Panics
    /// Panics if the upper bound of the range exceeds `self.capacity()`.
    ///
    /// # Returns
    /// A mutable slice of peeked items within the specified range, possibly shorter
    /// if the iterator is exhausted.
    pub fn peek_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl Iterator<Item = &mut <I as Iterator>::Item>
    where
        I: ExactSizeIterator,
    {
        use crate::get_start_end;
        let (start, end) = get_start_end(range, self.len());

        core::debug_assert!(
            start < end,
            "peek_range: start ({start}) must be less than end ({end})"
        );
        if start >= end {
            return self.buffer.range_mut(0..0);
        }

        for i in start..end {
            if self.peek_nth(i).is_none() {
                break;
            }
        }

        let safe_end = end.min(self.buffer.len());
        core::debug_assert!(
            end <= self.buffer.len(),
            "peek_range: end out of bounds: {} > {}",
            end,
            self.buffer.len()
        );
        self.buffer.range_mut(start..safe_end)
    }

    /// Advances the iterator and returns the next value only if it satisfies the predicate.
    ///
    /// If the next item does not match, it is pushed back to the peek buffer.
    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        match self.next() {
            Some(matched) if func(&matched) => Some(matched),
            Some(other) => {
                self.buffer.push_front(other);
                None
            }
            None => None,
        }
    }

    /// Advances the iterator and returns the next value only if it is equal to `expected`.
    ///
    /// If the value does not match, it is pushed back to the buffer.
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }

    /// Converts this `SizedPeekN` into a standard `Peekable`, discarding buffered items.
    ///
    /// This is a lossy conversion: any elements stored in the internal buffer will be dropped.
    pub fn into_peekable_lossy(self) -> Peekable<I> {
        self.iter.peekable()
    }

    /// Returns the number of items currently buffered (peeked but not consumed).
    #[inline]
    pub fn peeked_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the buffer contains at least `n + 1` items.
    #[inline]
    pub fn has_peeked(&self, n: usize) -> bool {
        self.peeked_len() > n
    }

    /// Clears all buffered items.
    #[inline]
    pub fn clear_peeked(&mut self) {
        self.buffer.clear();
    }

    /// Discards the first `until` buffered items.
    #[inline]
    pub fn drain_peeked(&mut self, until: usize) {
        let until = until.min(self.buffer.len());
        core::debug_assert!(
            until <= self.buffer.len(),
            "drain_peeked: requested to drain until {} but buffer length is {}",
            until,
            self.buffer.len()
        );
        self.buffer.drain(..until);
    }

    /// Consumes and yields elements from the iterator while the predicate returns `true`.
    ///
    /// If an element does not satisfy the predicate, it is pushed back to the front of the buffer.
    ///
    /// # Returns
    /// An iterator of matching elements. Stops at the first non-matching element.
    pub fn while_next(
        &mut self,
        mut func: impl FnMut(&I::Item) -> bool,
    ) -> impl Iterator<Item = I::Item> {
        core::iter::from_fn(move || {
            if let Some(peeked) = self.next() {
                if func(&peeked) {
                    Some(peeked)
                } else {
                    self.buffer.push_front(peeked);
                    None
                }
            } else {
                None
            }
        })
    }

    /// Peeks ahead while the predicate returns `true`, without consuming any elements.
    ///
    /// This method counts how many consecutive elements satisfy the predicate,
    /// stopping at the first failure or when the peek buffer limit (`capacity`) is reached.
    ///
    /// # Returns
    /// The number of items that match the predicate from the front.
    pub fn while_peek(&mut self, mut func: impl FnMut(&I::Item) -> bool) -> usize {
        let mut count = 0;
        let limit = self.capacity();

        while let Some(item) = self.peek_nth(count) {
            if func(item) && count < limit {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    /// Returns the maximum number of items that can be peeked without consuming.
    ///
    /// This reflects the fixed-size capacity of the internal buffer.
    /// Calling `peek_nth(n)` with `n >= capacity()` is undefined behavior (and may panic).
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buffer.capacity()
    }
}

/// Creates a new `SizedPeekN<I, S>` from the given iterator.
///
/// This is a convenience constructor, equivalent to `SizedPeekN::new(iter)`.
pub fn sizedpeekn<I, const S: usize>(iter: I) -> SizedPeekN<I, S>
where
    I: Iterator,
    I::Item: Copy,
{
    SizedPeekN::new(iter)
}
