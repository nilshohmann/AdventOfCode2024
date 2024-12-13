#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub(crate) fn convert<F>(&self, c: fn(&T) -> F) -> Point<F> {
        Point { x : c(&self.x), y : c(&self.y) }
    }
}
