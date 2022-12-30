use std::cmp;
use std::collections::HashSet;
use std::fmt;
use std::slice::Iter;

use point::PointT;
use point::Point;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Polyomino<T: PointT> {
    points: Vec<T>,
}

#[allow(dead_code)]
impl <T:PointT> Polyomino<T> {
    pub fn new(mut p: Vec<T>) -> Polyomino<T> {
        p.sort();
        p.dedup();
        Polyomino { points: p }
    }

    // The largest x and y in the set of points. Note: this point may not be in the polyomino
    fn bbox_top_right(&self) -> Point {
        self.points.iter().fold(Point::new(0, 0), |a, p| Point {
            x: cmp::max(a.x(), p.x()),
            y: cmp::max(a.y(), p.y()),
        })
    }

    pub fn show(&self) {
        // Inefficient, but it hardly matters
        let Point {
            x: width,
            y: height,
        } = self.bbox_top_right();
        for y in 0..height + 1 {
            for x in 0..width + 1 {
                if let Some(_p) = self.points.iter().find(|p| p.x() == x && p.y() == (height - y)) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    pub fn rotate(&self) -> Polyomino<T> {
        let height = self.bbox_top_right().y();
        Polyomino::new(
            self.points
                .iter()
                .map(|p| { let mut new_p:T = *p; new_p.set_x(height - p.y()); new_p.set_y(p.x()); new_p })
                .collect(),
        )
    }

    pub fn flip(&self) -> Polyomino<T> {
        let width = self.bbox_top_right().x();
        Polyomino::new(
            self.points
                .iter()
                .map(|p| { let mut new_p:T = *p; new_p.set_x(width - p.x()); new_p.set_y(p.y()); new_p })
                .collect(),
        )
    }

    pub fn iter(&self) -> Iter<T> {
        self.points.iter()
    }

    fn make_rotations(&self) -> Vec<Polyomino<T>> {
        let mut res = HashSet::new();

        res.insert(self.clone());
        res.insert(self.clone().rotate());
        res.insert(self.clone().rotate().rotate());
        res.insert(self.clone().rotate().rotate().rotate());

        let rotations = res.drain().collect();
        rotations
    }

    pub fn make_all_variations(&self) -> Vec<Polyomino<T>> {
        let mut res = HashSet::new();

        res.insert(self.clone());
        res.insert(self.clone().rotate());
        res.insert(self.clone().rotate().rotate());
        res.insert(self.clone().rotate().rotate().rotate());

        res.insert(self.clone().flip());
        res.insert(self.clone().flip().rotate());
        res.insert(self.clone().flip().rotate().rotate());
        res.insert(self.clone().flip().rotate().rotate().rotate());

        let rotations = res.drain().collect();
        rotations
    }
}

impl<T:PointT> fmt::Display for Polyomino<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Inefficient, but it hardly matters
        let Point {
            x: width,
            y: height,
        } = self.bbox_top_right();
        for y in 0..height + 1 {
            for x in 0..width + 1 {
                if let Some(_p) = self.points.iter().find(|p| p.x() == x && p.y() == (height - y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub mod polyomino_utils {
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
}

#[cfg(test)]
mod tests {
    use point::PointT;
    use polyomino::Polyomino;

    use polyomino::polyomino_utils::read_polyomino_file;

    #[test]
    fn test_read() {
        match read_polyomino_file("data/domino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 1);
            }
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/tetromino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 5);
            }
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/pentomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 12);
            }
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/hexomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 35);
            }
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/heptomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 108);
            }
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/octomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 369);
            }
            Err(..) => assert!(false),
        }
    }

    fn build_f_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(PointT::new(0, 1));
        v.push(PointT::new(1, 1));
        v.push(PointT::new(1, 0));
        v.push(PointT::new(2, 2));
        v.push(PointT::new(1, 2));

        Polyomino::new(v)
    }

    // Add points in different order, with duplicate
    fn build_alt_f_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(PointT::new(2, 2));
        v.push(PointT::new(1, 0));
        v.push(PointT::new(1, 2));
        v.push(PointT::new(1, 0));
        v.push(PointT::new(1, 1));
        v.push(PointT::new(0, 1));

        Polyomino::new(v)
    }

    fn build_i_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(PointT::new(0, 0));
        v.push(PointT::new(0, 1));
        v.push(PointT::new(0, 2));
        v.push(PointT::new(0, 3));
        v.push(PointT::new(0, 4));

        Polyomino::new(v)
    }

    fn build_alt_i_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(PointT::new(0, 4));
        v.push(PointT::new(0, 3));
        v.push(PointT::new(0, 2));
        v.push(PointT::new(0, 1));
        v.push(PointT::new(0, 0));

        Polyomino::new(v)
    }

    fn build_v_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(PointT::new(0, 0));
        v.push(PointT::new(0, 1));
        v.push(PointT::new(0, 2));
        v.push(PointT::new(1, 0));
        v.push(PointT::new(2, 0));

        Polyomino::new(v)
    }

    #[test]
    fn count() {
        assert_eq!(build_f_pentomino().iter().count(), 5);
    }

    #[test]
    fn rot() {
        let f = build_f_pentomino();
        let f_alt = build_alt_f_pentomino();
        let i = build_i_pentomino();
        let i_alt = build_alt_i_pentomino();

        assert!(f == f);
        assert!(f == f_alt);
        assert!(i == i_alt);
        assert!(i == i.rotate().rotate());
        assert!(i == i.flip());
        assert!(f != f.rotate());
        assert!(f == f.rotate().rotate().rotate().rotate());
        assert!(f.rotate().rotate().flip() == f.flip().rotate().rotate());
    }

    #[test]
    fn variations() {
        assert_eq!(build_f_pentomino().make_all_variations().len(), 8);
        assert_eq!(build_i_pentomino().make_all_variations().len(), 2);
        assert_eq!(build_v_pentomino().make_all_variations().len(), 4);
    }
}
