use std::fs;
use std::io::Error;

use rustc_hash::FxHashMap;

use point::PointT;
use point::PointPos;
use polyomino::Polyomino;

#[allow(dead_code)]
pub enum Restrictions {
    None,
    SquareSymmetry,
    RectangularSymmetry, // SingleSided
}

#[allow(dead_code)]
pub fn build_variations<T:PointT>(polys: &Vec<Polyomino<T>>, restrict: Restrictions) -> Vec<Vec<Polyomino<T>>> {
    let mut res = Vec::with_capacity(polys.len());
    let mut found_asym = false;

    for p in polys {
        let mut variations = p.make_all_variations();

        match restrict {
            Restrictions::None => (),
            Restrictions::SquareSymmetry => {
                if !found_asym && variations.len() == 8 {
                    found_asym = true;
                    variations = vec![p.clone()];
                }
            }
            Restrictions::RectangularSymmetry => {
                if !found_asym && variations.len() == 8 {
                    found_asym = true;
                    variations = vec![p.clone(), p.clone().rotate()];
                }
            }
        }
        res.push(variations);
    }

    res
}

#[derive(Eq,Hash,PartialEq)]
pub enum PredefinedPolyominoes {
    Monominoes,
    Dominoes,
    Triominoes,
    Tetrominoes,
    Pentominoes,
    Hexominoes,
    Heptominoes,
    Octominoes,
}

lazy_static! {
    static ref HASHMAP: FxHashMap<PredefinedPolyominoes, &'static str> = {
        let mut hm = FxHashMap::default();
        hm.insert(PredefinedPolyominoes::Monominoes, include_str!("../data/monomino.poly"));
        hm.insert(PredefinedPolyominoes::Dominoes, include_str!("../data/domino.poly"));
        hm.insert(PredefinedPolyominoes::Triominoes, include_str!("../data/triomino.poly"));
        hm.insert(PredefinedPolyominoes::Tetrominoes, include_str!("../data/tetromino.poly"));
        hm.insert(PredefinedPolyominoes::Pentominoes, include_str!("../data/pentomino.poly"));
        hm.insert(PredefinedPolyominoes::Hexominoes, include_str!("../data/hexomino.poly"));
        hm.insert(PredefinedPolyominoes::Heptominoes, include_str!("../data/heptomino.poly"));
        hm.insert(PredefinedPolyominoes::Octominoes, include_str!("../data/octomino.poly"));
        hm
    };
}

pub fn get_polyominoes<T:PointT>(polytype: PredefinedPolyominoes, make_point:&dyn Fn(PointPos, PointPos) -> T) -> Result<Vec<Polyomino<T>>, Error> {
    read_polyomino_string(&HASHMAP.get(&polytype).unwrap().to_string(), make_point)
}

pub fn read_polyominoes_from_file<T:PointT>(name: &str, make_point:&dyn Fn(PointPos, PointPos) -> T ) -> Result<Vec<Polyomino<T>>, Error> {
    let contents = fs::read_to_string(name)?;
    
    read_polyomino_string(&contents, make_point)
}

fn read_polyomino_string<T:PointT>(contents: &String, make_point:&dyn Fn(PointPos, PointPos) -> T) -> Result<Vec<Polyomino<T>>, Error> {
    let mut res = Vec::new();

    let mut count = 0;
    let mut points = Vec::new();

    for line in contents.split('\n') {
        match line {
            // polyominoes are separated by empty lines.
            "" => {
                res.push(Polyomino::new(points));
                points = Vec::new();
                count = 0;
            }
            // anything else is a definition
            str => {
                for (i, c) in str.chars().enumerate() {
                    if c != ' ' {
                        points.push(make_point(count, i as PointPos));
                    }
                }
                count += 1;
            }
        }
    }

    if !points.is_empty() {
        res.push(Polyomino::new(points));
    }

    Ok(res)
}
