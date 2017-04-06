mod poly;

use std::collections::HashSet;

use poly::board::Board;
use poly::board::board_utils;
use poly::polyomino::Polyomino;
use poly::point::Point;
use poly::utils;

fn main() {
    let mut b = Board::new(3, 20);
    let mut candidates = get_working_set();

    /*
    let mut candidates = utils::build_pentominoes();
    let mut rejected = Vec::new();

    for p in candidates {
        for variation in p.make_variations() {
            // Should fit take ownership of the value instead of a ref? What to do if it fails?
            if board_utils::fit(&mut b, variation) {
                // Found a match. All the rejected polys are now possible candidates again
                // candidates.extend(rejected.iter());
                break;
            }
        }
        // None of the variations worked
        rejected.push(p);
    }
    */
}

fn get_working_set() -> Vec<HashSet<Polyomino>> {
    build_variations(utils::build_pentominoes())
}

fn build_variations(polys : Vec<Polyomino>) -> Vec<HashSet<Polyomino>> {
    let mut res = Vec::new();

    for p in polys {
        res.push(p.make_variations());
    }

    res
}

/*
fn fill_board<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>) {
    if candidates.is_empty() {
        println!("{}", b);
        return;
    }

    // Try each polyomino in every orientation in turn
    for (idx, poly_variations) in candidates.iter().enumerate() {
        // build new vector, minus this element

        // I can see this is going to be *horribly* inefficient
        let l = candidates.iter().enumerate().filter(|&(i,_)| i != idx).map(|(_, e)| e.clone());
        let remaining_candidates:Vec<HashSet<Polyomino>> = l.collect();

        for p in poly_variations {
            if let Some(point) = board_utils::fit(b, &p) {
                fill_board(b, &remaining_candidates);
                b.remove_polyomino(point);
            }
        }
    }
}
*/
