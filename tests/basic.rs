use peeknth::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek() {
        let mut it = peekn(1..4);
        assert_eq!(it.peek(), Some(&1));
        assert_eq!(it.peek(), Some(&1)); // peek しても消えない
        assert_eq!(it.next(), Some(1)); // next で取り出す
        assert_eq!(it.peek(), Some(&2));
    }

    #[test]
    fn test_peek_nth() {
        let mut it = peekn(10..);
        assert_eq!(it.peek_nth(3), Some(&13));
        assert_eq!(it.peek(), Some(&10)); // peek したときの値は変わらない
        assert_eq!(it.next(), Some(10));
        assert_eq!(it.peek_nth(2), Some(&13)); // 次に 11, 12, 13 が来てる
    }

    #[test]
    fn test_peek_range() {
        let mut it = peekn(0..10);
        let peeked: Vec<_> = it.peek_range(2..5).cloned().collect();
        assert_eq!(peeked, vec![2, 3, 4]);
        assert_eq!(it.next(), Some(0));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.peek(), Some(&2));
    }

    #[test]
    fn test_peeked_len_and_has_peeked() {
        let mut it = peekn(0..);
        assert!(!it.has_peeked(0));
        it.peek_nth(2);
        assert_eq!(it.peeked_len(), 3);
        assert!(it.has_peeked(2));
        assert!(!it.has_peeked(3));
    }

    #[test]
    fn test_clear_and_drain_peeked() {
        let mut it = peekn(0..);
        it.peek_nth(4);
        assert_eq!(it.peeked_len(), 5);
        it.drain_peeked(2);
        assert_eq!(it.peeked_len(), 3);
        it.clear_peeked();
        assert_eq!(it.peeked_len(), 0);
    }

    #[test]
    fn test_next_with_peeked() {
        let mut it = peekn(5..);
        it.peek_nth(1); // 5, 6 をバッファ
        assert_eq!(it.next(), Some(5)); // 5 を消費
        assert_eq!(it.next(), Some(6)); // バッファから 6 を消費
        assert_eq!(it.next(), Some(7)); // イテレータから
    }

    #[test]
    fn test_peek_range_unbounded() {
        let mut it = peekn(100..);
        let _ = it.peek_range(..=2); // peek_nth(2) 相当
        assert_eq!(it.peeked_len(), 3);
    }

    #[test]
    fn test_clone_peekn() {
        let mut a = peekn(0..10);
        a.peek_nth(2);
        let mut b = a.clone();

        assert_eq!(a, b);

        assert_eq!(a.next(), Some(0));
        assert_eq!(b.next(), Some(0));
        assert_eq!(a, b); // 進行が同じ
    }

    #[test]
    fn test_debug_peekn() {
        let mut iter = peekn(1..4);
        iter.peek_nth(1);

        let debug_str = format!("{:?}", iter);
        assert!(debug_str.contains("PeekN"));
        assert!(debug_str.contains("buffer"));
        assert!(debug_str.contains("iter"));
    }

    #[test]
    fn test_eq_peekn() {
        let mut a = peekn(0..5);
        let mut b = peekn(0..5);
        assert_eq!(a, b);

        a.peek();
        b.peek();
        assert_eq!(a, b);

        a.next();
        assert_ne!(a, b);
    }

    #[test]
    fn test_fused_iterator() {
        let mut iter = peekn(0..2);
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None); // FusedIteratorなら何度呼んでもNone
        assert_eq!(iter.peek(), None);
    }

    #[test]
    fn test_exact_size_iterator() {
        let mut iter = peekn(0..5); // 5個ある
        assert_eq!(iter.len(), 5);

        iter.peek_nth(2); // バッファに3つ入る（0,1,2）
        assert_eq!(iter.len(), 5); // まだ全体では5個

        iter.next(); // consume 1個（0）
        assert_eq!(iter.len(), 4);
    }

    #[test]
    fn test_basic_iteration() {
        let mut p = PeekableDE::new(1..=3);
        assert_eq!(p.next(), Some(1));
        assert_eq!(p.next_back(), Some(3));
        assert_eq!(p.next(), Some(2));
        assert_eq!(p.next_back(), None);
    }

    #[test]
    fn test_peek_front_back() {
        let mut p = PeekableDE::new(10..=12);
        assert_eq!(p.peek_front(), Some(&10));
        assert_eq!(p.next(), Some(10));
        assert_eq!(p.peek_back(), Some(&12));
        assert_eq!(p.next_back(), Some(12));
    }

    #[test]
    fn test_peek_mut() {
        let mut p = PeekableDE::new(vec![100, 200, 300].into_iter());
        if let Some(v) = p.peek_front_mut() {
            *v = 111;
        }
        assert_eq!(p.next(), Some(111));

        if let Some(v) = p.peek_back_mut() {
            *v = 333;
        }
        assert_eq!(p.next_back(), Some(333));
    }

    #[test]
    fn test_next_if() {
        let mut p = PeekableDE::new(1..=3);
        assert_eq!(p.next_if(|&x| x == 1), Some(1));
        assert_eq!(p.next_if(|&x| x == 3), None);
        assert_eq!(p.next(), Some(2)); // 3 is peeked and preserved, 2 is next
    }

    #[test]
    fn test_next_back_if() {
        let mut p = PeekableDE::new(1..=3);
        assert_eq!(p.next_back_if(|&x| x == 3), Some(3));
        assert_eq!(p.next_back_if(|&x| x == 1), None);
        assert_eq!(p.next_back(), Some(2)); // 1 is preserved
    }

    #[test]
    fn test_next_if_eq() {
        let mut p = PeekableDE::new("abc".chars());
        assert_eq!(p.next_if_eq(&'a'), Some('a'));
        assert_eq!(p.next_if_eq(&'z'), None);
        assert_eq!(p.next(), Some('b'));
    }

    #[test]
    fn test_into_peekable_lossy() {
        let mut p = PeekableDE::new(1..=2);
        p.peek_front();
        p.peek_back();
        let mut std_peek = p.into_peekable_lossy();
        assert_eq!(std_peek.next(), None); // discards peeked state
    }

    #[test]
    fn test_has_peeked_flags() {
        let mut p = PeekableDE::new(1..=2);
        assert!(!p.has_front_peeked());
        assert!(!p.has_back_peeked());

        p.peek_front();
        assert!(p.has_front_peeked());
        assert!(!p.has_back_peeked());

        p.peek_back();
        assert!(p.has_back_peeked());
    }

    #[test]
    fn test_clone_eq_debug() {
        let p1 = PeekableDE::new(1..=3);
        let p2 = p1.clone();
        assert_eq!(format!("{:?}", p1), format!("{:?}", p2));
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_from_vec_into_iter() {
        let vec = vec![1, 2, 3];
        let p = PeekableDE::new(vec.into_iter());
        let collected: Vec<_> = p.collect();
        assert_eq!(collected, vec![1, 2, 3]);
    }

    #[test]
    fn test_exact_size() {
        let p = PeekableDE::new(0..3);
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn test_basic_peek_and_next() {
        let mut iter = peekdn(1..=5);
        assert_eq!(iter.peek_front(), Some(&1));
        assert_eq!(iter.peek_back(), Some(&5));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next(), Some(2));
    }

    #[test]
    fn test_peek_nth_front_back() {
        let mut iter = peekdn(1..=5);
        assert_eq!(iter.peek_front_nth(2), Some(&3));
        assert_eq!(iter.next(), Some(1)); // consumes first
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3)); // the one peeked
    }

    #[test]
    fn test_double_peek_mut() {
        let mut iter = peekdn(vec![10, 20, 30].into_iter());
        if let Some(v) = iter.peek_front_mut() {
            *v = 11;
        }
        assert_eq!(iter.next(), Some(11));
        if let Some(v) = iter.peek_back_mut() {
            *v = 33;
        }
        assert_eq!(iter.next_back(), Some(33));
    }

    #[test]
    fn test_next_if_and_next_back_if() {
        let mut iter = peekdn(1..=3);
        assert_eq!(iter.next_if(|&x| x == 1), Some(1));
        assert_eq!(iter.next_if(|&x| x == 3), None);
        assert_eq!(iter.next(), Some(2));

        let mut iter_back = peekdn(1..=3);
        assert_eq!(iter_back.next_back_if(|&x| x == 3), Some(3));
        assert_eq!(iter_back.next_back_if(|&x| x == 1), None);
        assert_eq!(iter_back.next_back(), Some(2));
    }

    #[test]
    fn test_peekdn_next_if_eq() {
        let mut iter = peekdn("abc".chars());
        assert_eq!(iter.next_if_eq(&'a'), Some('a'));
        assert_eq!(iter.next_if_eq(&'z'), None);
        assert_eq!(iter.next(), Some('b'));
    }

    #[test]
    fn test_range_peek_front() {
        let mut iter = peekdn(0..=4);
        let peeked: Vec<_> = iter.peek_front_range(1..=3).cloned().collect();
        assert_eq!(peeked, vec![1, 2, 3]);
    }

    #[test]
    fn test_range_peek_back() {
        let mut iter = peekdn(0..=4);
        let _ = iter.peek_back_range(0..2); // triggers buffer fill
        assert_eq!(iter.back_peeked_len() >= 2, true);
    }

    #[test]
    fn test_drain_and_clear() {
        let mut iter = peekdn(0..5);
        let _ = iter.peek_front_nth(2);
        let _ = iter.peek_back_nth(1);
        assert!(iter.has_front_peeked(1));
        assert!(iter.has_back_peeked(0));
        iter.drain_peeked(2, 1);
        assert_eq!(iter.front_peeked_len(), 1); // should remain 1 item
        iter.clear_peeked();
        assert_eq!(iter.front_peeked_len(), 0);
        assert_eq!(iter.back_peeked_len(), 0);
    }

    #[test]
    fn test_peekdn_clone_eq_debug() {
        let p1 = peekdn(1..=3);
        let p2 = p1.clone();
        assert_eq!(format!("{:?}", p1), format!("{:?}", p2));
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_exact_size_len() {
        let mut iter = peekdn(0..5);
        assert_eq!(iter.len(), 5);
        let _ = iter.peek_front_nth(1);
        assert_eq!(iter.len(), 5); // +1 front buffer
    }

    #[test]
    fn test_peekdn_into_peekable_lossy() {
        let mut iter = peekdn(0..3);
        iter.peek_front();
        iter.peek_back();
        let mut std_peek = iter.into_peekable_lossy();
        assert_eq!(std_peek.next(), Some(1));
    }
}
