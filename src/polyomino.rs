use std::cmp;
use std::fmt;
use std::hash::Hash;
use std::slice::Iter;

use rustc_hash::FxHashSet;

use crate::point::Pt;
use crate::point::Point;

pub trait TagTrait: PartialEq + Eq + Hash + Copy + Default {}
impl <T> TagTrait for T where T: PartialEq + Eq + Hash + Copy + Default {}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Polyomino<S: TagTrait, T: Pt> {
    tag: S,
    points: Vec<T>,
}

#[allow(dead_code)]
impl <S: TagTrait, T:Pt> Polyomino<S, T> {
    pub fn new(new_tag: S, mut p: Vec<T>) -> Polyomino<S, T> {
        p.sort();
        p.dedup();
        Polyomino { tag: new_tag, points: p }
    }
    
    pub fn basic(p: Vec<T>) -> Polyomino<S, T> {
        Self::new(Default::default(), p)
    }
    
    pub fn set_tag(&mut self, new_tag: S) {
        self.tag = new_tag.clone();
    }

    pub fn get_tag(&self) -> S {
        self.tag.clone()
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

    pub fn rotate(&self) -> Polyomino<S, T> {
        let height = self.bbox_top_right().y();
        Polyomino::new(
            self.tag,
            self.points
                .iter()
                .map(|p| { let mut new_p:T = *p; new_p.set_x(height - p.y()); new_p.set_y(p.x()); new_p })
                .collect(),
        )
    }

    pub fn flip(&self) -> Polyomino<S, T> {
        let width = self.bbox_top_right().x();
        Polyomino::new(
            self.tag,
            self.points
                .iter()
                .map(|p| { let mut new_p:T = *p; new_p.set_x(width - p.x()); new_p.set_y(p.y()); new_p })
                .collect(),
        )
    }

    pub fn iter(&self) -> Iter<T> {
        self.points.iter()
    }

    pub fn get_nth(&self, nth: usize) -> Option<&T> {
        self.points.get(nth)
    }
    
    fn make_rotations(&self) -> Vec<Polyomino<S, T>> {
        let mut res = FxHashSet::default();

        res.insert(self.clone());
        res.insert(self.clone().rotate());
        res.insert(self.clone().rotate().rotate());
        res.insert(self.clone().rotate().rotate().rotate());

        let rotations = res.drain().collect();
        rotations
    }

    pub fn make_all_variations(&self) -> Vec<Polyomino<S, T>> {
        let mut res = FxHashSet::default();

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

impl<S:TagTrait, T:Pt> fmt::Display for Polyomino<S, T> {
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

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::polyomino::Polyomino;
    use crate::polyomino::TagTrait;
    use crate::utils;
    use crate::utils::PredefinedPolyominoes;
    
    #[test]
    fn test_read() {
        match utils::get_polyominoes(PredefinedPolyominoes::Dominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 1);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes(PredefinedPolyominoes::Triominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 2);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes(PredefinedPolyominoes::Tetrominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 5);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes(PredefinedPolyominoes::Pentominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 12);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes(PredefinedPolyominoes::Hexominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 35);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes(PredefinedPolyominoes::Heptominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 108);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes(PredefinedPolyominoes::Octominoes, &Point::new) {
            Ok(polys) => {
                assert_eq!(polys.len(), 369);
            }
            Err(..) => assert!(false),
        }
    }

    fn build_f_pentomino() -> Polyomino<(), Point> {
        let mut v = Vec::new();
        v.push(Point::new(0, 1));
        v.push(Point::new(1, 1));
        v.push(Point::new(1, 0));
        v.push(Point::new(2, 2));
        v.push(Point::new(1, 2));

        Polyomino::new(v)
    }

    // Add points in different order, with duplicate
    fn build_alt_f_pentomino() -> Polyomino<(), Point> {
        let mut v = Vec::new();
        v.push(Point::new(2, 2));
        v.push(Point::new(1, 0));
        v.push(Point::new(1, 2));
        v.push(Point::new(1, 0));
        v.push(Point::new(1, 1));
        v.push(Point::new(0, 1));

        Polyomino::new(v)
    }

    fn build_i_pentomino() -> Polyomino<(), Point> {
        let mut v = Vec::new();
        v.push(Point::new(0, 0));
        v.push(Point::new(0, 1));
        v.push(Point::new(0, 2));
        v.push(Point::new(0, 3));
        v.push(Point::new(0, 4));

        Polyomino::new(v)
    }

    fn build_alt_i_pentomino() -> Polyomino<Point> {
        let mut v = Vec::new();
        v.push(Point::new(0, 4));
        v.push(Point::new(0, 3));
        v.push(Point::new(0, 2));
        v.push(Point::new(0, 1));
        v.push(Point::new(0, 0));

        Polyomino::new(v)
    }

    fn build_v_pentomino() -> Polyomino<Point> {
        let mut v = Vec::new();
        v.push(Point::new(0, 0));
        v.push(Point::new(0, 1));
        v.push(Point::new(0, 2));
        v.push(Point::new(1, 0));
        v.push(Point::new(2, 0));

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
