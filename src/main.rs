mod poly;

use std::collections::HashSet;

extern crate bit_vec;
use bit_vec::BitVec;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::utils;

fn main() {
    let pentominoes = build_variations(&utils::build_pentominoes());
    let mut b = Board::new(3, 20);

    fill(&mut b, &pentominoes);
}

fn fill<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>) {
    let mut usable_candidates = BitVec::from_elem(candidates.len(), true);
    
    fill_board(b, candidates, &usable_candidates);
}

fn build_variations(polys : &Vec<Polyomino>) -> Vec<HashSet<Polyomino>> {
    let mut res = Vec::with_capacity(polys.len());

    for p in polys {
        res.push(p.make_variations());
    }

    res
}

fn fill_board<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>, usable_candidates: &BitVec) {

    if usable_candidates.none() {
        println!("{}", b);
        return;
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
                fill_board(b, candidates, &uc);
                uc.set(n, true);
                b.remove_polyomino(point);
            }
        }
    }
}
