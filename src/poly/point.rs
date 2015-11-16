#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize
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
