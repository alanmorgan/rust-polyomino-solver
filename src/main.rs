mod poly;

extern crate bit_vec;

use std::collections::HashSet;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::polyomino_utils;

fn main() {

    if let Ok(pentominoes) = polyomino_utils::read_polyomino_file("data/pentomino.txt") {
        let all_pentominoes = polyomino_utils::build_rect_variations(&pentominoes);
        if let Ok(mut b) = Board::from_file("data/b8x8holes.txt") {
            println!("{} solutions\n", board_utils::fill(&mut b, &all_pentominoes));
        }
    }
}
