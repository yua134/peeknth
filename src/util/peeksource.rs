pub enum PeekSource<T> {
    Front(T),
    Iter(T),
    Back(T),
}
impl<T> PeekSource<T> {
    #[inline]
    pub fn into_item(self) -> T {
        match self {
            PeekSource::Front(t) | PeekSource::Iter(t) | PeekSource::Back(t) => t,
        }
    }

    #[inline]
    pub fn as_ref(&self) -> &T {
        match self {
            PeekSource::Front(t) | PeekSource::Iter(t) | PeekSource::Back(t) => t,
        }
    }
}
