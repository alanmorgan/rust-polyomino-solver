use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
struct Polyomino {
    points: Vec<Point>
}

impl Polyomino {
    pub fn top_right(&self) -> Point {
        self.points.iter().fold(self.points[0], |a, ref p| Point { x: cmp::max(a.x, p.x), y: cmp::max(a.y, p.y) })
    }
    
    pub fn bottom_left(&self) -> Point {
        self.points.iter().fold(self.points[0], |a, ref p| Point { x: cmp::min(a.x, p.x), y: cmp::min(a.y, p.y) })
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

    pub fn rotatecw(&self) -> Polyomino {
        let height = self.top_right().y;

        let mut new_points = Vec::new();
        
        for p in self.points.iter().map(|p| Point {x: height-p.y, y: p.x}) {
            new_points.push(p);
        }
        Polyomino{ points: new_points }
    }
}

fn main() {
    let p = Polyomino{points: vec![Point{x:0,y:1}, Point{x:1,y:0}, Point{x:1,y:1}, Point{x:1,y:2}, Point{x:2,y:2}]};

    println!("p = {:?}", p);
    println!("top right   = {:?}", p.top_right());
    println!("bottom left = {:?}", p.bottom_left());

    p.show();
    println!("");
    p.rotatecw().show();
    println!("");
    p.rotatecw().rotatecw().show();
}
      
