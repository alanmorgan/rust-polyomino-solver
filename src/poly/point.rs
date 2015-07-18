#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(new_x:i32, new_y:i32) -> Point {
        Point { x: new_x, y: new_y }
    }
}
