mod poly;

use std::collections::HashSet;

extern crate bit_vec;
use bit_vec::BitVec;

use poly::board::Board;
use poly::board::BoardState;
use poly::board::board_utils;
use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::utils;

fn main() {
    let pentominoes = utils::build_variations(&utils::build_pentominoes());

    let mut b = Board::new(3, 20);

    println!("{} solutions\n", fill(&mut b, &pentominoes));
}

fn fill<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>) -> i32 {
    let usable_candidates = BitVec::from_elem(candidates.len(), true);
    
    fill_board(b, candidates, &usable_candidates)
}

fn fill_board<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>, usable_candidates: &BitVec) -> i32 {
    let mut total = 0;

    if usable_candidates.none() {
        println!("{}", b);
        return 1;
    }

    let mut uc = usable_candidates.clone();

    let foo = usable_candidates.iter().
        enumerate().
        zip(candidates.iter()).
        filter_map(|((n, b), polys)|
                   if b {
                       Some((n, polys))
                   } else {
                       None
                   });

    for (n, polys) in foo {
        for poly in polys {
            if let Some(point) = board_utils::fit(b, &poly) {
                uc.set(n, false);
                total += fill_board(b, candidates, &uc);
                uc.set(n, true);
                b.remove_polyomino(point);
            }
        }
    }

    total
}
