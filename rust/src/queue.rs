pub const MAX_LENGTH: usize = super::point::MAX_COORD * super::point::MAX_COORD;

// Using VecDeque directly doesn't seem to hurt performance much
pub struct Queue<T: Copy> {
    start: usize,
    end: usize,
    data: [std::mem::MaybeUninit<T>; MAX_LENGTH],
}

impl<T: Copy> Queue<T> {
    pub fn new() -> Self {
        Self {
            data: unsafe { std::mem::MaybeUninit::uninit().assume_init() },
            start: 0,
            end: 0,
        }
    }
    pub fn push(&mut self, value: T) {
        let cell = if cfg!(feature = "unsafe-indexing") {
            unsafe { self.data.get_unchecked_mut(self.end) }
        } else {
            &mut self.data[self.end]
        };
        *cell = std::mem::MaybeUninit::new(value);
        self.end += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            let cell = if cfg!(feature = "unsafe-indexing") {
                unsafe { self.data.get_unchecked(self.start) }
            } else {
                &self.data[self.start]
            };
            let result = Some(unsafe { cell.assume_init() });
            self.start += 1;
            result
        }
    }
}
