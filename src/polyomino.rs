use std::fmt;
use std::cmp;
use std::collections::HashSet;
use std::slice::Iter;

use point::Point;

#[derive(Debug, Hash, Clone)]
#[allow(dead_code)]
pub struct Polyomino {
    points: Vec<Point>
}

impl PartialEq for Polyomino {
    fn eq(&self, other: &Polyomino) -> bool {
        self.points == other.points
    }

    fn ne(&self, other: &Polyomino) -> bool {
        !self.eq(other)
    }
}

impl Eq for Polyomino {
}

#[allow(dead_code)]
impl Polyomino {
    pub fn new(mut p: Vec<Point>) -> Polyomino {
        p.sort();
        p.dedup();
        Polyomino { points: p }
    }

    // The largest x and y in the set of points. Note: this point may not be in the polyomino
    fn bbox_top_right(&self) -> Point {
        self.points.iter().fold(Point::min_point(), |a, ref p| Point { x: cmp::max(a.x, p.x), y: cmp::max(a.y, p.y) })
    }
    
    pub fn show(&self) {
        // Inefficient, but it hardly matters
        let Point { x: width, y: height } = self.bbox_top_right();
        for y in 0..height+1 {
            for x in 0..width+1 {
                if let Some(_p) = self.points.iter().find(|ref p| p.x == x && p.y == (height-y)) {
                    print!("X");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn rotate(&self) -> Polyomino {
        let height = self.bbox_top_right().y;
        Polyomino::new(self.points.iter().map(|p| Point {x: height-p.y, y: p.x}).collect())
    }

    pub fn flip(&self) -> Polyomino {
        let width = self.bbox_top_right().x;
        Polyomino::new(self.points.iter().map(|p| Point {x: width-p.x, y: p.y}).collect())
    }

    pub fn iter(&self) -> Iter<Point> {
        self.points.iter()
    }

    fn make_rotations(&self) -> Vec<Polyomino> {
        let mut res = HashSet::new();
        
        res.insert(self.clone());
        res.insert(self.clone().rotate());
        res.insert(self.clone().rotate().rotate());
        res.insert(self.clone().rotate().rotate().rotate());

        let rotations = res.drain().collect();
        rotations
    }

    pub fn make_all_variations(&self) -> Vec<Polyomino> {
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

impl fmt::Display for Polyomino {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // Inefficient, but it hardly matters
        let Point { x: width, y: height } = self.bbox_top_right();
        for y in 0..height+1 {
            for x in 0..width+1 {
                if let Some(_p) = self.points.iter().find(|ref p| p.x == x && p.y == (height-y)) {
                    try!(write!(f, "X"));
                } else {
                    try!(write!(f, " "));
                }
            }
            try!(writeln!(f, ""));
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
    use point::PointPos;
    use polyomino::Polyomino;

    #[allow(dead_code)]
    pub enum Restrictions {
        None,
        SquareSymmetry,
        RectangularSymmetry
        // SingleSided
    }

    #[allow(dead_code)]
    pub fn build_variations(polys: &Vec<Polyomino>, restrict: Restrictions) -> Vec<Vec<Polyomino>> {
        let mut res = Vec::with_capacity(polys.len());
        let mut found_asym = false;

        for p in polys {
            let mut variations = p.make_all_variations();

            match *&restrict {
                Restrictions::None => (),
                Restrictions::SquareSymmetry => {
                    if !found_asym && variations.len() == 8 {
                        found_asym = true;
                        variations = Vec::new();
                        variations.push(p.clone());
                    }
                },
                Restrictions::RectangularSymmetry => {
                    if !found_asym && variations.len() == 8 {
                        found_asym = true;
                        variations = Vec::new();
                        variations.push(p.clone());
                        variations.push(p.clone().rotate());
                    }
                }
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
                            points.push(Point::new(count, i as PointPos));
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

}

#[cfg(test)]
mod tests {
    use point::Point;
    use polyomino::Polyomino;

    use polyomino::polyomino_utils::read_polyomino_file;
        
    #[test]
    fn test_read() {
        match read_polyomino_file("data/domino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 1);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/tetromino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 5);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/pentomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 12);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/hexomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 35);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/heptomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 108);
            },
            Err(..) => assert!(false),
        }
        match read_polyomino_file("data/octomino.poly") {
            Ok(polys) => {
                assert_eq!(polys.len(), 369);
            },
            Err(..) => assert!(false),
        }
    }

    fn build_f_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(Point::new(0,1));
        v.push(Point::new(1,1));
        v.push(Point::new(1,0));
        v.push(Point::new(2,2));
        v.push(Point::new(1,2));
        
        Polyomino::new(v)
    }

    // Add points in different order, with duplicate
    fn build_alt_f_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(Point::new(2,2));
        v.push(Point::new(1,0));
        v.push(Point::new(1,2));
        v.push(Point::new(1,0));
        v.push(Point::new(1,1));
        v.push(Point::new(0,1));
        
        Polyomino::new(v)
    }

    fn build_i_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(Point::new(0,0));
        v.push(Point::new(0,1));
        v.push(Point::new(0,2));
        v.push(Point::new(0,3));
        v.push(Point::new(0,4));

        Polyomino::new(v)
    }

    fn build_alt_i_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(Point::new(0,4));
        v.push(Point::new(0,3));
        v.push(Point::new(0,2));
        v.push(Point::new(0,1));
        v.push(Point::new(0,0));

        Polyomino::new(v)
    }

    fn build_v_pentomino() -> Polyomino {
        let mut v = Vec::new();
        v.push(Point::new(0,0));
        v.push(Point::new(0,1));
        v.push(Point::new(0,2));
        v.push(Point::new(1,0));
        v.push(Point::new(2,0));

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
