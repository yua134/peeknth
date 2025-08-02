#[cfg(feature = "peekde")]
use crate::PeekableDE;
use crate::SizedPeekN;
use core::{
    iter::{FusedIterator, Iterator, Peekable},
    ops::RangeBounds,
};

extern crate alloc;

use alloc::collections::VecDeque;

/// `PeekN` is an iterator adapter that allows peeking at any future element
/// in the iterator, not just the next one.
///
/// Unlike the standard `Peekable` iterator, which only supports peeking at the
/// next element, `PeekN` allows you to look ahead by any number of steps,
/// buffering elements as needed.
///
/// # Examples
///
/// ```
/// # use peeknth::peekn;
///
/// let mut iter = peekn(1..);
///
/// assert_eq!(iter.peek(), Some(&1));
/// assert_eq!(iter.peek_nth(2), Some(&3));
/// assert_eq!(iter.next(), Some(1));
/// ```
pub struct PeekN<I: Iterator> {
    pub(crate) iter: I,
    pub(crate) buffer: VecDeque<I::Item>,
}

impl<I: Iterator> Iterator for PeekN<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.buffer.pop_front() {
            Some(item)
        } else {
            self.iter.next()
        }
    }
}

impl<I> From<Peekable<I>> for PeekN<Peekable<I>>
where
    I: Iterator,
    I::Item: Clone,
{
    fn from(mut peekable: Peekable<I>) -> Self {
        let buffer = peekable.peek().cloned().into_iter().collect();

        PeekN {
            iter: peekable,
            buffer,
        }
    }
}

impl<I, const S: usize> From<SizedPeekN<I, S>> for PeekN<I>
where
    I: Iterator,
    I::Item: Copy,
{
    fn from(value: SizedPeekN<I, S>) -> Self {
        PeekN {
            iter: value.iter,
            buffer: VecDeque::from(value.buffer),
        }
    }
}

#[cfg(feature = "peekde")]
impl<I: DoubleEndedIterator> From<PeekableDE<I>> for PeekN<I> {
    fn from(peekable_de: PeekableDE<I>) -> Self {
        let buffer = peekable_de.front.flatten().into_iter().collect();
        PeekN {
            iter: peekable_de.iter,
            buffer,
        }
    }
}

impl<I> Clone for PeekN<I>
where
    I: Iterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        PeekN {
            iter: self.iter.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl<I: FusedIterator> FusedIterator for PeekN<I> {}

impl<I> core::fmt::Debug for PeekN<I>
where
    I: Iterator + core::fmt::Debug,
    I::Item: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PeekN")
            .field("iter", &self.iter)
            .field("buffer", &self.buffer)
            .finish()
    }
}

impl<I> Eq for PeekN<I>
where
    I: Iterator + Eq,
    I::Item: Eq,
{
}

impl<I> PartialEq for PeekN<I>
where
    I: Iterator + PartialEq,
    I::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter == other.iter && self.buffer == other.buffer
    }
}

impl<I: ExactSizeIterator> ExactSizeIterator for PeekN<I> {
    fn len(&self) -> usize {
        self.buffer.len() + self.iter.len()
    }
}

impl<I: Iterator> PeekN<I> {
    /// Creates a new `PeekN` wrapping the given iterator.
    pub fn new(iter: I) -> Self {
        PeekN {
            iter,
            buffer: VecDeque::new(),
        }
    }

    /// Creates a new `PeekN` with pre-allocated buffer capacity.
    ///
    /// This can improve performance if you expect to peek ahead multiple elements.
    ///
    /// # Arguments
    ///
    /// - `capacity`: The initial capacity of the internal buffer.
    ///
    /// # Examples
    /// ```
    /// # use peeknth::PeekN;
    /// let iter = 0..;
    /// let peek_iter = PeekN::with_capacity(iter, 10);
    /// ```
    pub fn with_capacity(iter: I, capacity: usize) -> Self {
        PeekN {
            iter,
            buffer: VecDeque::with_capacity(capacity),
        }
    }

    /// Peeks at the `n`-th element from the current position without advancing the iterator.
    ///
    /// Elements are buffered internally as needed.
    ///
    /// # Arguments
    ///
    /// - `n`: The number of steps to look ahead (0-based).
    ///
    /// # Panics
    /// This function will panic in debug builds if called with `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use peeknth::{peekn, PeekN};
    /// let mut iter = peekn(10..);
    /// assert_eq!(iter.peek_nth(3), Some(&13));
    /// ```
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

    /// Returns a mutable reference to the `n`-th element without advancing the iterator.
    ///
    /// This allows you to modify a peeked value in-place before it's consumed by `next()`.
    ///
    /// # Arguments
    ///
    /// - `n`: The number of steps to look ahead (0-based).
    ///
    /// # Panics
    /// This function will panic in debug builds if called with `usize::MAX`.
    ///
    /// # Examples
    /// ```
    /// # use peeknth::{peekn, PeekN};
    /// let mut iter = peekn(0..);
    /// if let Some(x) = iter.peek_nth_mut(1) {
    ///     *x += 10;
    /// }
    /// assert_eq!(iter.next(), Some(0));
    /// assert_eq!(iter.next(), Some(11));
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// # use peeknth::{peekn, PeekN};
    /// let mut iter = peekn(10..);
    /// assert_eq!(iter.peek(), Some(&10));
    /// ```
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_nth(0)
    }

    /// Peeks at the next item in the iterator as a mutable reference, without consuming it.
    ///
    /// Equivalent to `peek_nth_mut(0)`.
    ///
    /// This allows in-place mutation of the next item.
    ///
    /// # Examples
    /// ```
    /// # use peeknth::peekn;
    /// let mut iter = peekn(0..);
    /// if let Some(x) = iter.peek_mut() {
    ///     *x += 100;
    /// }
    /// assert_eq!(iter.next(), Some(100));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_nth_mut(0)
    }

    /// Peeks a range of elements from the internal buffer without consuming them.
    ///
    /// This method attempts to fill the internal buffer up to the specified range by repeatedly
    /// calling `peek_nth(i)` for each index in the range. If any of the required elements
    /// cannot be fetched (i.e., the iterator is exhausted), the returned slice will be truncated
    /// accordingly.
    ///
    /// # Arguments
    /// * `range` - The range of indices to access within the buffer. Must satisfy `start < end`.
    ///
    /// # Returns
    /// A slice of peeked items in the specified range. If the iterator runs out of items,
    /// the returned slice will be shorter than requested.
    ///
    /// # Examples
    /// ```
    /// # use peeknth::{peekn, PeekN};
    /// let mut iter = peekn(0..5);
    /// let values: Vec<_> = iter.peek_range(1..4).cloned().collect();
    /// assert_eq!(values, vec![1, 2, 3]);
    /// ```
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

    /// Mutably peeks a range of elements from the internal buffer without consuming them.
    ///
    /// This method attempts to fill the internal buffer up to the specified range by repeatedly
    /// calling `peek_nth(i)` for each index in the range. If any of the required elements
    /// cannot be fetched (i.e., the iterator is exhausted), the returned slice will be truncated
    /// accordingly.
    ///
    /// # Arguments
    /// * `range` - The range of indices to access within the buffer. Must satisfy `start < end`.
    ///
    /// # Returns
    /// A mutable iterator over the available elements in the specified range,
    /// potentially shorter than requested if the iterator runs out of items.
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
    ///
    /// # Examples
    /// ```
    /// # use peeknth::peekn;
    /// let mut iter = peekn(0..);
    /// assert_eq!(iter.next_if(|&x| x < 3), Some(0));
    /// assert_eq!(iter.next_if(|&x| x > 10), None); // 1 is pushed back
    /// assert_eq!(iter.peek(), Some(&1));
    /// ```
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
    ///
    /// # Examples
    /// ```
    /// # use peeknth::peekn;
    /// let mut iter = peekn([1, 2, 3].into_iter());
    /// assert_eq!(iter.next_if_eq(&1), Some(1));
    /// assert_eq!(iter.next_if_eq(&5), None); // 2 is pushed back
    /// assert_eq!(iter.peek(), Some(&2));
    /// ```
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }

    /// Converts this `PeekN` into a standard `Peekable`, discarding buffered items.
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

    /// Consumes and yields items while the predicate returns `true`.
    ///
    /// This method consumes items from the iterator one by one and yields them
    /// as long as the predicate returns `true`. If the predicate fails,
    /// the item is pushed back into the front of the internal buffer for future use.
    ///
    /// # Arguments
    /// * `func` - A predicate function that returns `true` to continue consuming.
    ///
    /// # Returns
    /// An iterator over the consumed items that matched the predicate.
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

    /// Counts how many items satisfy the predicate without consuming them.
    ///
    /// This method peeks at the `n`-th item in the buffer using `peek_nth(count)`,
    /// starting from `n = 0`, and continues while the predicate returns `true`.
    /// The iteration stops at the first item that fails the predicate.
    ///
    /// # Arguments
    /// * `func` - A predicate to test each peeked item.
    ///
    /// # Returns
    /// The number of consecutive peeked elements that satisfy the predicate.
    pub fn while_peek(&mut self, mut func: impl FnMut(&I::Item) -> bool) -> usize {
        let mut count = 0;
        while let Some(item) = self.peek_nth(count) {
            if func(item) {
                count += 1;
            } else {
                break;
            }
        }

        count
    }
}

/// Creates a `PeekN` from a `Peekable` iterator, discarding its current peek state.
///
/// This is a lossy conversion that resets the peeking buffer.
///
/// # Note
/// Use `From<Peekable<I>>` if you want to retain the peeked value.
///
/// # Examples
/// ```
/// use std::iter::Peekable;
/// use peeknth::PeekN;
/// let peekable = (0..).peekable();
/// let peekn = PeekN::from_peekable_lossy(peekable);
/// ```
impl<I: Iterator> PeekN<Peekable<I>> {
    pub fn from_peekable_lossy(peekable: Peekable<I>) -> Self {
        PeekN::new(peekable)
    }
}

/// A convenient function to wrap an iterator into `PeekN`.
///
/// # Examples
/// ```
/// use peeknth::peekn;
/// let mut iter = peekn(0..);
/// assert_eq!(iter.peek(), Some(&0));
/// ```
pub fn peekn<I: Iterator>(iter: I) -> PeekN<I> {
    PeekN::new(iter)
}
