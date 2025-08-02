use crate::util::{Buffer, Either, PeekSource};

use core::{
    iter::{DoubleEndedIterator, FusedIterator, Peekable},
    ops::RangeBounds,
};

#[cfg(feature = "peekde")]
use crate::peekablede::PeekableDE;

#[cfg(feature = "peekn")]
use crate::peekn::SizedPeekN;

/// A double-ended peekable iterator with fixed-size front and back buffers.
///
/// `SizedPeekDN<I, const F: usize, const B: usize>` allows peeking from both the front and back
/// of a double-ended iterator, with up to `F` items in the front buffer and `B` in the back buffer.
///
/// This enables symmetric lookahead/rollback from either end, while bounding memory usage
/// and preventing unbounded buffering. Attempting to access beyond the configured sizes
/// will result in a panic.
///
/// # Type Parameters
/// - `I`: A double-ended iterator.
/// - `I::Item`: Must implement `Copy`, since items are stored by value in the buffer.
/// - `F`: Maximum number of front items that can be peeked.
/// - `B`: Maximum number of back items that can be peeked.
///
/// # Panics
/// Peeking past `F` front items or `B` back items using `peek_nth`, `peek_range`, or their
/// mutable equivalents will panic.
///
/// # Example
/// ```rust
/// use peeknth::sizedpeekdn;
/// let mut peekdn = sizedpeekdn::<_, 3, 2>(1..=5);
/// assert_eq!(peekdn.peek_front_nth(1), Some(&2));
/// assert_eq!(peekdn.peek_back_nth(1), Some(&4));
/// ```
pub struct SizedPeekDN<I, const F: usize, const B: usize>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    pub(crate) iter: I,
    pub(crate) front: Buffer<I::Item, F>,
    pub(crate) back: Buffer<I::Item, B>,
}

impl<I, const F: usize, const B: usize> DoubleEndedIterator for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back
            .pop_front()
            .or_else(|| self.iter.next_back())
            .or_else(|| self.front.pop_back())
    }
}

impl<I, const F: usize, const B: usize> Iterator for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.front
            .pop_front()
            .or_else(|| self.iter.next())
            .or_else(|| self.back.pop_back())
    }
}

#[cfg(feature = "peekn")]
impl<I, const F: usize, const B: usize> From<SizedPeekN<I, F>> for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    fn from(value: SizedPeekN<I, F>) -> Self {
        SizedPeekDN {
            iter: value.iter,
            front: value.buffer,
            back: Buffer::new(),
        }
    }
}

#[cfg(feature = "peekde")]
impl<I, const F: usize, const B: usize> From<PeekableDE<I>> for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    fn from(peekable_de: PeekableDE<I>) -> Self {
        let front = Buffer::from_iter(peekable_de.front.flatten());
        let back = Buffer::from_iter(peekable_de.back.flatten());
        SizedPeekDN {
            iter: peekable_de.iter,
            front,
            back,
        }
    }
}

impl<I, const F: usize, const B: usize> From<Peekable<I>> for SizedPeekDN<Peekable<I>, F, B>
where
    I: DoubleEndedIterator,
    I::Item: Clone + Copy,
{
    fn from(mut peekable: Peekable<I>) -> Self {
        let front = Buffer::from_iter(peekable.peek().cloned());

        SizedPeekDN {
            iter: peekable,
            front,
            back: Buffer::new(),
        }
    }
}

impl<I, const B: usize, const F: usize> Clone for SizedPeekDN<I, B, F>
where
    I: DoubleEndedIterator + Clone,
    I::Item: Copy + Clone,
{
    fn clone(&self) -> Self {
        SizedPeekDN {
            iter: self.iter.clone(),
            front: Clone::clone(&self.front),
            back: Clone::clone(&self.back),
        }
    }
}

impl<I, const F: usize, const B: usize> core::fmt::Debug for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator + core::fmt::Debug,
    I::Item: core::fmt::Debug + Copy,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SizedPeekDN")
            .field("iter", &self.iter)
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
    }
}

impl<I, const F: usize, const B: usize> ExactSizeIterator for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator + ExactSizeIterator,
    I::Item: Copy,
{
    fn len(&self) -> usize {
        self.iter.len() + self.front.len() + self.back.len()
    }
}
impl<I, const F: usize, const B: usize> FusedIterator for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator + FusedIterator,
    I::Item: Copy,
{
}

impl<I, const F: usize, const B: usize> Eq for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator + Eq,
    I::Item: Eq + Copy,
{
}

impl<I, const F: usize, const B: usize> PartialEq for SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator + PartialEq,
    I::Item: PartialEq + Copy,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter == other.iter && self.front == other.front && self.back == other.back
    }
}

impl<I, const B: usize, const F: usize> SizedPeekDN<I, B, F>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    pub fn new(iter: I) -> Self {
        SizedPeekDN {
            iter,
            front: Buffer::new(),
            back: Buffer::new(),
        }
    }

    /// Peeks at the `n`-th item from the front without consuming it.
    ///
    /// The element at index `n` will be returned if available and within the
    /// configured peek buffer size.
    ///
    /// # Arguments
    /// * `n` - Zero-based index from the front of the iterator.
    ///
    /// # Panics
    /// Panics if `n >= self.front_capacity()`.
    ///
    /// # Returns
    /// `Some(&item)` if the element exists within bounds, otherwise `None`.
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

    /// Peeks at the `n`-th item from the back without consuming it.
    ///
    /// The element at index `n` (from the back) will be returned if available and
    /// within the configured peek buffer size.
    ///
    /// # Arguments
    /// * `n` - Zero-based index from the back of the iterator.
    ///
    /// # Panics
    /// Panics if `n >= self.back_capacity()`.
    ///
    /// # Returns
    /// `Some(&item)` if the element exists within bounds, otherwise `None`.
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

    /// Mutably peeks at the `n`-th front element without consuming it.
    ///
    /// Internally fills the front buffer up to index `n` if necessary.
    ///
    /// # Panics
    /// Panics if `n >= self.front_capacity()`.
    ///
    /// # Returns
    /// `Some(&mut item)` if available, otherwise `None`.
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

    /// Mutably peeks at the `n`-th back element without consuming it.
    ///
    /// Internally fills the back buffer up to index `n` if necessary.
    ///
    /// # Panics
    /// Panics if `n >= self.back_capacity()`.
    ///
    /// # Returns
    /// `Some(&mut item)` if available, otherwise `None`.
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

    /// Peeks a range of items from the front without consuming them.
    ///
    /// This returns a slice of references to elements within the given range.
    /// The range must be within the configured front peek buffer.
    ///
    /// # Arguments
    /// * `range` - Range of indices (e.g., `0..3`, `2..=4`) from the front.
    ///
    /// # Panics
    /// Panics if `range.end > self.front_capacity()`
    ///
    /// # Returns
    /// A slice of peeked items if available, otherwise an empty slice.
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
        for i in start..end {
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

    /// Peeks a range of items from the back without consuming them.
    ///
    /// This returns a slice of references to elements within the given range.
    /// The range must be within the configured back peek buffer.
    ///
    /// # Arguments
    /// * `range` - Range of indices (e.g., `0..3`, `2..=4`) from the back.
    ///
    /// # Panics
    /// Panics if `range.end > self.back_capacity()`
    ///
    /// # Returns
    /// A slice of peeked items if available, otherwise an empty slice.
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

    /// Mutably peeks a range of items from the front without consuming them.
    ///
    /// Returns a mutable slice of elements in the specified range from the front buffer.
    /// The range must lie entirely within the configured front peek capacity.
    ///
    /// # Arguments
    /// * `range` - A range of indices from the front (e.g., `0..3`, `2..=4`).
    ///
    /// # Panics
    /// Panics if the upper bound of the range is greater than `self.front_capacity()`.
    ///
    /// # Returns
    /// A mutable slice of peeked items in the specified front range.
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

    /// Mutably peeks a range of items from the back without consuming them.
    ///
    /// Returns a mutable slice of elements in the specified range from the back buffer.
    /// The range must lie entirely within the configured back peek capacity.
    ///
    /// # Arguments
    /// * `range` - A range of indices from the back (e.g., `0..3`, `2..=4`).
    ///
    /// # Panics
    /// Panics if the upper bound of the range is greater than `self.back_capacity()`.
    ///
    /// # Returns
    /// A mutable slice of peeked items in the specified back range.
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

    /// Consumes and yields items from the front as long as the predicate returns `true`.
    ///
    /// This method pulls items from the front of the iterator and yields them
    /// while the predicate returns `true`. Once the predicate fails,
    /// the current item is pushed back to the front buffer for future access.
    ///
    /// # Arguments
    /// * `func` - A predicate that returns `true` to continue yielding, or `false` to stop.
    ///
    /// # Returns
    /// An iterator of items from the front that matched the predicate.
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

    /// Consumes and yields items from the back as long as the predicate returns `true`.
    ///
    /// This method pulls items from the back of the iterator and yields them
    /// while the predicate returns `true`. If the predicate fails,
    /// the current item is pushed back to the back buffer.
    ///
    /// # Arguments
    /// * `func` - A predicate that returns `true` to continue yielding, or `false` to stop.
    ///
    /// # Returns
    /// An iterator of items from the back that matched the predicate.
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

    /// Peeks forward without consuming while the predicate returns `true`.
    ///
    /// Repeatedly peeks at the `n`-th front item and counts how many elements
    /// satisfy the predicate without consuming them.
    ///
    /// # Arguments
    /// * `func` - A predicate to test each peeked item.
    ///
    /// # Returns
    /// The number of front items that match the predicate.
    pub fn while_peek_front(&mut self, mut func: impl FnMut(&I::Item) -> bool) -> usize {
        let mut count = 0;

        let limit = self.front_capacity();

        while let Some(item) = self.peek_front_nth(count) {
            if func(item) && count < limit {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    /// Peeks backward without consuming while the predicate returns `true`.
    ///
    /// Repeatedly peeks at the `n`-th back item and counts how many elements
    /// satisfy the predicate without consuming them.
    ///
    /// # Arguments
    /// * `func` - A predicate to test each peeked item.
    ///
    /// # Returns
    /// The number of back items that match the predicate.
    pub fn while_peek_back(&mut self, mut func: impl FnMut(&I::Item) -> bool) -> usize {
        let mut count = 0;
        let limit = self.back_capacity();

        while let Some(item) = self.peek_back_nth(count) {
            if func(item) && count < limit {
                count += 1;
            } else {
                break;
            }
        }

        count
    }

    /// Returns the maximum number of elements that can be peeked from the front.
    ///
    /// This value represents the logical upper limit of peekable elements from the front,
    /// as configured by the user. Attempting to `peek_front_nth(n)` with `n >= front_capacity()`
    /// will panic.
    ///
    /// # Returns
    /// The configured maximum number of peekable front elements.
    #[inline(always)]
    pub fn front_capacity(&self) -> usize {
        self.front.capacity()
    }

    /// Returns the maximum number of elements that can be peeked from the back.
    ///
    /// This value represents the logical upper limit of peekable elements from the back,
    /// as configured by the user. Attempting to `peek_back_nth(n)` with `n >= back_capacity()`
    /// will panic.
    ///
    /// # Returns
    /// The configured maximum number of peekable back elements.
    #[inline(always)]
    pub fn back_capacity(&self) -> usize {
        self.back.capacity()
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
impl<I, const B: usize, const F: usize> SizedPeekDN<Peekable<I>, B, F>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    /// Constructs `SizedPeekDN` from a `Peekable` iterator, discarding peeked state.
    pub fn from_peekable_lossy(peekable: Peekable<I>) -> Self {
        SizedPeekDN::new(peekable)
    }
}

/// Creates a new `SizedPeekDN` from a given iterator with statically defined buffer sizes.
///
/// This is a shorthand for `SizedPeekDN::new(...)`, typically used with const generics.
///
/// # Type Parameters
/// * `F` - Maximum number of front elements to peek.
/// * `B` - Maximum number of back elements to peek.
///
/// # Arguments
/// * `iter` - A double-ended iterator to wrap.
///
/// # Returns
/// A new `SizedPeekDN<I, F, B>` instance.
pub fn sizedpeekdn<I, const F: usize, const B: usize>(iter: I) -> SizedPeekDN<I, F, B>
where
    I: DoubleEndedIterator,
    I::Item: Copy,
{
    SizedPeekDN::new(iter)
}
