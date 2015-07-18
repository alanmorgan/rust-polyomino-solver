use std::cmp;

mod poly;

use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::board::Board;
fn main() {
    let b = Board::new(3, 20);
    b.print();
}

/*
{
    let p = Polyomino::new (vec![Point::new(0, 1), Point::new(1,0), Point::new(1,1), Point::new(1,2), Point::new(2,2)]);

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
*/      
