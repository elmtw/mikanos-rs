use crate::vector::Vector2D;

#[derive(Debug, Copy, Clone)]
pub struct Rectangle<T> {
    origin: Vector2D<T>,
    end: Vector2D<T>,
}


impl<T: Copy> Rectangle<T> {
    pub fn new(origin: Vector2D<T>, end: Vector2D<T>) -> Rectangle<T> {
        Self { origin, end }
    }


    pub fn origin(&self) -> Vector2D<T> {
        self.origin
    }


    pub fn end(&self) -> Vector2D<T> {
        self.end
    }
}


impl<T: PartialEq + Copy> PartialEq for Rectangle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin() && self.end == other.end()
    }
}


#[cfg(test)]
mod tests {
    use crate::rectangle::Rectangle;
    use crate::vector::Vector2D;

    #[test]
    fn it_partial_eq_rectangle() {
        let r1 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        let r2 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        assert_eq!(r1, r2);
    }


    #[test]
    fn it_partial_ne_rectangle() {
        let r1 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(10, 100));
        let r2 = Rectangle::new(Vector2D::new(0, 0), Vector2D::new(100, 100));
        assert_ne!(r1, r2);
    }
}
