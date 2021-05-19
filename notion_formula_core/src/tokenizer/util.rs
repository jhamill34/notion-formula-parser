use lookahead_buffer::LookaheadBuffer;

pub fn consume_number_literal(buffer: &mut LookaheadBuffer<char>) {
    consume_digits(buffer);

    if let Some('.') = buffer.peek(0) {
        consume_fraction(buffer);
    }

    if let Some('e') | Some('E') = buffer.peek(0) {
        consume_exponent(buffer);
    }
}

fn consume_digits(buffer: &mut LookaheadBuffer<char>) {
    while let Some('0'..='9') = buffer.peek(0) {
        buffer.advance();
    }
}

fn consume_exponent(buffer: &mut LookaheadBuffer<char>) {
    match buffer.peek(1) {
        Some('-') | Some('+') => {
            if let Some('0'..='9') = buffer.peek(2) {
                buffer.advance();
                buffer.advance();
                consume_digits(buffer);
            }
        }
        Some('0'..='9') => {
            buffer.advance();
            consume_digits(buffer)
        }
        _ => {}
    }
}

fn consume_fraction(buffer: &mut LookaheadBuffer<char>) {
    if let Some('0'..='9') = buffer.peek(1) {
        buffer.advance();
        consume_digits(buffer);
    }
}
