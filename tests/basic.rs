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
}
