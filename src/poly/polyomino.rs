use poly::point;
use std::cmp;

#[derive(Debug)]
pub struct Polyomino {
    pub points: Vec<point::Point>
}

impl Polyomino {
    pub fn top_right(&self) -> point::Point {
        self.points.iter().fold(self.points[0], |a, ref p| point::Point { x: cmp::max(a.x, p.x), y: cmp::max(a.y, p.y) })
    }
    
    pub fn bottom_left(&self) -> point::Point {
        self.points.iter().fold(self.points[0], |a, ref p| point::Point { x: cmp::min(a.x, p.x), y: cmp::min(a.y, p.y) })
    }

    pub fn show(&self) {
        // Inefficient, but it hardly matters
        let point::Point { x: width, y: height } = self.top_right();
        for y in 0..height+1 {
            for x in 0..width+1 {
                if let Some(_p) = self.points.iter().find(|ref p| p.x == x && p.y == (height-y)) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn rotate(&self) -> Polyomino {
        let height = self.top_right().y;
        Polyomino { points: self.points.iter().map(|p| point::Point {x: height-p.y, y: p.x}).collect() }
    }

    pub fn flip(&self) -> Polyomino {
        let width = self.top_right().x;
        Polyomino { points: self.points.iter().map(|p| point::Point {x: width-p.x, y: p.y}).collect() }
    }
}

