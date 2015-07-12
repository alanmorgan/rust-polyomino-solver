use std::cmp;

mod poly;

fn main() {
    let p = poly::Polyomino{points: vec![poly::Point{x:0,y:1}, poly::Point{x:1,y:0}, poly::Point{x:1,y:1}, poly::Point{x:1,y:2}, poly::Point{x:2,y:2}]};

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
      
