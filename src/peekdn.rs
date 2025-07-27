#[cfg(feature = "peekn")]
use crate::peekn::PeekN;
use std::{
    collections::VecDeque,
    iter::{FusedIterator, Peekable},
    ops::RangeBounds,
};

#[cfg(feature = "peekde")]
use crate::peekablede::PeekableDE;

/// `PeekDN` is an iterator adapter that enables double-ended peeking.
///
/// Unlike `Peekable`, which only allows peeking at the next item from the front,
/// `PeekDN` allows peeking from both the front and back of a `DoubleEndedIterator`.
///
/// It maintains internal buffers for both directions and can conditionally consume items.
///
/// # Features
///
/// - Peek at arbitrary indices from front or back
/// - Mutable peek access
/// - Conditional consumption (`next_if`, `next_back_if`)
/// - Supports `From` conversions from other peekable types (e.g. `PeekN`, `PeekableDE`)
///
/// # Examples
///
/// ```
/// use peeknth::peekdn;
///
/// let mut iter = peekdn(1..=5);
///
/// assert_eq!(iter.peek_front(), Some(&1));
/// assert_eq!(iter.peek_back(), Some(&5));
///
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next_back(), Some(5));
/// ```
pub struct PeekDN<I: DoubleEndedIterator> {
    pub(crate) iter: I,
    pub(crate) front: VecDeque<I::Item>,
    pub(crate) back: VecDeque<I::Item>,
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for PeekDN<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back
            .pop_front()
            .or_else(|| self.iter.next_back())
            .or_else(|| self.front.pop_back())
    }
}

impl<I: DoubleEndedIterator> Iterator for PeekDN<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.front
            .pop_front()
            .or_else(|| self.iter.next())
            .or_else(|| self.back.pop_back())
    }
}

#[cfg(feature = "peekn")]
impl<I: DoubleEndedIterator> From<PeekN<I>> for PeekDN<I> {
    fn from(peekn: PeekN<I>) -> Self {
        PeekDN {
            iter: peekn.iter,
            front: peekn.buffer,
            back: VecDeque::new(),
        }
    }
}

#[cfg(feature = "peekde")]
impl<I: DoubleEndedIterator> From<PeekableDE<I>> for PeekDN<I> {
    fn from(peekable_de: PeekableDE<I>) -> Self {
        let front = peekable_de.front.flatten().into_iter().collect();
        let back = peekable_de.back.flatten().into_iter().collect();
        PeekDN {
            iter: peekable_de.iter,
            front,
            back,
        }
    }
}

impl<I> From<Peekable<I>> for PeekDN<Peekable<I>>
where
    I: DoubleEndedIterator,
    I::Item: Clone,
{
    fn from(mut peekable: Peekable<I>) -> Self {
        let front = peekable.peek().cloned().into_iter().collect();

        PeekDN {
            iter: peekable.into_iter(),
            front,
            back: VecDeque::new(),
        }
    }
}

impl<I> Clone for PeekDN<I>
where
    I: DoubleEndedIterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        PeekDN {
            iter: self.iter.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
        }
    }
}

impl<I> std::fmt::Debug for PeekDN<I>
where
    I: DoubleEndedIterator + std::fmt::Debug,
    I::Item: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PeekDN")
            .field("iter", &self.iter)
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
    }
}

impl<I> Eq for PeekDN<I>
where
    I: DoubleEndedIterator + Eq,
    I::Item: Eq,
{
}

impl<I> PartialEq for PeekDN<I>
where
    I: DoubleEndedIterator + PartialEq,
    I::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter == other.iter && self.front == other.front && self.back == other.back
    }
}

impl<I: ExactSizeIterator + DoubleEndedIterator> ExactSizeIterator for PeekDN<I> {
    fn len(&self) -> usize {
        self.iter.len() + self.front.len() + self.back.len()
    }
}

impl<I: FusedIterator + DoubleEndedIterator> std::iter::FusedIterator for PeekDN<I> {}

impl<I: DoubleEndedIterator> PeekDN<I> {
    /// Creates a new `PeekDN` from the given `DoubleEndedIterator`.
    pub fn new(iter: I) -> Self {
        PeekDN {
            iter,
            front: VecDeque::new(),
            back: VecDeque::new(),
        }
    }

    /// Creates a new `PeekDN` with preallocated front and back buffer capacities.
    ///
    /// # Arguments
    ///
    /// - `iter`: The iterator to wrap.
    /// - `front`: Initial capacity for the front peek buffer.
    /// - `back`: Initial capacity for the back peek buffer.
    pub fn with_capacity(iter: I, front: usize, back: usize) -> Self {
        PeekDN {
            iter,
            front: VecDeque::with_capacity(front),
            back: VecDeque::with_capacity(back),
        }
    }

    /// Peeks at the `n`-th element from the front without consuming it.
    ///
    /// Internally fills the front buffer up to index `n` as needed.
    ///
    /// Returns `None` if the iterator ends before reaching index `n`.
    pub fn peek_front_nth(&mut self, n: usize) -> Option<&I::Item> {
        debug_assert!(
            n < usize::MAX,
            "peek_front_nth() with usize::MAX is likely a bug"
        );

        while self.front.len() <= n {
            let front_item = self.iter.next()?;
            self.front.push_back(front_item);
        }
        self.front.get(n)
    }

    /// Peeks at the `n`-th element from the back without consuming it.
    ///
    /// Internally fills the back buffer up to index `n` as needed.
    ///
    /// Returns `None` if the iterator ends before reaching index `n`.
    pub fn peek_back_nth(&mut self, n: usize) -> Option<&I::Item> {
        debug_assert!(
            n < usize::MAX,
            "peek_back_nth() with usize::MAX is likely a bug"
        );

        while self.back.len() <= n {
            let back_item = self.iter.next_back()?;
            self.back.push_back(back_item);
        }
        self.back.get(n)
    }

    /// Mutably peeks at the `n`-th element from the front.
    pub fn peek_front_nth_mut(&mut self, n: usize) -> Option<&mut I::Item> {
        debug_assert!(
            n < usize::MAX,
            "peek_front_nth_mut() with usize::MAX is likely a bug"
        );

        while self.front.len() <= n {
            let next_item = self.iter.next()?;
            self.front.push_back(next_item);
        }

        self.front.get_mut(n)
    }

    /// Mutably peeks at the `n`-th element from the back.
    pub fn peek_back_nth_mut(&mut self, n: usize) -> Option<&mut I::Item> {
        debug_assert!(
            n < usize::MAX,
            "peek_back_nth_mut() with usize::MAX is likely a bug"
        );

        while self.back.len() <= n {
            let next_item = self.iter.next_back()?;
            self.back.push_back(next_item);
        }

        self.back.get_mut(n)
    }

    /// Peeks at the next front item (same as `peek_front_nth(0)`).
    pub fn peek_front(&mut self) -> Option<&I::Item> {
        self.peek_front_nth(0)
    }

    /// Peeks at the next back item (same as `peek_back_nth(0)`).
    pub fn peek_back(&mut self) -> Option<&I::Item> {
        self.peek_back_nth(0)
    }

    /// Mutably peeks at the next front item.
    pub fn peek_front_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_front_nth_mut(0)
    }

    /// Mutably peeks at the next back item.
    pub fn peek_back_mut(&mut self) -> Option<&mut I::Item> {
        self.peek_back_nth_mut(0)
    }

    /// Peeks at a range of elements from the front.
    ///
    /// Fills the front buffer as needed. The range is inclusive-exclusive (`start..end`).
    pub fn peek_front_range<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl Iterator<Item = &I::Item> {
        use std::ops::Bound;

        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.front.len(),
        };

        debug_assert!(
            start < end,
            "peek_range: start ({start}) must be less than end ({end})"
        );

        if start >= end {
            return self.front.range(0..0);
        }

        for i in start..end {
            if self.peek_front_nth(i).is_none() {
                break;
            }
        }

        let safe_end = end.min(self.front.len());
        debug_assert!(
            end <= self.front.len(),
            "peek_range: end out of bounds: {} > {}",
            end,
            self.front.len()
        );
        self.front.range(start..safe_end)
    }

    /// Peeks at a range of elements from the back.
    ///
    /// Fills the back buffer as needed. The range is inclusive-exclusive (`start..end`).
    pub fn peek_back_range<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl Iterator<Item = &I::Item> {
        use std::ops::Bound;

        let start = match range.start_bound() {
            Bound::Included(&n) => n,
            Bound::Excluded(&n) => n + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&n) => n + 1,
            Bound::Excluded(&n) => n,
            Bound::Unbounded => self.back.len(),
        };

        debug_assert!(
            start < end,
            "peek_range: start ({start}) must be less than end ({end})"
        );

        if start >= end {
            return self.back.range(0..0);
        }

        for i in start..end {
            if self.peek_back_nth(i).is_none() {
                break;
            }
        }

        let safe_end = end.min(self.back.len());
        debug_assert!(
            end <= self.back.len(),
            "peek_range: end out of bounds: {} > {}",
            end,
            self.back.len()
        );
        self.back.range(start..safe_end)
    }

    /// Consumes and returns the next item only if it satisfies the predicate.
    ///
    /// If the predicate fails, the item is pushed back to the front buffer.
    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        match self.next() {
            Some(matched) if func(&matched) => Some(matched),
            Some(other) => {
                self.front.push_front(other);
                None
            }
            None => None,
        }
    }

    /// Consumes and returns the next item from the back only if it satisfies the predicate.
    ///
    /// If the predicate fails, the item is pushed back to the back buffer.
    pub fn next_back_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        match self.next_back() {
            Some(matched) if func(&matched) => Some(matched),
            Some(other) => {
                self.back.push_front(other);
                None
            }
            None => None,
        }
    }

    /// Consumes and returns the next front item if it equals `expected`.
    ///
    /// Otherwise, the item is pushed back to the buffer.
    pub fn next_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_if(|next| next == expected)
    }

    /// Consumes and returns the next back item if it equals `expected`.
    pub fn next_back_if_eq<T>(&mut self, expected: &T) -> Option<I::Item>
    where
        T: ?Sized,
        I::Item: PartialEq<T>,
    {
        self.next_back_if(|next| next == expected)
    }

    /// Converts `PeekDN` into a standard `Peekable`, discarding peeked items.
    ///
    /// This is a lossy conversion.
    pub fn into_peekable_lossy(self) -> Peekable<I> {
        self.iter.peekable()
    }

    /// Clears all front-peeked elements.
    #[inline]
    pub fn clear_front_peeked(&mut self) {
        self.front.clear();
    }

    /// Clears all back-peeked elements.
    #[inline]
    pub fn clear_back_peeked(&mut self) {
        self.back.clear();
    }

    /// Clears all peeked elements from both ends.
    #[inline]
    pub fn clear_peeked(&mut self) {
        self.clear_front_peeked();
        self.clear_back_peeked();
    }

    /// Returns the number of items in the front peek buffer.
    #[inline]
    pub fn front_peeked_len(&self) -> usize {
        self.front.len()
    }

    /// Returns the number of items in the back peek buffer.
    #[inline]
    pub fn back_peeked_len(&self) -> usize {
        self.back.len()
    }

    /// Returns `true` if there are at least `n + 1` items peeked from the front.
    #[inline]
    pub fn has_front_peeked(&self, n: usize) -> bool {
        self.front_peeked_len() > n
    }

    /// Returns `true` if there are at least `n + 1` items peeked from the back.
    #[inline]
    pub fn has_back_peeked(&self, n: usize) -> bool {
        self.back_peeked_len() > n
    }

    /// Removes up to `until` items from the front peek buffer.
    #[inline]
    pub fn drain_front_peeked(&mut self, until: usize) {
        let until = until.min(self.front.len());
        debug_assert!(
            until <= self.front.len(),
            "drain_peeked: requested to drain until {} but buffer length is {}",
            until,
            self.front.len()
        );
        self.front.drain(..until);
    }

    /// Removes up to `until` items from the back peek buffer.
    #[inline]
    pub fn drain_back_peeked(&mut self, until: usize) {
        debug_assert!(
            until <= self.back.len(),
            "drain_peeked: requested to drain until {} but buffer length is {}",
            until,
            self.back.len()
        );
        let until = until.min(self.back.len());
        self.back.drain(..until);
    }

    /// Drains both front and back peek buffers up to the given limits.
    #[inline]
    pub fn drain_peeked(&mut self, front_until: usize, back_until: usize) {
        self.drain_front_peeked(front_until);
        self.drain_back_peeked(back_until);
    }
}

impl<I: DoubleEndedIterator> PeekDN<Peekable<I>> {
    /// Constructs `PeekDN` from a `Peekable` iterator, discarding peeked state.
    pub fn from_peekable_lossy(peekable: Peekable<I>) -> Self {
        PeekDN::new(peekable.into_iter())
    }
}

/// A Convenient function to wrap an iterator into `PeekDN`.
///
/// # Examples
/// ```
/// use peeknth::peekdn;
/// let mut iter = peekdn(0..=3);
/// assert_eq!(iter.peek_front(), Some(&0));
/// ```
pub fn peekdn<I: DoubleEndedIterator>(iter: I) -> PeekDN<I> {
    PeekDN::new(iter)
}
