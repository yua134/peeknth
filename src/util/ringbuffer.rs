use core::{mem::MaybeUninit, ops::RangeBounds, slice};

#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;

use crate::util::Either;

pub struct Buffer<T: Copy, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T: Copy + PartialEq, const N: usize> PartialEq for Buffer<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && (0..self.len).all(|i| self.get(i) == other.get(i))
    }
}

impl<T: Copy, const N: usize> Default for Buffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + Eq, const N: usize> Eq for Buffer<T, N> {}

impl<T: Copy, const N: usize> Clone for Buffer<T, N> {
    fn clone(&self) -> Self {
        let mut new = Buffer::new();
        for i in 0..self.len {
            new.push_back(*self.get(i).unwrap());
        }
        new
    }
}

impl<T: Copy + core::fmt::Debug, const N: usize> core::fmt::Debug for Buffer<T, N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Buffer[")?;
        for i in 0..self.len {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}", self.get(i).unwrap())?;
        }
        write!(f, "]")
    }
}

impl<T: Copy, const N: usize> FromIterator<T> for Buffer<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut buffer = Buffer::new();
        for item in iter {
            if buffer.len() >= N {
                panic!("Buffer overflow in FromIterator: max size is {}", N);
            }
            buffer.push_back(item);
        }
        buffer
    }
}

#[cfg(feature = "alloc")]
impl<T: Copy, const N: usize> From<Buffer<T, N>> for VecDeque<T> {
    fn from(mut buf: Buffer<T, N>) -> Self {
        let mut deque = VecDeque::with_capacity(buf.len());
        while let Some(val) = buf.pop_front() {
            deque.push_back(val);
        }
        deque
    }
}

impl<T: Copy, const N: usize> Buffer<T, N> {
    #[inline]
    pub fn new() -> Self {
        Buffer {
            buffer: [MaybeUninit::uninit(); N],
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    pub fn push_front(&mut self, value: T) {
        if self.len == N {
            panic!("buffer full");
        }

        self.head = (self.head + N - 1) % N;
        self.buffer[self.head].write(value);
        self.len += 1;
    }

    pub fn push_back(&mut self, value: T) {
        if self.len == N {
            panic!("buffer full");
        }

        self.buffer[self.tail].write(value);
        self.tail = (self.tail + 1) % N;
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        let value = unsafe { self.buffer[self.head].assume_init() };
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(value)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.tail = (self.tail + N - 1) % N;
        let value = unsafe { self.buffer[self.tail].assume_init() };
        self.len -= 1;
        Some(value)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let pos = (self.head + index) % N;
        Some(unsafe { &*self.buffer[pos].as_ptr() })
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        let pos = (self.head + index) % N;
        Some(unsafe { &mut *self.buffer[pos].as_mut_ptr() })
    }

    pub fn range<'a>(
        &'a self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = &'a T>
    where
        T: 'a,
    {
        use crate::get_start_end;

        let (start, end) = get_start_end(range, self.len);

        assert!(end <= self.len, "range out of bounds");

        let count = end - start;

        if count == 0 {
            let empty: &'a [T] = &[];
            return Either::Single(empty.iter());
        }

        let first_index = (self.head + start) % N;
        let last_index = (self.head + end) % N;

        if first_index < last_index || (first_index == last_index && count > 0) {
            let slice = unsafe {
                let ptr = self.buffer[first_index].as_ptr();
                slice::from_raw_parts(ptr, count)
            };
            Either::Single(slice.iter())
        } else {
            let first_len = if first_index <= last_index {
                end - start
            } else {
                N - first_index
            };

            let second_len = count - first_len;

            let first =
                unsafe { slice::from_raw_parts(self.buffer[first_index].as_ptr(), first_len) };

            let second = unsafe { slice::from_raw_parts(self.buffer[0].as_ptr(), second_len) };

            Either::Chain(first.iter().chain(second.iter()))
        }
    }

    pub fn range_mut<'a>(
        &'a mut self,
        range: impl RangeBounds<usize>,
    ) -> impl DoubleEndedIterator<Item = &'a mut T>
    where
        T: 'a,
    {
        use crate::get_start_end;

        let (start, end) = get_start_end(range, self.len);

        assert!(end <= self.len, "range out of bounds");

        let count = end - start;

        if count == 0 {
            let empty: &'a mut [T] = &mut [];
            return Either::Single(empty.iter_mut());
        }

        let first_index = (self.head + start) % N;
        let last_index = (self.head + end) % N;

        if first_index < last_index || (first_index == last_index && count > 0) {
            let slice = unsafe {
                let ptr = self.buffer[first_index].as_mut_ptr();
                slice::from_raw_parts_mut(ptr, count)
            };
            Either::Single(slice.iter_mut())
        } else {
            let first_len = if first_index <= last_index {
                end - start
            } else {
                N - first_index
            };

            let second_len = count - first_len;

            let first = unsafe {
                slice::from_raw_parts_mut(self.buffer[first_index].as_mut_ptr(), first_len)
            };

            let second =
                unsafe { slice::from_raw_parts_mut(self.buffer[0].as_mut_ptr(), second_len) };

            Either::Chain(first.iter_mut().chain(second.iter_mut()))
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn clear(&mut self) {
        self.len = 0;
        self.head = 0;
        self.tail = 0;
    }

    pub fn drain(&mut self, range: impl RangeBounds<usize>) {
        use crate::get_start_end;

        let (start, end) = get_start_end(range, self.len);

        assert!(end <= self.len, "range out of bounds");

        let count = end - start;

        if count == 0 {
            return;
        }

        if start == 0 {
            self.head = (self.head + count) % N;
        } else if end == self.len {
            self.tail = (self.tail + N - count) % N;
        } else {
            for i in 0..self.len - end {
                let from = (self.head + end + i) % N;
                let to = (self.head + start + i) % N;

                let value = unsafe { self.buffer[from].assume_init() };
                self.buffer[to].write(value);
            }

            self.tail = (self.tail + N - count) % N;
        }

        self.len -= count;
    }

    #[inline]
    pub fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        iter.into_iter().collect()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        N
    }
}
