use std::collections::VecDeque;

pub struct Queue<T: Copy> {
    inner: VecDeque<T>,
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Self {
        Self {
            inner: VecDeque::new(),
        }
    }
    pub fn push(&mut self, value: T) {
        self.inner.push_back(value);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }
}
