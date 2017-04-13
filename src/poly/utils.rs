use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

use poly::point::Point;
use poly::polyomino::Polyomino;

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

pub fn read_polyomino_file(name: &str) -> Result<Vec<Polyomino>, Error> {
    let mut res = Vec::new();

    let f = try!(File::open(name));

    let buff_file = BufReader::new(&f);

    let mut count = 0;
    let mut points = Vec::new();

    for line in buff_file.lines() {
        match line.unwrap().as_ref() {
            // polyominoes are separated by empty lines.
            "" => { res.push(Polyomino::new(points));
                    points = Vec::new();
                    count = 0;
            },
            // anything else is a definition
            str => {
                for (i, c) in str.chars().enumerate() {
                    if c != ' ' {
                        points.push(Point::new(count, i));
                    }
                }
                count=count+1;
            },
        }
    }

    if !points.is_empty() {
        res.push(Polyomino::new(points));
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use utils::*;

    #[test]
    fn test_read() {
        match read_polyomino_file("data/domino.txt") {
            Ok(polys) => {
                assert_eq!(polys.len(), 1);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/tetromino.txt") {
            Ok(polys) => {
                assert_eq!(polys.len(), 5);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/pentomino.txt") {
            Ok(polys) => {
                assert_eq!(polys.len(), 12);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/hexomino.txt") {
            Ok(polys) => {
                assert_eq!(polys.len(), 35);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/heptomino.txt") {
            Ok(polys) => {
                assert_eq!(polys.len(), 108);
            },
            Err(..) => assert!(false),
        }
    }
}
