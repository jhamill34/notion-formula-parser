use lookahead_buffer::LookaheadBuffer;

pub fn consume_number_literal(buffer: &mut LookaheadBuffer<u8>) {
    consume_digits(buffer);

    if let Some(b'.') = buffer.peek(0) {
        consume_fraction(buffer);
    }

    if let Some(b'e') | Some(b'E') = buffer.peek(0) {
        consume_exponent(buffer);
    }
}

fn consume_digits(buffer: &mut LookaheadBuffer<u8>) {
    while let Some(b'0'..=b'9') = buffer.peek(0) {
        buffer.advance();
    }
}

fn consume_exponent(buffer: &mut LookaheadBuffer<u8>) {
    match buffer.peek(1) {
        Some(b'-') | Some(b'+') => {
            if let Some(b'0'..=b'9') = buffer.peek(2) {
                buffer.advance();
                buffer.advance();
                consume_digits(buffer);
            }
        }
        Some(b'0'..=b'9') => {
            buffer.advance();
            consume_digits(buffer)
        }
        _ => {}
    }
}

fn consume_fraction(buffer: &mut LookaheadBuffer<u8>) {
    if let Some(b'0'..=b'9') = buffer.peek(1) {
        buffer.advance();
        consume_digits(buffer);
    }
}
