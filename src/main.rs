use std::cmp;
use std::collections::HashSet;

mod poly;

use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::board::Board;
fn main() {
    let p = make_simple_polyomino();
    let mut b = Board::new(3, 20);

    b.add_polyomino(&p, Point::new(0, 0));
    b.print();
}

fn make_simple_polyomino() -> Polyomino {
    let mut s = HashSet::new();
    s.insert(Point::new(0,1));
    s.insert(Point::new(1,0));
    s.insert(Point::new(1,1));
    s.insert(Point::new(1,2));
    s.insert(Point::new(2,2));
    
    Polyomino::new(s)
}

#[test]
fn make_polys () {
    let p = make_simple_polyomino();

    assert_eq!(p, p);
    assert!(p != p.rotate());
    assert_eq!(p, p.rotate().rotate().rotate().rotate());

}
