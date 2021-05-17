use lookahead_buffer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_peek_at_top_values() {
        let input: LookaheadBuffer<u8> = "abcd".into();
        assert_eq!(&b'a', input.peek(0).unwrap());
        assert_eq!(&b'b', input.peek(1).unwrap());
    }

    #[test]
    fn test_can_consume_slice() {
        let mut input: LookaheadBuffer<u8> = "abcd".into();
        input.advance();
        input.advance();
        input.advance();

        let slice_a = input.get_slice();
        let slice_b = input.get_slice();
        let top = input.peek(0);

        assert_eq!(&[b'a', b'b', b'c'], slice_a);
        assert_eq!(&[b'a', b'b', b'c'], slice_b);
        assert_eq!(&b'd', top.unwrap());
    }

    #[test]
    fn test_can_consume_different_slices_after_commit() {
        let mut input: LookaheadBuffer<u8> = "abcd".into();
        input.advance();
        input.advance();
        input.advance();

        let slice_a = input.get_slice();
        assert_eq!(&[b'a', b'b', b'c'], slice_a);

        input.commit();
        input.advance();

        let slice_b = input.get_slice();
        assert_eq!(&[b'd'], slice_b);
    }

    #[test]
    fn test_can_consume_empty_slices_after_commit() {
        let mut input: LookaheadBuffer<u8> = "abcd".into();
        input.advance();
        input.advance();
        input.advance();

        let slice_a = input.get_slice();
        assert_eq!(&[b'a', b'b', b'c'], slice_a);

        input.commit();

        let slice_b: &[u8] = input.get_slice();
        let expected: &[u8] = &[];
        assert_eq!(expected, slice_b);

        let top = input.peek(0);
        assert_eq!(&b'd', top.unwrap());
    }

    #[test]
    fn test_peek_returns_null_byte_when_at_end() {
        let mut input: LookaheadBuffer<u8> = "abcd".into();
        input.advance();
        input.advance();
        input.advance();
        input.advance();
        input.commit();

        let top = input.peek(0);
        assert!(top.is_none())
    }

    #[test]
    fn test_advancing_past_length_of_buffer_does_nothing() {
        let mut input: LookaheadBuffer<u8> = "abcd".into();
        input.advance();
        input.advance();
        input.advance();
        input.advance();
        input.advance();
        input.advance();

        let slice = input.get_slice();
        assert_eq!(&[b'a', b'b', b'c', b'd'], slice);

        input.commit();

        let top = input.peek(0);
        assert!(top.is_none())
    }
}
