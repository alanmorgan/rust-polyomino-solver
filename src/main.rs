use std::cmp;
use std::collections::HashSet;

mod poly;

use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::board::Board;
fn main() {
    let b = Board::new(3, 20);
    b.print();
}

#[test]
fn make_polys () {
    let mut s = HashSet::new();
    s.insert(Point::new(0,1));
    s.insert(Point::new(1,0));
    s.insert(Point::new(1,1));
    s.insert(Point::new(1,2));
    s.insert(Point::new(2,2));
    
    let p = Polyomino::new (s);

    println!("p = {:?}", p);
    println!("top right   = {:?}", p.top_right());
    println!("bottom left = {:?}", p.bottom_left());

    p.show();
    println!("");
    p.rotate().show();
    println!("");
    p.rotate().rotate().show();
    println!("");
    p.rotate().rotate().rotate().show();
    println!("");
    p.flip().show();
    println!("");
    p.flip().rotate().show();
    println!("");
    p.flip().rotate().rotate().show();
    println!("");
    p.flip().rotate().rotate().rotate().show();
}
