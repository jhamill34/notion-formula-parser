pub struct LookaheadBuffer<T: Clone> {
    buf: Vec<T>,
    start: usize,
    current: usize,
}

impl<T: Clone> LookaheadBuffer<T> {
    pub fn new(buffer: Vec<T>) -> Self {
        LookaheadBuffer {
            buf: buffer,
            current: 0,
            start: 0,
        }
    }

    pub fn peek(&self, n: usize) -> Option<T> {
        if self.current + n >= self.buf.len() {
            return None;
        }

        Some(self.buf[self.current + n].clone())
    }

    pub fn advance(&mut self) {
        if self.current == self.buf.len() {
            return;
        }

        self.current = self.current + 1;
    }

    pub fn get_slice(&self) -> Vec<T> {
        self.buf[self.start..self.current].to_vec()
    }

    pub fn commit(&mut self) {
        self.start = self.current;
    }
}
