#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub(crate) fn convert<F>(&self, c: fn(&T) -> F) -> Point<F> {
        Point {
            x: c(&self.x),
            y: c(&self.y),
        }
    }
}

impl Point<usize> {
    pub fn top(&self) -> Point<usize> {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }
    pub fn right(&self) -> Point<usize> {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    pub fn bottom(&self) -> Point<usize> {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn left(&self) -> Point<usize> {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }
}