use std::cmp;

mod poly;

use poly::polyomino;
use poly::point;

fn main() {
    let p = polyomino::Polyomino{points: vec![point::Point{x:0,y:1}, point::Point{x:1,y:0}, point::Point{x:1,y:1}, point::Point{x:1,y:2}, point::Point{x:2,y:2}]};

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
      
