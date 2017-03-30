mod poly;

use poly::board::Board;
use poly::board::board_utils;
use poly::point::Point;
use poly::polyomino::Polyomino;
use poly::utils;

fn main() {
    let p = make_simple_polyomino();
    let p1 = p.rotate();
    let mut b = Board::new(3, 20);

    b.add_polyomino(&p, Point::new(1, 0));
    b.add_polyomino(&p1, Point::new(5,0));
    
    println!("{}", b);

    let adj = board_utils::get_all_adjacent(Point::new(0,0), &b);

    println!("Adjacent to 0,0");
    for p in adj {
        println!("{}, {}", p.x, p.y);
    }

    let adj = board_utils::get_all_adjacent(Point::new(4,0), &b);

    println!("Adjacent to 4,0");
    for p in adj {
        println!("{}, {}", p.x, p.y);
    }

    for p in utils::build_pentominoes() {
        println!("{}", p);
    }
}

fn make_simple_polyomino() -> Polyomino {
    let mut v = Vec::new();
    v.push(Point::new(0,1));
    v.push(Point::new(1,0));
    v.push(Point::new(1,1));
    v.push(Point::new(1,2));
    v.push(Point::new(2,2));
    
    Polyomino::new(v)
}

