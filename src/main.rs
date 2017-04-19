mod poly;

extern crate bit_vec;

use std::time::Instant;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::polyomino_utils;

fn main() {
    let board_name = "data/cross.board";
    let polyomino_name = "data/pentomino.poly";

    if let Ok(polyominoes) = polyomino_utils::read_polyomino_file(polyomino_name) {
        let all_polyominoes = polyomino_utils::build_rect_variations(&polyominoes);
        let mut b = Board::new(12, 5);
        let start_time = Instant::now();

        let num_solutions = board_utils::fill(&mut b, &all_polyominoes);
        let elapsed = start_time.elapsed();
        let elapsed_millis = elapsed.as_secs() * 1000 as u64 + (elapsed.subsec_nanos() / 1000000) as u64;

        println!("{} solutions found in {}ms ({} solutions/second)", num_solutions, elapsed_millis, (num_solutions as f64/elapsed_millis as f64)*1000.0);

    }
    else
    {
        panic!("Can't find polyomino file {}", polyomino_name);
    }
}
