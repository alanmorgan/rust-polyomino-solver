use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x < other.x {
            return Ordering::Less;
        }

        if self.x > other.x {
            return Ordering::Greater;
        }

        self.y.cmp(&other.y)
    }
}


impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[allow(dead_code)]
impl Point {
    pub fn new(new_x:usize, new_y:usize) -> Point {
        Point { x: new_x, y: new_y }
    }

    pub fn max_point() -> Point {
        Point::new(usize::max_value(), usize::max_value())
    }
    
    pub fn min_point() -> Point {
        Point::new(0,0)
    }
}

#[cfg(test)]
mod tests {
    use point::Point;

    #[test]
    fn ord() {
        let p00 = Point::new(0, 0);
        let p07 = Point::new(0, 7);
        let p10 = Point::new(1, 0);
        let p16 = Point::new(1, 6);
        
        assert!(p00 < p07);
        assert!(p07 > p00);
        assert!(p07 < p10);
        assert!(p07 < p16);
        assert!(p07 == p07);
    }
}
