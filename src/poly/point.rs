use std::i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

/*
impl PartialEq for Point {
    pub fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

    pub fn ne(&self, other: &Point) -> bool {
        !self.eq(other)
    }
}

impl Eq for Point {}
 */

#[allow(dead_code)]
impl Point {
    pub fn new(new_x:i32, new_y:i32) -> Point {
        Point { x: new_x, y: new_y }
    }

    pub fn max_point() -> Point {
        Point { x: i32::MAX, y: i32::MAX }
    }
    
    pub fn min_point() -> Point {
        Point { x: i32::MIN, y: i32::MIN }
    }
}
