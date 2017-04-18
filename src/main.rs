mod poly;

extern crate bit_vec;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::polyomino_utils;

fn main() {

    if let Ok(pentominoes) = polyomino_utils::read_polyomino_file("data/pentomino.poly") {
        let all_pentominoes = polyomino_utils::build_square_variations(&pentominoes);
        if let Ok(mut b) = Board::from_file("data/b8x8nocorners.board") {
            println!("{} solutions\n", board_utils::fill(&mut b, &all_pentominoes));
        }
        else
        {
            panic!("Can't find board data/b8x8holes.board");
        }
    }
    else
    {
        panic!("Can't find polyomino file data/pentomino.poly");
    }
}
