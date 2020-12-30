use super::Point;

pub struct Array2D<T> {
    inner: Vec<T>,
    width: usize,
    // Height is not actually used
    #[allow(dead_code)]
    height: usize,
}

impl<T> Array2D<T> {
    fn index(&self, pos: Point) -> usize {
        pos.0 as usize + self.width * pos.1 as usize
    }
}

impl<T: Default + Clone> Array2D<T> {
    pub fn new_default(width: usize, height: usize) -> Self {
        Self::filled_with(T::default(), width, height)
    }
}

impl<T: Clone> Array2D<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Self {
        Self {
            inner: vec![value; width * height],
            width,
            height,
        }
    }
    pub fn fill(&mut self, value: T) {
        for x in &mut self.inner {
            x.clone_from(&value);
        }
    }
}

impl<T> std::ops::Index<Point> for Array2D<T> {
    type Output = T;
    fn index(&self, pos: Point) -> &T {
        let index: usize = self.index(pos);
        unsafe { self.inner.get_unchecked(index) }
    }
}

impl<T> std::ops::IndexMut<Point> for Array2D<T> {
    fn index_mut(&mut self, pos: Point) -> &mut T {
        let index: usize = self.index(pos);
        unsafe { self.inner.get_unchecked_mut(index) }
    }
}
