use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;

use point::Point;
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

pub fn read_polyomino_file(name: &str) -> Result<Vec<Polyomino<Point>>, Error> {
    let mut res = Vec::new();

    let f = File::open(name)?;

    let buff_file = BufReader::new(&f);

    let mut count = 0;
    let mut points = Vec::new();

    for line in buff_file.lines() {
        match line.unwrap().as_ref() {
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
                        points.push(Point::new(count, i as PointPos));
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
