use std::fs;
use std::hash::Hash;
use std::io::Error;

use lazy_static::lazy_static;

use rustc_hash::FxHashMap;

use crate::point::Point;
use crate::polyomino::Polyomino;
use crate::polyomino::TagTrait;

#[allow(dead_code)]
pub enum Restrictions {
    None,
    SquareSymmetry,
    RectangularSymmetry, // SingleSided
}

#[allow(dead_code)]
pub fn build_variations<S:TagTrait, T:Point>(polys: &Vec<Polyomino<S, T>>, restrict: Restrictions) -> Vec<Vec<Polyomino<S, T>>> {
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

pub fn get_polyominoes<S:TagTrait, T:Point>(polytype: PredefinedPolyominoes, make_point:&dyn Fn(i16, i16) -> T) -> Result<Vec<Polyomino<S, T>>, Error> {
    read_polyomino_string(HASHMAP.get(&polytype).unwrap(), make_point)
}

pub fn read_polyominoes_from_file<S:TagTrait, T:Point>(name: &str, make_point:&dyn Fn(i16, i16) -> T ) -> Result<Vec<Polyomino<S, T>>, Error> {
    let contents = fs::read_to_string(name)?;
    
    read_polyomino_string(&contents, make_point)
}

fn read_polyomino_string<S:TagTrait, T:Point>(contents: &str, make_point:&dyn Fn(i16, i16) -> T) -> Result<Vec<Polyomino<S, T>>, Error> {
    let mut res = Vec::new();

    let mut count = 0;
    let mut points = Vec::new();

    for line in contents.split('\n') {
        match line {
            // polyominoes are separated by empty lines.
            "" => {
                res.push(Polyomino::basic(points));
                points = Vec::new();
                count = 0;
            }
            // anything else is a definition
            str => {
                for (i, c) in str.chars().enumerate() {
                    if c != ' ' {
                        points.push(make_point(count, i as i16));
                    }
                }
                count += 1;
            }
        }
    }

    if !points.is_empty() {
        res.push(Polyomino::basic(points));
    }

    Ok(res)
}
