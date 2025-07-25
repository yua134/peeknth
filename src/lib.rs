use std::{
    collections::VecDeque,
    ops::{Bound, RangeBounds},
};

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
    iter: I,
    buffer: VecDeque<I::Item>,
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
    /// # Examples
    /// ```
    /// # use peeknth::{peekn, PeekN};
    /// let mut iter = peekn(10..);
    /// assert_eq!(iter.peek_nth(3), Some(&13));
    /// ```
    pub fn peek_nth(&mut self, n: usize) -> Option<&I::Item> {
        while self.buffer.len() <= n {
            let next_item = self.iter.next()?;
            self.buffer.push_back(next_item);
        }

        self.buffer.get(n)
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

    /// Peeks at a range of elements from the iterator and returns them as references.
    ///
    /// The internal buffer is filled up to the highest required index if needed.
    ///
    /// # Examples
    /// ```
    /// # use peeknth::{peekn, PeekN};
    /// let mut iter = peekn(0..);
    /// let values: Vec<_> = iter.peek_range(1..4).cloned().collect();
    /// assert_eq!(values, vec![1, 2, 3]);
    /// ```
    pub fn peek_range<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl Iterator<Item = &<I as Iterator>::Item> {
        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n.saturating_add(1),
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n.saturating_add(1),
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.peeked_len(),
        };

        if start >= end {
            return self.buffer.range(0..0);
        }

        for i in start..end {
            if self.peek_nth(i).is_none() {
                break;
            }
        }

        let safe_end = end.min(self.buffer.len());
        self.buffer.range(start..safe_end)
    }

    /// Returns the number of items currently buffered (peeked but not consumed).
    pub fn peeked_len(&self) -> usize {
        self.buffer.len()
    }

    /// Returns `true` if the buffer contains at least `n + 1` items.
    pub fn has_peeked(&self, n: usize) -> bool {
        self.peeked_len() > n
    }

    /// Clears all buffered items.
    pub fn clear_peeked(&mut self) {
        self.buffer.clear();
    }

    /// Discards the first `until` buffered items.
    pub fn drain_peeked(&mut self, until: usize) {
        let until = until.min(self.buffer.len());
        self.buffer.drain(..until);
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
