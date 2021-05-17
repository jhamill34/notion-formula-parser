pub struct LookaheadBuffer<T> {
    buf: Vec<T>,
    start: usize,
    current: usize
}

impl From<&str> for LookaheadBuffer<u8> {
    fn from(input: &str) -> Self {
        LookaheadBuffer::new(Vec::from(input))
    }
}

impl<T> From<Vec<T>> for LookaheadBuffer<T> {
    fn from(input: Vec<T>) -> Self {
        LookaheadBuffer::new(input)
    }
}

impl<T> LookaheadBuffer<T> {
    pub fn new(buffer: Vec<T>) -> Self {
        LookaheadBuffer { buf: buffer, current: 0, start: 0 }
    }

    pub fn peek(&self, n: usize) -> Option<&T> {
        if self.current + n >= self.buf.len() {
            return None;
        }

        Some(&self.buf[self.current + n])
    }

    pub fn advance(&mut self) {
        if self.current == self.buf.len() {
            return;
        }

        self.current = self.current + 1;
    }

    pub fn get_slice(&self) -> &[T] {
        &self.buf[self.start..self.current]
    }

    pub fn commit(&mut self) {
        self.start = self.current;
    }
}
