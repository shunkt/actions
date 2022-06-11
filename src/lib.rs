pub struct Point {
    x: isize,
    y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn square(&self) -> isize {
        self.x * self.y
    }
}
#[cfg(test)]
mod tests {
    use super::Point;
    #[test]
    fn test_squre() {
        let p = Point::new(2, 2);
        assert_eq!(p.square(), 4);
    }
}
