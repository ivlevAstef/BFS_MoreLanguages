pub struct Array2D<T> {
    inner: Vec<T>,
    width: usize,
    // Height is not actually used
    #[allow(dead_code)]
    height: usize,
}

impl<T: Clone> Array2D<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Self {
        Self {
            inner: vec![value; width * height],
            width,
            height,
        }
    }
}

impl<T> std::ops::Index<(usize, usize)> for Array2D<T> {
    type Output = T;
    fn index(&self, pos: (usize, usize)) -> &T {
        unsafe { self.inner.get_unchecked(pos.0 + self.width * pos.1) }
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, pos: (usize, usize)) -> &mut T {
        unsafe { self.inner.get_unchecked_mut(pos.0 + self.width * pos.1) }
    }
}
