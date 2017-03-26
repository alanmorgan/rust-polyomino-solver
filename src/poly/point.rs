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
        Point { x: usize::max_value(), y: usize::max_value() }
    }
    
    pub fn min_point() -> Point {
        Point { x: 0, y: 0 }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use Point;

    #[test]
    fn ord() {
        let p00 = Point {x:0, y:0};
        let p01 = Point {x:0, y:1};
        let p10 = Point {x:1, y:0};
        let p11 = Point {x:1, y:1};
        
        let p11_new = Point::new(1, 1);

        assert_eq!(p00.cmp(&p01), Ordering::Less);
        assert_eq!(p01.cmp(&p00), Ordering::Greater);
        assert_eq!(p01.cmp(&p11), Ordering::Less);
        assert_eq!(p10.cmp(&p11), Ordering::Less);
        assert_eq!(p00.cmp(&p00), Ordering::Equal);

        assert_eq!(p11.cmp(&p11_new), Ordering::Equal);
    }
}
