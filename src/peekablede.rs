use core::iter::{FusedIterator, Peekable};

use crate::util::PeekSource;

#[cfg(feature = "peekn")]
use crate::PeekN;

#[cfg(feature = "peekdn")]
use crate::PeekDN;

/// `PeekableDE` is an iterator adapter that allows peeking both forward and backward
/// in a `DoubleEndedIterator`, similar to `Peekable` but bidirectional.
///
/// It supports:
/// - Peeking the next item from the front (`peek_front`)
/// - Peeking the next item from the back (`peek_back`)
/// - Mutable peeking from either end
/// - Conditional consumption via `next_if`, `next_back_if`, etc.
/// - Conversions from `PeekN` and `PeekDN` (if features enabled)
///
/// # Examples
/// ```
/// # use peeknth::PeekableDE;
/// let mut iter = PeekableDE::new(1..=3);
/// assert_eq!(iter.peek_front(), Some(&1));
/// assert_eq!(iter.peek_back(), Some(&3));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next_back(), Some(3));
/// ```
pub struct PeekableDE<I: DoubleEndedIterator> {
    pub(crate) iter: I,
    pub(crate) front: Option<Option<I::Item>>,
    pub(crate) back: Option<Option<I::Item>>,
}

impl<I: DoubleEndedIterator> Iterator for PeekableDE<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.front
            .take()
            .flatten()
            .or_else(|| self.iter.next())
            .or_else(|| self.back.take().flatten())
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for PeekableDE<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back
            .take()
            .flatten()
            .or_else(|| self.iter.next_back())
            .or_else(|| self.front.take().flatten())
    }
}

impl<I> Clone for PeekableDE<I>
where
    I: DoubleEndedIterator + Clone,
    I::Item: Clone,
{
    fn clone(&self) -> Self {
        PeekableDE {
            iter: self.iter.clone(),
            front: self.front.clone(),
            back: self.back.clone(),
        }
    }
}

impl<I> core::fmt::Debug for PeekableDE<I>
where
    I: DoubleEndedIterator + core::fmt::Debug,
    I::Item: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PeekableDE")
            .field("iter", &self.iter)
            .field("front", &self.front)
            .field("back", &self.back)
            .finish()
    }
}

impl<I: ExactSizeIterator + DoubleEndedIterator> ExactSizeIterator for PeekableDE<I> {
    fn len(&self) -> usize {
        self.iter.len()
            + matches!(self.front, Some(Some(_))) as usize
            + matches!(self.back, Some(Some(_))) as usize
    }
}

impl<I> Eq for PeekableDE<I>
where
    I: DoubleEndedIterator + Eq,
    I::Item: Eq,
{
}

impl<I> PartialEq for PeekableDE<I>
where
    I: DoubleEndedIterator + PartialEq,
    I::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter == other.iter && self.front == other.front && self.back == other.back
    }
}

impl<I> From<Peekable<I>> for PeekableDE<Peekable<I>>
where
    I: DoubleEndedIterator,
    I::Item: Clone,
{
    fn from(mut peekable: Peekable<I>) -> Self {
        let front = peekable.peek().cloned().map(Some);

        PeekableDE {
            iter: peekable,
            front,
            back: None,
        }
    }
}

#[cfg(feature = "peekn")]
impl<I: DoubleEndedIterator> From<PeekN<I>> for PeekableDE<I> {
    fn from(mut peekn: PeekN<I>) -> Self {
        let front = {
            let len = peekn.peeked_len();
            if len == 0 {
                None
            } else {
                Some(peekn.buffer.pop_back())
            }
        };
        PeekableDE {
            iter: peekn.iter,
            front,
            back: None,
        }
    }
}

#[cfg(feature = "peekdn")]
impl<I: DoubleEndedIterator> From<PeekDN<I>> for PeekableDE<I> {
    fn from(mut peekdn: PeekDN<I>) -> Self {
        let front = {
            let len = peekdn.front_peeked_len();
            if len == 0 {
                None
            } else {
                Some(peekdn.front.pop_back())
            }
        };
        let back = {
            let len = peekdn.back_peeked_len();
            if len == 0 {
                None
            } else {
                Some(peekdn.back.pop_back())
            }
        };
        PeekableDE {
            iter: peekdn.iter,
            front,
            back,
        }
    }
}

impl<I: FusedIterator + DoubleEndedIterator> FusedIterator for PeekableDE<I> {}

impl<I: DoubleEndedIterator> PeekableDE<I> {
    /// Creates a new `PeekableDE` from a double-ended iterator.
    pub fn new(iter: I) -> Self {
        PeekableDE {
            iter,
            front: None,
            back: None,
        }
    }

    /// Peeks at the next item from the front without consuming it.
    ///
    /// Returns `Some(&item)` if an item is available, or `None` otherwise.
    pub fn peek_front(&mut self) -> Option<&I::Item> {
        if let Some(item) = self.front.get_or_insert_with(|| self.iter.next()).as_ref() {
            return Some(item);
        }
        self.back.as_ref().and_then(|b| b.as_ref())
    }

    /// Peeks at the next item from the back without consuming it.
    pub fn peek_back(&mut self) -> Option<&I::Item> {
        if let Some(item) = self
            .back
            .get_or_insert_with(|| self.iter.next_back())
            .as_ref()
        {
            return Some(item);
        }
        self.front.as_ref().and_then(|b| b.as_ref())
    }

    /// Peeks at the next item from the front as a mutable reference.
    pub fn peek_front_mut(&mut self) -> Option<&mut I::Item> {
        if let Some(item) = self.front.get_or_insert_with(|| self.iter.next()).as_mut() {
            return Some(item);
        }
        self.back.as_mut().and_then(|b| b.as_mut())
    }

    /// Peeks at the next item from the back as a mutable reference.
    pub fn peek_back_mut(&mut self) -> Option<&mut I::Item> {
        if let Some(item) = self
            .back
            .get_or_insert_with(|| self.iter.next_back())
            .as_mut()
        {
            return Some(item);
        }
        self.front.as_mut().and_then(|b| b.as_mut())
    }

    /// Consumes and returns the next front item if it satisfies the predicate.
    ///
    /// If the predicate fails, the item is pushed back and preserved.
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

    /// Consumes and returns the next back item if it satisfies the predicate.
    ///
    /// If the predicate fails, the item is pushed back and preserved.
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

    /// Converts this `PeekableDE` into a standard `Peekable`, discarding peek state.
    ///
    /// Any peeked front/back values will be lost.
    #[inline]
    pub fn into_peekable_lossy(self) -> Peekable<I> {
        self.iter.peekable()
    }

    /// Returns `true` if an item has been peeked from the front.
    #[inline]
    pub fn has_front_peeked(&self) -> bool {
        matches!(self.front, Some(Some(_)))
    }

    /// Returns `true` if an item has been peeked from the back.
    #[inline]
    pub fn has_back_peeked(&self) -> bool {
        matches!(self.back, Some(Some(_)))
    }

    /// Discards the currently peeked front item without consuming it.
    #[inline]
    pub fn clear_front_peeked(&mut self) {
        self.front = None;
    }

    /// Discards the currently peeked back item without consuming it.
    #[inline]
    pub fn clear_back_peeked(&mut self) {
        self.back = None;
    }

    /// Clears both front and back peeked items, if any, without advancing the iterator.
    #[inline]
    pub fn clear_peeked(&mut self) {
        self.clear_front_peeked();
        self.clear_back_peeked();
    }

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

    fn next_with_source(&mut self) -> Option<PeekSource<I::Item>> {
        if let Some(front) = self.front.take().flatten() {
            Some(PeekSource::Front(front))
        } else if let Some(iter) = self.iter.next() {
            Some(PeekSource::Iter(iter))
        } else {
            self.back.take().flatten().map(PeekSource::Back)
        }
    }

    fn cache_front(&mut self, item: PeekSource<I::Item>) {
        match item {
            PeekSource::Front(front) => self.front = Some(Some(front)),
            PeekSource::Iter(iter) => self.front = Some(Some(iter)),
            PeekSource::Back(back) => self.back = Some(Some(back)),
        }
    }

    fn next_back_with_source(&mut self) -> Option<PeekSource<I::Item>> {
        if let Some(back) = self.back.take().flatten() {
            Some(PeekSource::Back(back))
        } else if let Some(iter) = self.iter.next_back() {
            Some(PeekSource::Iter(iter))
        } else {
            self.front.take().flatten().map(PeekSource::Front)
        }
    }

    fn cache_back(&mut self, item: PeekSource<I::Item>) {
        match item {
            PeekSource::Front(front) => self.front = Some(Some(front)),
            PeekSource::Iter(iter) => self.back = Some(Some(iter)),
            PeekSource::Back(back) => self.back = Some(Some(back)),
        }
    }
}

/// A convenient function to wrap an iterator into `PeekN`.
///
/// # Examples
/// ```
/// use peeknth::peekablede;
/// let mut iter = peekablede(0..=5);
/// assert_eq!(iter.peek_front(), Some(&0));
/// ```
pub fn peekablede<I: DoubleEndedIterator>(iter: I) -> PeekableDE<I> {
    PeekableDE::new(iter)
}
