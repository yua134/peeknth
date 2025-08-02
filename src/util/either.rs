pub enum Either<S, C> {
    Single(S),
    Chain(C),
}
impl<S, C> Iterator for Either<S, C>
where
    S: Iterator,
    C: Iterator<Item = S::Item>,
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Single(i) => i.next(),
            Either::Chain(i) => i.next(),
        }
    }
}
impl<S, C> DoubleEndedIterator for Either<S, C>
where
    S: DoubleEndedIterator,
    C: DoubleEndedIterator<Item = S::Item>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        match self {
            Either::Single(i) => i.next_back(),
            Either::Chain(i) => i.next_back(),
        }
    }
}
impl<S, C> Clone for Either<S, C>
where
    S: Iterator + Clone,
    C: Iterator<Item = S::Item> + Clone,
{
    fn clone(&self) -> Self {
        match self {
            Either::Single(i) => Either::Single(i.clone()),
            Either::Chain(i) => Either::Chain(i.clone()),
        }
    }
}
impl<S, C> core::fmt::Debug for Either<S, C>
where
    S: Iterator + core::fmt::Debug,
    C: Iterator<Item = S::Item> + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Either::Single(s) => f.debug_tuple("Single").field(s).finish(),
            Either::Chain(c) => f.debug_tuple("Chain").field(c).finish(),
        }
    }
}
impl<S, C> core::iter::FusedIterator for Either<S, C>
where
    S: core::iter::FusedIterator,
    C: Iterator<Item = S::Item> + core::iter::FusedIterator,
{
}

impl<S, C> core::iter::ExactSizeIterator for Either<S, C>
where
    S: core::iter::ExactSizeIterator,
    C: Iterator<Item = S::Item> + core::iter::ExactSizeIterator,
{
}
