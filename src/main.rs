mod poly;

use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::board::Board;
use poly::board::board_utils;

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

fn make_simple_polyomino_2() -> Polyomino {
    let mut v = Vec::new();
    v.push(Point::new(0,1));
    v.push(Point::new(1,1));
    v.push(Point::new(1,0));
    v.push(Point::new(2,2));
    v.push(Point::new(1,2));
    
    Polyomino::new(v)
}

fn make_simple_polyomino_3() -> Polyomino {
    let mut v = Vec::new();
    v.push(Point::new(0,1));
    v.push(Point::new(1,1));
    v.push(Point::new(1,0));
    v.push(Point::new(2,2));
    v.push(Point::new(2,2));
    v.push(Point::new(1,2));
    
    Polyomino::new(v)
}

#[test]
fn make_polys () {
    let p = make_simple_polyomino();

    assert_eq!(p, p);
    assert!(p != p.rotate());
    assert_eq!(p, p.rotate().rotate().rotate().rotate());

}

#[test]
fn compare_polys () {
    let p = make_simple_polyomino();
    let p2 = make_simple_polyomino_2();
    let p3 = make_simple_polyomino_3();

    assert_eq!(p, p2);
    assert_eq!(p, p3);
}

#[test]
fn find_adjacent() {
    let p = make_simple_polyomino();
    let p1 = p.rotate();
    let mut b = Board::new(3, 20);

    b.add_polyomino(&p, Point::new(1, 0));
    b.add_polyomino(&p1, Point::new(5,0));
    
    let adj = board_utils::get_all_adjacent(Point::new(0,0), &b);

    assert!(adj.len() == 5);
    assert!(adj.contains(&Point::new(0,0)));
    assert!(adj.contains(&Point::new(1,0)));
    assert!(adj.contains(&Point::new(0,1)));
    assert!(adj.contains(&Point::new(0,2)));
    assert!(adj.contains(&Point::new(1,2)));

    let adj = board_utils::get_all_adjacent(Point::new(4,0), &b);

    assert!(adj.len() == 6);
    assert!(adj.contains(&Point::new(3,0)));
    assert!(adj.contains(&Point::new(4,0)));
    assert!(adj.contains(&Point::new(5,0)));
    assert!(adj.contains(&Point::new(3,1)));
    assert!(adj.contains(&Point::new(4,1)));
    assert!(adj.contains(&Point::new(4,2)));
}
