use std::cmp::Ordering;
use std::hash::Hash;

pub type PointPos = i8;

pub trait PointT : Ord + PartialOrd + Copy + Hash {
    fn x(&self) -> PointPos;
    fn set_x(&mut self, x: PointPos) -> ();
    
    fn y(&self) -> PointPos;
    fn set_y(&mut self, x: PointPos) -> ();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: PointPos,
    pub y: PointPos,
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.x() < other.x() {
            return Ordering::Less;
        }

        if self.x() > other.x() {
            return Ordering::Greater;
        }

        self.y().cmp(&other.y())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl PointT for Point {
    fn x(&self) -> PointPos {
        self.x
    }

    fn set_x(&mut self, new_x: PointPos) -> () {
        self.x = new_x;
    }
    
    fn y(&self) -> PointPos {
        self.y
    }
    
    fn set_y(&mut self, new_y: PointPos) -> () {
        self.y = new_y;
    }
}


#[allow(dead_code)]
impl Point {
    pub fn new(new_x: PointPos, new_y: PointPos) -> Point {
        Point { x: new_x, y: new_y }
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







