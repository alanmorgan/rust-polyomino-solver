mod poly;

extern crate bit_vec;
use bit_vec::BitVec;

use std::collections::HashSet;

use poly::board::Board;
use poly::board::BoardState;
use poly::board::board_utils;
use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::utils;

fn main() {
    if let Ok(pentominoes) = utils::read_polyomino_file("data/pentomino.txt") {
        let all_pentominoes = utils::build_rect_variations(&pentominoes);
        let mut b = Board::new(20, 3);
        println!("{} solutions\n", board_utils::fill(&mut b, &all_pentominoes));
    }
}
