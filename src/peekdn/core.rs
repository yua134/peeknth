#[cfg(feature = "peekn")]
use crate::peekn::{PeekN, SizedPeekN};

extern crate alloc;

use alloc::collections::VecDeque;

use core::{
    iter::{DoubleEndedIterator, FusedIterator, Peekable},
    ops::RangeBounds,
};

use crate::{
    SizedPeekDN,
    util::{Either, PeekSource},
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
    fn from(value: PeekN<I>) -> Self {
        PeekDN {
            iter: value.iter,
            front: value.buffer,
            back: VecDeque::new(),
        }
    }
}

impl<I, const S: usize> From<SizedPeekN<I, S>> for PeekDN<I>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    fn from(value: SizedPeekN<I, S>) -> Self {
        let front = VecDeque::from(value.buffer);
        PeekDN {
            iter: value.iter,
            front,
            back: VecDeque::new(),
        }
    }
}

impl<I, const F: usize, const B: usize> From<SizedPeekDN<I, F, B>> for PeekDN<I>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    fn from(value: SizedPeekDN<I, F, B>) -> Self {
        PeekDN {
            iter: value.iter,
            front: VecDeque::from(value.front),
            back: VecDeque::from(value.back),
        }
    }
}

#[cfg(feature = "peekde")]
impl<I: DoubleEndedIterator> From<PeekableDE<I>> for PeekDN<I> {
    fn from(value: PeekableDE<I>) -> Self {
        let front = value.front.flatten().into_iter().collect();
        let back = value.back.flatten().into_iter().collect();
        PeekDN {
            iter: value.iter,
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
            iter: peekable,
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
            front: Clone::clone(&self.front),
            back: Clone::clone(&self.back),
        }
    }
}

impl<I> core::fmt::Debug for PeekDN<I>
where
    I: DoubleEndedIterator + core::fmt::Debug,
    I::Item: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PeekDN")
            .field("iter", &self.iter)
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
    }
}

impl<I: DoubleEndedIterator + ExactSizeIterator> ExactSizeIterator for PeekDN<I> {
    fn len(&self) -> usize {
        self.iter.len() + self.front.len() + self.back.len()
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

impl<I: DoubleEndedIterator + FusedIterator> FusedIterator for PeekDN<I> {}

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
    ///
    /// # Example
    /// ```
    /// use peeknth::peekdn;
    ///
    /// let mut iter = peekdn(0..=5);
    ///
    /// assert_eq!(iter.peek_front_nth(0), Some(&0));
    /// assert_eq!(iter.peek_front_nth(1), Some(&1));
    /// assert_eq!(iter.peek_front_nth(4), Some(&4));
    /// assert_eq!(iter.peek_front_nth(10), None);
    /// ```
    pub fn peek_front_nth(&mut self, n: usize) -> Option<&I::Item> {
        core::debug_assert!(
            n < usize::MAX,
            "peek_front_nth() with usize::MAX is likely a bug"
        );

        if self.front.len() > n {
            return self.front.get(n);
        }

        while self.front.len() <= n {
            match self.iter.next() {
                Some(item) => self.front.push_back(item),
                None => {
                    return self.back.get((self.back.len() + self.front.len()).checked_sub(n + 1)?);
                }
            }
        }

        self.front.get(n)
    }

    /// Peeks at the `n`-th element from the back without consuming it.
    ///
    /// Internally fills the back buffer up to index `n` as needed.
    ///
    /// Returns `None` if the iterator ends before reaching index `n`.
    ///
    /// # Example
    /// ```
    /// use peeknth::peekdn;
    ///
    /// let mut iter = peekdn(0..=5);
    ///
    /// assert_eq!(iter.peek_back_nth(0), Some(&5));
    ///
    /// iter.peek_front_nth(0);
    ///
    /// assert_eq!(iter.peek_back_nth(5), Some(&0));
    /// ```
    pub fn peek_back_nth(&mut self, n: usize) -> Option<&I::Item> {
        core::debug_assert!(
            n < usize::MAX,
            "peek_back_nth() with usize::MAX is likely a bug"
        );

        if self.back.len() > n {
            return self.back.get(n);
        }

        while self.back.len() <= n {
            match self.iter.next_back() {
                Some(item) => self.back.push_back(item),
                None => {
                    return self.front.get((self.back.len() + self.front.len()).checked_sub(n + 1)?);
                }
            }
        }

        self.back.get(n)
    }

    /// Mutably peeks at the `n`-th element from the front.
    pub fn peek_front_nth_mut(&mut self, n: usize) -> Option<&mut I::Item> {
        core::debug_assert!(
            n < usize::MAX,
            "peek_front_nth_mut() with usize::MAX is likely a bug"
        );

        if self.front.len() > n {
            return self.front.get_mut(n);
        }

        while self.front.len() <= n {
            match self.iter.next() {
                Some(item) => self.front.push_back(item),
                None => {
                    return self
                        .back
                        .get_mut((self.back.len() + self.front.len()).checked_sub(n + 1)?);
                }
            }
        }

        self.front.get_mut(n)
    }

    /// Mutably peeks at the `n`-th element from the back.
    pub fn peek_back_nth_mut(&mut self, n: usize) -> Option<&mut I::Item> {
        core::debug_assert!(
            n < usize::MAX,
            "peek_back_nth_mut() with usize::MAX is likely a bug"
        );

        if self.back.len() > n {
            return self.back.get_mut(n);
        }

        while self.back.len() <= n {
            match self.iter.next_back() {
                Some(item) => self.back.push_back(item),
                None => {
                    return self
                        .front
                        .get_mut((self.back.len() + self.front.len()).checked_sub(n + 1)?);
                }
            }
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
    ) -> impl DoubleEndedIterator<Item = &I::Item>
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
            return Either::Single(self.front.range(0..0));
        }

        let mut actual_end = 0;
        for i in 0..end {
            if self.peek_front_nth(i).is_none() {
                break;
            }
            actual_end += 1;
        }

        let len = self.front.len();
        if actual_end <= len {
            Either::Single(self.front.range(start..actual_end))
        } else {
            let back = self.back.len();
            let from = (back + len).saturating_sub(actual_end);
            Either::Chain(
                self.front
                    .range(start..len)
                    .chain(self.back.range(from..back).rev()),
            )
        }
    }

    pub fn peek_front_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = &mut I::Item>
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
            return Either::Single(self.front.range_mut(0..0));
        }

        let mut actual_end = 0;
        for i in start..end {
            if self.peek_front_nth(i).is_none() {
                break;
            }
            actual_end += 1;
        }

        let len = self.front.len();
        if actual_end <= len {
            Either::Single(self.front.range_mut(start..actual_end))
        } else {
            let back = self.back.len();
            let from = (back + len).saturating_sub(actual_end);
            Either::Chain(
                self.front
                    .range_mut(start..len)
                    .chain(self.back.range_mut(from..back).rev()),
            )
        }
    }

    /// Peeks at a range of elements from the back.
    ///
    /// Fills the back buffer as needed. The range is inclusive-exclusive (`start..end`).
    pub fn peek_back_range<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = &I::Item>
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
            return Either::Single(self.back.range(0..0));
        }

        let mut actual_end = 0;
        for i in start..end {
            if self.peek_back_nth(i).is_none() {
                break;
            }
            actual_end += 1;
        }

        let len = self.back.len();
        if actual_end <= len {
            Either::Single(self.back.range(start..actual_end))
        } else {
            let front = self.front.len();
            let from = (front + len).saturating_sub(actual_end);
            Either::Chain(
                self.back
                    .range(start..len)
                    .chain(self.front.range(from..front).rev()),
            )
        }
    }

    pub fn peek_back_range_mut<R: RangeBounds<usize>>(
        &mut self,
        range: R,
    ) -> impl DoubleEndedIterator<Item = &mut I::Item>
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
            return Either::Single(self.back.range_mut(0..0));
        }

        let mut actual_end = 0;
        for i in start..end {
            if self.peek_back_nth(i).is_none() {
                break;
            }
            actual_end += 1;
        }

        let len = self.back.len();
        if actual_end <= len {
            Either::Single(self.back.range_mut(start..actual_end))
        } else {
            let front = self.front.len();
            let from = (front + len).saturating_sub(actual_end);
            Either::Chain(
                self.back
                    .range_mut(start..len)
                    .chain(self.front.range_mut(from..front).rev()),
            )
        }
    }

    /// Consumes and returns the next item only if it satisfies the predicate.
    ///
    /// If the predicate fails, the item is pushed back to the front buffer.
    pub fn next_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        if let Some(matched) = self.next_with_source() {
            if func(matched.as_ref()) {
                Some(matched.into_item())
            } else {
                self.cache_front(matched);
                None
            }
        } else {
            None
        }
    }

    /// Consumes and returns the next item from the back only if it satisfies the predicate.
    ///
    /// If the predicate fails, the item is pushed back to the back buffer.
    pub fn next_back_if(&mut self, func: impl FnOnce(&I::Item) -> bool) -> Option<I::Item> {
        if let Some(matched) = self.next_back_with_source() {
            if func(matched.as_ref()) {
                Some(matched.into_item())
            } else {
                self.cache_back(matched);
                None
            }
        } else {
            None
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
    pub fn into_peekable_lossy(self) -> Peekable<PeekDN<I>> {
        self.into_iter().peekable()
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
        core::debug_assert!(
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
        core::debug_assert!(
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

    /// Consumes and yields items from the front while the predicate returns `true`.
    ///
    /// This method repeatedly calls `next()` and yields the item if the predicate returns `true`.
    /// If the predicate fails, the item is cached back to the front buffer and iteration stops.
    ///
    /// # Arguments
    /// * `func` - A predicate that returns `true` to continue consuming, or `false` to stop.
    ///
    /// # Returns
    /// An `Iterator<Item = I::Item>` of matched items from the front.
    pub fn while_next_front(
        &mut self,
        mut func: impl FnMut(&I::Item) -> bool,
    ) -> impl Iterator<Item = I::Item> {
        core::iter::from_fn(move || {
            if let Some(peeked) = self.next_with_source() {
                if func(peeked.as_ref()) {
                    Some(peeked.into_item())
                } else {
                    self.cache_front(peeked);
                    None
                }
            } else {
                None
            }
        })
    }

    /// Consumes and yields items from the back while the predicate returns `true`.
    ///
    /// This method repeatedly calls `next_back()` and yields the item if the predicate returns `true`.
    /// If the predicate fails, the item is cached back to the back buffer and iteration stops.
    ///
    /// # Arguments
    /// * `func` - A predicate that returns `true` to continue consuming, or `false` to stop.
    ///
    /// # Returns
    /// An `Iterator<Item = I::Item>` of matched items from the back.
    pub fn while_next_back(
        &mut self,
        mut func: impl FnMut(&I::Item) -> bool,
    ) -> impl Iterator<Item = I::Item> {
        core::iter::from_fn(move || {
            if let Some(peeked) = self.next_back_with_source() {
                if func(peeked.as_ref()) {
                    Some(peeked.into_item())
                } else {
                    self.cache_back(peeked);
                    None
                }
            } else {
                None
            }
        })
    }

    /// Counts how many items from the front satisfy the predicate without consuming them.
    ///
    /// Peeks the `n`-th front element repeatedly and stops when the predicate returns `false`
    /// or there are no more elements to peek.
    ///
    /// # Arguments
    /// * `func` - A predicate to test each peeked item.
    ///
    /// # Returns
    /// The number of items from the front that satisfy the predicate.
    pub fn while_peek_front(&mut self, mut func: impl FnMut(&I::Item) -> bool) -> usize {
        let mut count = 0;

        while let Some(item) = self.peek_front_nth(count) {
            if func(item) {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    /// Counts how many items from the back satisfy the predicate without consuming them.
    ///
    /// Peeks the `n`-th back element repeatedly and stops when the predicate returns `false`
    /// or there are no more elements to peek.
    ///
    /// # Arguments
    /// * `func` - A predicate to test each peeked item.
    ///
    /// # Returns
    /// The number of items from the back that satisfy the predicate.
    pub fn while_peek_back(&mut self, mut func: impl FnMut(&I::Item) -> bool) -> usize {
        let mut count = 0;

        while let Some(item) = self.peek_back_nth(count) {
            if func(item) {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    fn next_with_source(&mut self) -> Option<PeekSource<I::Item>> {
        if let Some(front) = self.front.pop_front() {
            Some(PeekSource::Front(front))
        } else if let Some(iter) = self.iter.next() {
            Some(PeekSource::Iter(iter))
        } else {
            self.back.pop_back().map(PeekSource::Back)
        }
    }

    fn cache_front(&mut self, item: PeekSource<I::Item>) {
        match item {
            PeekSource::Front(front) => self.front.push_back(front),
            PeekSource::Iter(iter) => self.front.push_back(iter),
            PeekSource::Back(back) => self.back.push_front(back),
        }
    }

    fn next_back_with_source(&mut self) -> Option<PeekSource<I::Item>> {
        if let Some(back) = self.back.pop_front() {
            Some(PeekSource::Back(back))
        } else if let Some(iter) = self.iter.next_back() {
            Some(PeekSource::Iter(iter))
        } else {
            self.front.pop_back().map(PeekSource::Front)
        }
    }

    fn cache_back(&mut self, item: PeekSource<I::Item>) {
        match item {
            PeekSource::Front(front) => self.front.push_front(front),
            PeekSource::Iter(iter) => self.back.push_back(iter),
            PeekSource::Back(back) => self.back.push_back(back),
        }
    }
}

impl<I: DoubleEndedIterator> PeekDN<Peekable<I>> {
    /// Constructs `PeekDN` from a `Peekable` iterator, discarding peeked state.
    pub fn from_peekable_lossy(peekable: Peekable<I>) -> Self {
        PeekDN::new(peekable)
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
