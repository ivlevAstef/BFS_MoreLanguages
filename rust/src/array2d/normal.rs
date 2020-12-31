use super::{point, Point};

pub struct Array2D<T> {
    inner: Vec<Vec<T>>,
}

impl<T: Copy> Array2D<T> {
    pub fn filled_with(value: T, width: usize, height: usize) -> Self {
        Self {
            inner: vec![vec![value; height]; width],
        }
    }
}

impl<T> std::ops::Index<Point> for Array2D<T> {
    type Output = T;
    fn index(&self, pos: Point) -> &T {
        #[cfg(not(feature = "unsafe-indexing"))]
        return &self.inner[pos.x() as usize][pos.y() as usize];
        #[cfg(feature = "unsafe-indexing")]
        return unsafe {
            self.inner
                .get_unchecked(pos.x() as usize)
                .get_unchecked(pos.y() as usize)
        };
    }
}

impl<T> std::ops::IndexMut<Point> for Array2D<T> {
    fn index_mut(&mut self, pos: Point) -> &mut T {
        #[cfg(not(feature = "unsafe-indexing"))]
        return &mut self.inner[pos.x() as usize][pos.y() as usize];
        #[cfg(feature = "unsafe-indexing")]
        return unsafe {
            self.inner
                .get_unchecked_mut(pos.x() as usize)
                .get_unchecked_mut(pos.y() as usize)
        };
    }
}
