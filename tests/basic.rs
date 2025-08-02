use peeknth::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_peek_nth_and_next() {
        let mut iter = peekn(0..);
        assert_eq!(iter.peek_nth(2), Some(&2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_peek_nth_mut() {
        let mut iter = peekn(0..);
        if let Some(x) = iter.peek_nth_mut(1) {
            *x += 10;
        }
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(11));
    }

    #[test]
    fn test_peek_and_peek_mut() {
        let mut iter = peekn(10..);
        assert_eq!(iter.peek(), Some(&10));
        if let Some(x) = iter.peek_mut() {
            *x += 5;
        }
        assert_eq!(iter.next(), Some(15));
    }

    #[test]
    fn test_next_if() {
        let mut iter = peekn(0..);
        assert_eq!(iter.next_if(|&x| x < 3), Some(0));
        assert_eq!(iter.next_if(|&x| x > 10), None); // 1 gets pushed back
        assert_eq!(iter.peek(), Some(&1));
    }

    #[test]
    fn test_next_if_eq() {
        let mut iter = peekn([1, 2, 3].into_iter());
        assert_eq!(iter.next_if_eq(&1), Some(1));
        assert_eq!(iter.next_if_eq(&5), None); // 2 gets pushed back
        assert_eq!(iter.peek(), Some(&2));
    }

    #[test]
    fn test_while_next() {
        let mut iter = peekn(0..);
        let result: Vec<_> = iter.while_next(|&x| x < 5).collect();
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
        assert_eq!(iter.peek(), Some(&5));
    }

    #[test]
    fn test_while_peek() {
        let mut iter = peekn(0..);
        let count = iter.while_peek(|&x| x < 3);
        assert_eq!(count, 3);
        assert_eq!(iter.next(), Some(0));
    }

    #[test]
    fn test_peeked_len_and_clear() {
        let mut iter = peekn(0..);
        let _ = iter.peek_nth(3);
        assert_eq!(iter.peeked_len(), 4);
        iter.clear_peeked();
        assert_eq!(iter.peeked_len(), 0);
    }

    #[test]
    fn test_drain_peeked() {
        let mut iter = peekn(0..);
        let _ = iter.peek_nth(4);
        assert_eq!(iter.peeked_len(), 5);
        iter.drain_peeked(3);
        assert_eq!(iter.peeked_len(), 2);
    }
    #[test]
    fn test_peek_nth() {
        let mut iter = sizedpeekn::<_, 4>(0..);
        assert_eq!(iter.peek_nth(2), Some(&2));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_sized_peek_nth_mut() {
        let mut iter = sizedpeekn::<_, 4>(0..);
        if let Some(x) = iter.peek_nth_mut(1) {
            *x += 10;
        }
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(11));
    }

    #[test]
    fn test_sized_peek_and_peek_mut() {
        let mut iter = sizedpeekn::<_, 4>(10..);
        assert_eq!(iter.peek(), Some(&10));
        if let Some(x) = iter.peek_mut() {
            *x += 5;
        }
        assert_eq!(iter.next(), Some(15));
    }

    #[test]
    fn test_sized_next_if() {
        let mut iter = sizedpeekn::<_, 3>(0..);
        assert_eq!(iter.next_if(|&x| x < 2), Some(0));
        assert_eq!(iter.next_if(|&x| x > 10), None); // 1 is pushed back
        assert_eq!(iter.peek(), Some(&1));
    }

    #[test]
    fn test_sized_next_if_eq() {
        let mut iter = sizedpeekn::<_, 3>([1, 2, 3].into_iter());
        assert_eq!(iter.next_if_eq(&1), Some(1));
        assert_eq!(iter.next_if_eq(&5), None); // 2 gets pushed back
        assert_eq!(iter.peek(), Some(&2));
    }

    #[test]
    fn test_sized_while_next() {
        let mut iter = sizedpeekn::<_, 6>(0..);
        let result: Vec<_> = iter.while_next(|&x| x < 4).collect();
        assert_eq!(result, vec![0, 1, 2, 3]);
        assert_eq!(iter.peek(), Some(&4));
    }

    #[test]
    fn test_sized_while_peek() {
        let mut iter = sizedpeekn::<_, 6>(0..);
        let count = iter.while_peek(|&x| x < 3);
        assert_eq!(count, 3);
        assert_eq!(iter.peek(), Some(&0)); // 位置は変わらない
    }

    #[test]
    fn test_sized_peeked_len_and_clear() {
        let mut iter = sizedpeekn::<_, 5>(0..);
        let _ = iter.peek_nth(3);
        assert_eq!(iter.peeked_len(), 4);
        iter.clear_peeked();
        assert_eq!(iter.peeked_len(), 0);
    }

    #[test]
    fn test_sized_drain_peeked() {
        let mut iter = sizedpeekn::<_, 5>(0..);
        let _ = iter.peek_nth(4);
        assert_eq!(iter.peeked_len(), 5);
        iter.drain_peeked(3);
        assert_eq!(iter.peeked_len(), 2);
    }

    #[test]
    fn test_has_peeked() {
        let mut iter = sizedpeekn::<_, 4>(0..);
        let _ = iter.peek_nth(2);
        assert!(iter.has_peeked(1));
        assert!(!iter.has_peeked(3));
    }
    #[test]
    fn test_peek_front_back() {
        let mut iter = peekdn(1..=5);
        assert_eq!(iter.peek_front(), Some(&1));
        assert_eq!(iter.peek_back(), Some(&5));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(5));
    }

    #[test]
    fn test_peek_front_nth_and_back_nth() {
        let mut iter = peekdn(0..=5);
        assert_eq!(iter.peek_front_nth(0), Some(&0));
        assert_eq!(iter.peek_front_nth(3), Some(&3));
        assert_eq!(iter.peek_back_nth(0), Some(&5));
        assert_eq!(iter.peek_back_nth(2), Some(&3));
    }

    #[test]
    fn test_next_if_and_next_back_if() {
        let mut iter = peekdn(0..=4);
        assert_eq!(iter.next_if(|&x| x == 0), Some(0));
        assert_eq!(iter.next_if(|&x| x == 2), None); // 1が返されなかった
        assert_eq!(iter.peek_front(), Some(&1));
        assert_eq!(iter.next_back_if(|&x| x == 4), Some(4));
        assert_eq!(iter.next_back_if(|&x| x == 2), None); // 3が返されなかった
        assert_eq!(iter.peek_back(), Some(&3));
    }

    #[test]
    fn test_next_if_eq_variants() {
        let mut iter = peekdn([1, 2, 3].into_iter());
        assert_eq!(iter.next_if_eq(&1), Some(1));
        assert_eq!(iter.next_if_eq(&5), None);
        assert_eq!(iter.peek_front(), Some(&2));

        let mut iter = peekdn([1, 2, 3].into_iter());
        assert_eq!(iter.next_back_if_eq(&3), Some(3));
        assert_eq!(iter.next_back_if_eq(&9), None);
        assert_eq!(iter.peek_back(), Some(&2));
    }

    #[test]
    fn test_while_peek_and_next_front_back() {
        let mut iter = peekdn(0..10);
        assert_eq!(iter.while_peek_front(|&x| x < 5), 5);
        assert_eq!(iter.while_peek_back(|&x| x >= 5), 5);

        let mut iter = peekdn(0..10);
        let vec: Vec<_> = iter.while_next_front(|&x| x < 5).collect();
        assert_eq!(vec, vec![0, 1, 2, 3, 4]);

        let mut iter = peekdn(0..10);
        let vec: Vec<_> = iter.while_next_back(|&x| x >= 5).collect();
        assert_eq!(vec, vec![9, 8, 7, 6, 5]);
    }

    #[test]
    fn test_clear_and_drain() {
        let mut iter = peekdn(0..10);
        let _ = iter.peek_front_nth(3);
        let _ = iter.peek_back_nth(2);
        assert!(iter.has_front_peeked(2));
        assert!(iter.has_back_peeked(1));
        iter.clear_peeked();
        assert_eq!(iter.front_peeked_len(), 0);
        assert_eq!(iter.back_peeked_len(), 0);
    }

    #[test]
    fn test_range_peek() {
        let mut iter = peekdn(0..10);
        let front: Vec<_> = iter.peek_front_range(2..6).cloned().collect();
        assert_eq!(front, vec![2, 3, 4, 5]);

        let back: Vec<_> = iter.peek_back_range(0..3).cloned().collect();
        assert_eq!(back, vec![9, 8, 7]);
    }

    #[test]
    fn test_peek_front_back_nth() {
        let mut it = sizedpeekdn::<_, 3, 3>(1..=5);
        assert_eq!(it.peek_front_nth(1), Some(&2));
        assert_eq!(it.peek_back_nth(1), Some(&4));
    }

    #[test]
    fn test_peek_front_nth_mut() {
        let mut it = sizedpeekdn::<_, 3, 3>(10..=20);
        if let Some(x) = it.peek_front_nth_mut(0) {
            *x += 5;
        }
        assert_eq!(it.next(), Some(15));
    }

    #[test]
    fn test_next_and_next_back() {
        let mut it = sizedpeekdn::<_, 3, 3>(1..=5);
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next_back(), Some(5));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next_back(), Some(4));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next_back(), None);
    }

    #[test]
    fn test_next_if_variants() {
        let mut it = sizedpeekdn::<_, 3, 3>(0..=5);
        assert_eq!(it.next_if(|&x| x == 0), Some(0));
        assert_eq!(it.next_if_eq(&2), None); // 1 is pushed back
        assert_eq!(it.next(), Some(1));
    }

    #[test]
    fn test_next_back_if_variants() {
        let mut it = sizedpeekdn::<_, 3, 3>(0..=4);
        assert_eq!(it.next_back_if(|&x| x == 4), Some(4));
        assert_eq!(it.next_back_if_eq(&2), None); // 3 pushed back
        assert_eq!(it.next_back(), Some(3));
    }

    #[test]
    fn test_clear_peeked_and_len() {
        let mut it = sizedpeekdn::<_, 3, 3>(0..10);
        let _ = it.peek_front_nth(2);
        let _ = it.peek_back_nth(2);
        assert!(it.has_front_peeked(1));
        assert!(it.has_back_peeked(1));
        it.clear_peeked();
        assert_eq!(it.front_peeked_len(), 0);
        assert_eq!(it.back_peeked_len(), 0);
    }

    #[test]
    fn test_into_peekable_lossy() {
        let mut it = sizedpeekdn::<_, 3, 3>(1..=5);
        let _ = it.peek_front_nth(2);
        let mut std_peek = it.into_peekable_lossy();
        assert_eq!(std_peek.peek(), Some(&4)); // バッファ捨てられたあと先頭が4になる
    }

    #[test]
    fn test_peek_front_and_back() {
        let mut iter = peekablede(1..=3);
        assert_eq!(iter.peek_front(), Some(&1));
        assert_eq!(iter.peek_back(), Some(&3));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_peek_mut_and_modify() {
        let mut iter = peekablede(vec![10, 20, 30].into_iter());
        if let Some(x) = iter.peek_front_mut() {
            *x += 5;
        }
        assert_eq!(iter.next(), Some(15));
        if let Some(x) = iter.peek_back_mut() {
            *x *= 2;
        }
        assert_eq!(iter.next_back(), Some(60));
    }

    #[test]
    fn test_next_if_de_variants() {
        let mut iter = peekablede(0..=2);
        assert_eq!(iter.next_if(|&x| x == 0), Some(0));
        assert_eq!(iter.next_if_eq(&2), None); // 1が保持される
        assert_eq!(iter.peek_front(), Some(&1));

        let mut iter = peekablede(5..8);
        assert_eq!(iter.next_back_if(|&x| x == 7), Some(7));
        assert_eq!(iter.next_back_if_eq(&6), Some(6));
        assert_eq!(iter.next_back_if_eq(&9), None);
        assert_eq!(iter.peek_back(), Some(&5));
    }

    #[test]
    fn test_has_and_clear_peeked() {
        let mut iter = peekablede(1..=5);
        let _ = iter.peek_front();
        let _ = iter.peek_back();
        assert!(iter.has_front_peeked());
        assert!(iter.has_back_peeked());

        iter.clear_peeked();
        assert!(!iter.has_front_peeked());
        assert!(!iter.has_back_peeked());
    }

    #[test]
    fn test_de_into_peekable_lossy() {
        let mut iter = peekablede(0..=2);
        let _ = iter.peek_front(); // state will be lost
        let mut std_peek = iter.into_peekable_lossy();
        assert_eq!(std_peek.peek(), Some(&1)); // 一部 peek 状態が捨てられてる
    }

    #[test]
    fn test_while_next_front_and_back() {
        let mut iter = peekablede(0..10);
        let front: Vec<_> = iter.while_next_front(|&x| x < 3).collect();
        assert_eq!(front, vec![0, 1, 2]);

        let mut iter = peekablede(5..10);
        let back: Vec<_> = iter.while_next_back(|&x| x >= 8).collect();
        assert_eq!(back, vec![9, 8]);
    }
}
