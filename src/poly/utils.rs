use std::collections::HashSet;

use poly::point::Point;
use poly::polyomino::Polyomino;

#[allow(dead_code)]
pub fn build_monominoes() -> Vec<Polyomino> {
    let mut r = Vec::new();

    let mut p = Vec::new();
    p.push(Point::new(0,0));
    r.push(Polyomino::new(p));

    r
}

#[allow(dead_code)]
pub fn build_dominoes() -> Vec<Polyomino> {
    let mut r = Vec::new();

    let mut p = Vec::new();
    p.push(Point::new(0, 0));
    p.push(Point::new(1, 0));
    r.push(Polyomino::new(p));

    r
}

#[allow(dead_code)]
pub fn build_triominoes() -> Vec<Polyomino> {
    let mut r = Vec::new();

    let mut p = Vec::new();
    p.push(Point::new(0, 0));
    p.push(Point::new(1, 0));
    p.push(Point::new(2, 0));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0, 0));
    p.push(Point::new(1, 0));
    p.push(Point::new(0, 1));
    r.push(Polyomino::new(p));

    r
}

#[allow(dead_code)]
pub fn build_tetrominoes() -> Vec<Polyomino> {
    let mut r = Vec::new();
    
    let mut p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(1,1));
    p.push(Point::new(1,0));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(1,0));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(0,3));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(1,1));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(1,1));
    p.push(Point::new(1,2));
    r.push(Polyomino::new(p));

    r
}

#[allow(dead_code)]
pub fn build_pentominoes() -> Vec<Polyomino> {
    let mut r = Vec::new();

    let mut p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(0,3));
    p.push(Point::new(0,4));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(0,3));
    p.push(Point::new(1,3));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(1,2));
    p.push(Point::new(0,3));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(1,1));
    p.push(Point::new(2,1));
    p.push(Point::new(0,2));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(1,1));
    p.push(Point::new(1,0));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(1,1));
    p.push(Point::new(1,2));
    p.push(Point::new(1,3));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(0,1));
    p.push(Point::new(1,1));
    p.push(Point::new(1,2));
    p.push(Point::new(2,2));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(1,0));
    p.push(Point::new(1,1));
    p.push(Point::new(1,2));
    p.push(Point::new(2,2));
    p.push(Point::new(0,1));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(1,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    p.push(Point::new(1,2));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(0,0));
    p.push(Point::new(1,0));
    p.push(Point::new(2,0));
    p.push(Point::new(0,1));
    p.push(Point::new(0,2));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(1,0));
    p.push(Point::new(1,1));
    p.push(Point::new(1,2));
    p.push(Point::new(0,0));
    p.push(Point::new(2,2));
    r.push(Polyomino::new(p));

    p = Vec::new();
    p.push(Point::new(1,0));
    p.push(Point::new(1,1));
    p.push(Point::new(1,2));
    p.push(Point::new(0,1));
    p.push(Point::new(2,1));
    r.push(Polyomino::new(p));

    r
}

#[cfg(test)]
mod tests
{
    use utils::*;

    #[test]
    fn count_tetrominoes() {
        let mut i = 0;
        for p in build_tetrominoes() {
            i += p.make_variations().len();
        }
        assert_eq!(i, 19);
    }

    #[test]
    fn count_pentominoes() {
        let mut i = 0;
        for p in build_pentominoes() {
            i += p.make_variations().len();
        }
        assert_eq!(i, 63);
    }
}


// Build all variations of the polyominos (rotated and reflected)
pub fn build_variations(polys: &Vec<Polyomino>) -> Vec<HashSet<Polyomino>> {
    let mut res = Vec::with_capacity(polys.len());

    for p in polys {
        res.push(p.make_all_variations());
    }

    res
}

// Build all variations of the polyominos (rotated and reflected), *except* 
// take one polyomino which has no rotational symmetry (so 8 variations in total)
// and give just two variations of it. The idea is to eliminate rotations and
// reflections on solutions and we do this by enforcing an oritentation of one
// asymmetric polyomino
pub fn build_rect_variations(polys: &Vec<Polyomino>) -> Vec<HashSet<Polyomino>> {
    let mut res = Vec::with_capacity(polys.len());
    let mut found_asym = false;

    for p in polys {
        let mut variations = p.make_all_variations();

        if variations.len() == 8 && !found_asym {
            found_asym = true;
            variations = HashSet::new();
            variations.insert(p.clone());
            variations.insert(p.clone().rotate());
        }

        res.push(variations);
    }

    res
}
