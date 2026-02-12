use std::cmp;
use std::fmt;
use std::hash::Hash;
use std::marker::Sized;
use std::slice::Iter;

use rustc_hash::FxHashSet;

use crate::point::Point;
use crate::point::SimplePoint;

pub trait Polyomino : Sized + fmt::Display + Hash + Clone + PartialEq + Eq {
    type Pt : Point;

    fn new(ptrs: Vec<Self::Pt>) -> Self;
    
    fn iter(&self) -> Iter<'_, Self::Pt>;

    fn set_points(&mut self, pts: Vec<Self::Pt>);

    fn get_nth(&self, n: usize) -> Option<&Self::Pt>;
    
    // The largest x and y in the set of points. Note: this point may not be in the polyomino
    fn bbox_top_right(&self) -> SimplePoint {
        self.iter().fold(SimplePoint::new(0, 0), |a, p| SimplePoint {
            x: cmp::max(a.x(), p.x()),
            y: cmp::max(a.y(), p.y()),
        })
    }
    
    fn rotate(&self) -> Self {
        let mut rot = self.clone();
        let height = rot.bbox_top_right().y();
        
        rot.set_points(
            self.iter()
                .map(|p| { let mut new_p:Self::Pt = *p; new_p.set_x(height - p.y()); new_p.set_y(p.x()); new_p })
                .collect());

        rot
    }
    
    fn flip(&self) -> Self {
        let mut flipped = self.clone();
        let width = flipped.bbox_top_right().x();

        flipped.set_points(
            self.iter()
                .map(|p| { let mut new_p:Self::Pt = *p; new_p.set_x(width - p.x()); new_p.set_y(p.y()); new_p })
                .collect(),
            );

        flipped
    }
            
    fn make_rotations(&self) -> Vec<Self> {
        let mut res = FxHashSet::default();

        res.insert(self.clone());
        res.insert(self.clone().rotate());
        res.insert(self.clone().rotate().rotate());
        res.insert(self.clone().rotate().rotate().rotate());

        let rotations = res.drain().collect();
        rotations
    }

   fn make_all_variations(&self) -> Vec<Self> {
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
    
    fn show(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Inefficient, but it hardly matters
        let SimplePoint {
            x: width,
            y: height,
        } = self.bbox_top_right();
        for y in 0..height + 1 {
            for x in 0..width + 1 {
                if let Some(pt) = self.iter().find(|p| p.x() == x && p.y() == (height - y)) {
                    write!(f, "{}", pt.to_string())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct SimplePolyomino<T: Point> {
    points: Vec<T>,
}

impl <T: Point> Polyomino for SimplePolyomino<T> {
    type Pt = T;
    
    fn new(mut points : Vec<T>) -> SimplePolyomino<T> {
        points.sort();
        points.dedup();
        SimplePolyomino{ points: points}
    }

    fn iter(&self) -> Iter<'_, Self::Pt> {
        self.points.iter()
    }

    fn get_nth(&self, nth: usize) -> Option<&Self::Pt> {
        self.points.get(nth)
    }
    
    fn set_points(&mut self, mut points: Vec<Self::Pt>) {
        points.sort();
        points.dedup();
        self.points = points;
    }
}


impl <T: Point> fmt::Display for SimplePolyomino<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.show(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::point::Point;
    use crate::point::SimplePoint;
    use crate::polyomino::Polyomino;
    use crate::polyomino::SimplePolyomino;
    use crate::utils;
    use crate::utils::PredefinedPolyominoes;
    
    #[test]
    fn test_read() {
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Dominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 1);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Triominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 2);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Tetrominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 5);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Pentominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 12);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Hexominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 35);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Heptominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 108);
            }
            Err(..) => assert!(false),
        }
        match utils::get_polyominoes::<SimplePolyomino<SimplePoint>>(PredefinedPolyominoes::Octominoes) {
            Ok(polys) => {
                assert_eq!(polys.len(), 369);
            }
            Err(..) => assert!(false),
        }
    }

    fn build_f_pentomino() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(0, 1));
        v.push(SimplePoint::new(1, 1));
        v.push(SimplePoint::new(1, 0));
        v.push(SimplePoint::new(2, 2));
        v.push(SimplePoint::new(1, 2));

        SimplePolyomino::new(v)
    }

    // Add points in different order, with duplicate
    fn build_alt_f_pentomino() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(2, 2));
        v.push(SimplePoint::new(1, 0));
        v.push(SimplePoint::new(1, 2));
        v.push(SimplePoint::new(1, 0));
        v.push(SimplePoint::new(1, 1));
        v.push(SimplePoint::new(0, 1));

        SimplePolyomino::new(v)
    }

    fn build_i_pentomino() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(0, 0));
        v.push(SimplePoint::new(0, 1));
        v.push(SimplePoint::new(0, 2));
        v.push(SimplePoint::new(0, 3));
        v.push(SimplePoint::new(0, 4));

        SimplePolyomino::new(v)
    }

    fn build_alt_i_pentomino() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(0, 4));
        v.push(SimplePoint::new(0, 3));
        v.push(SimplePoint::new(0, 2));
        v.push(SimplePoint::new(0, 1));
        v.push(SimplePoint::new(0, 0));

        SimplePolyomino::new(v)
    }

    fn build_v_pentomino() -> SimplePolyomino<SimplePoint> {
        let mut v = Vec::new();
        v.push(SimplePoint::new(0, 0));
        v.push(SimplePoint::new(0, 1));
        v.push(SimplePoint::new(0, 2));
        v.push(SimplePoint::new(1, 0));
        v.push(SimplePoint::new(2, 0));

        SimplePolyomino::new(v)
    }

    #[test]
    fn count() {
        assert_eq!(build_f_pentomino().iter().count(), 5);
    }

    #[test]
    fn f_is_f() {
        let f = build_f_pentomino();
        assert!(f == f);
    }

    #[test]
    fn f_is_f_alt() {
        let f = build_f_pentomino();
        let f_alt = build_alt_f_pentomino();
        
        assert!(f == f_alt);
    }

    #[test]
    fn i_is_i_alt() {
        let i = build_i_pentomino();
        let i_alt = build_alt_i_pentomino();

        assert!(i == i_alt);
    }

    #[test]
    fn i_is_i_double_rot() {
        let i = build_i_pentomino();
        
        assert!(i == i.rotate().rotate());
    }
    
    #[test]
    fn i_is_i_flip() {
        let i = build_i_pentomino();
        assert!(i == i.flip());
    }

    #[test]
    fn f_is_not_f_rot() {
        let f = build_f_pentomino();
        
        assert!(f != f.rotate());
    }
    
    #[test]
    fn f_is_f_360() {
        let f = build_f_pentomino();
        assert!(f == f.rotate().rotate().rotate().rotate());
    }

    #[test]
    fn f_flip_rotate() {
        let f = build_f_pentomino();
        
        assert!(f.rotate().rotate().flip() == f.flip().rotate().rotate());
    }
    
    #[test]
    fn variations_f() {
        assert_eq!(build_f_pentomino().make_all_variations().len(), 8);
    }
    
    #[test]
    fn variations_i() {
        assert_eq!(build_i_pentomino().make_all_variations().len(), 2);
    }
    
    #[test]
    fn variations_v() {
        assert_eq!(build_v_pentomino().make_all_variations().len(), 4);
    }
}
















