mod poly;

use poly::board::Board;
use poly::board::board_utils;
use poly::point::Point;
use poly::polyomino::Polyomino;
use poly::utils;

fn main() {
    let w = build_w();
    let l = build_l();
    let y = build_y();

    let mut b = Board::new(6, 10);
    fit(&mut b, &w);
    fit(&mut b, &l);
    fit(&mut b, &y);

    println!("{}", b);
}


// This should really go into board_utils

fn fit<'a>(b: &mut Board<'a>, p: &'a Polyomino) -> bool {
    // Attempt to fit the polyomino at the first unoccuped spot on the board.

    /* Consider the board with the first unoccupied spot marked 'A'

      +-+-+-+-+-+
      | | | | | |
      +-+-+-+-+-+
      |A| | | | |
      +-+-+-+-+-+
      |X| | | | |
      +-+-+-+-+-+
      |X|X|X| | |
      +-+-+-+-+-+

     And the polyomino
    
      x
     xxx
      x

    To fit this on the board we don't put the (0,0) point of the polyomino on A, we try to put the
    first point (0, 1) on A. This gives us the best possible fit.

      +-+-+-+-+-+
      | |X| | | |
      +-+-+-+-+-+
      |X|X|X| | |
      +-+-+-+-+-+
      |X|X| | | |
      +-+-+-+-+-+
      |X|X|X| | |
      +-+-+-+-+-+

    */

    if let Some(target_pt) = board_utils::get_first_unoccupied(&b) {
        if let Some(poly_pt) = p.iter().nth(0) {
            if poly_pt.x <= target_pt.x && poly_pt.y <= target_pt.y {
                return b.add_polyomino(p, Point::new(target_pt.x - poly_pt.x, target_pt.y - poly_pt.y));
            }
        }
    }

    false
}

fn build_w() -> Polyomino {
    let mut p = Vec::new();
    p.push(Point::new(0, 0));
    p.push(Point::new(1, 0));
    p.push(Point::new(1, 1));
    p.push(Point::new(2, 1));
    p.push(Point::new(2, 2));
    
    Polyomino::new(p)
}

fn build_l() -> Polyomino {
    let mut p = Vec::new();
    p.push(Point::new(0, 0));
    p.push(Point::new(0, 1));
    p.push(Point::new(0, 2));
    p.push(Point::new(0, 3));
    p.push(Point::new(1, 3));
    
    Polyomino::new(p)
}

fn build_y() -> Polyomino {
    let mut p = Vec::new();
    p.push(Point::new(0, 1));
    p.push(Point::new(1, 1));
    p.push(Point::new(2, 1));
    p.push(Point::new(3, 1));
    p.push(Point::new(2, 0));
    
    Polyomino::new(p)
}
