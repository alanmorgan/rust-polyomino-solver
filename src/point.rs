use std::cmp::Ordering;
use std::fmt;
use std::hash::Hash;

use point_derive::OrdForPoint;

pub trait Pt : Ord + PartialOrd + Copy + Hash + fmt::Display {
    fn x(&self) -> i16;
    fn set_x(&mut self, x: i16);
    
    fn y(&self) -> i16;
    fn set_y(&mut self, x: i16);
}

#[derive(Debug, Clone, Copy, OrdForPoint)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Pt for Point {
    fn x(&self) -> i16 {
        self.x
    }

    fn set_x(&mut self, new_x: i16) {
        self.x = new_x;
    }
    
    fn y(&self) -> i16 {
        self.y
    }
    
    fn set_y(&mut self, new_y: i16) {
        self.y = new_y;
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, " ")?;

        Ok(())
    }
}

#[allow(dead_code)]
impl Point {
    pub fn new(new_x: i16, new_y: i16) -> Point {
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







