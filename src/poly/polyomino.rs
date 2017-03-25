use std::cmp;
use std::slice::Iter;

use poly::point::Point;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Polyomino {
    points: Vec<Point>
}

impl PartialEq for Polyomino {
    fn eq(&self, other: &Polyomino) -> bool {
        self.points == other.points
    }

    fn ne(&self, other: &Polyomino) -> bool {
        !self.eq(other)
    }
}

impl Eq for Polyomino {
}

pub struct PolyominoIterator<'a> {
    i : Iter<'a, Point>
}

impl<'a> Iterator for PolyominoIterator<'a> {
    type Item = & 'a Point;
    
    fn next(&mut self) -> Option<& 'a Point> {
        self.i.next()
    }
}

#[allow(dead_code)]
impl Polyomino {
    pub fn new(mut p: Vec<Point>) -> Polyomino {
        p.sort();
        Polyomino { points: p }
    }
    
    pub fn top_right(&self) -> Point {
        self.points.iter().fold(Point::min_point(), |a, ref p| Point { x: cmp::max(a.x, p.x), y: cmp::max(a.y, p.y) })
    }
    
    pub fn bottom_left(&self) -> Point {
        self.points.iter().fold(Point::max_point(), |a, ref p| Point { x: cmp::min(a.x, p.x), y: cmp::min(a.y, p.y) })
    }

    pub fn show(&self) {
        // Inefficient, but it hardly matters
        let Point { x: width, y: height } = self.top_right();
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
        Polyomino { points: self.points.iter().map(|p| Point {x: height-p.y, y: p.x}).collect() }
    }

    pub fn flip(&self) -> Polyomino {
        let width = self.top_right().x;
        Polyomino { points: self.points.iter().map(|p| Point {x: width-p.x, y: p.y}).collect() }
    }

    pub fn iter(&self) -> PolyominoIterator {
        PolyominoIterator { i: self.points.iter() }
    }
}

