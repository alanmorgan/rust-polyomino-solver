use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::ops::Range;

use poly::point::Point;
use poly::polyomino::Polyomino;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoardState<'a> {
     Void,      // Out of bounds/a hole in the board
     Empty,     // A valid part of the board, but no piece is there
     Full(& 'a Polyomino, usize, usize)  // Has a piece
}

impl<'a> fmt::Display for BoardState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.rep())
    }
}

impl<'a> BoardState<'a> {
    pub fn rep(&self) -> &str {
        match *self {
            BoardState::Void => " ",
            BoardState::Empty => ".",
            BoardState::Full(_p, _x, _y) => " "
        }
    }

}

pub struct Board<'a> {
    height: usize,
    width: usize,
    board: Vec<BoardState<'a>>
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn print_top_row_border(s: &Board, f: &mut fmt::Formatter) -> fmt::Result {
            try!(f.write_str(if s.get(0,0) == BoardState::Void {
                " " 
            } else {
                "+"
            }));

            for x in 0 .. s.width {
                let piece = s.get(x, 0);
                
                try!(f.write_str(if piece == BoardState::Void {
                    if s.get(x+1, 0) == BoardState::Void {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                }));
            }

            f.write_str("\n")
        }
        
        fn print_row(s: &Board, f: &mut fmt::Formatter, y: usize) -> fmt::Result {
            for x in 0 .. s.width {
                let piece = s.get(x, y);
                
                if x == 0 {
                    try!(f.write_str(if piece == BoardState::Void {
                        " "
                    } else {
                        "|"
                    }));
                }

                try!(f.write_str(piece.rep()));

                try!(f.write_str(if piece == s.get(x+1, y) {
                    " "
                } else {
                    "|"
                }));
            }

            try!(f.write_str("\n"));

            print_row_bottom_border(s, f, y)
        }
        
        fn print_row_bottom_border(s: &Board, f: &mut fmt::Formatter, y: usize) -> fmt::Result {
            try!(f.write_str(if s.get(0,y) == BoardState::Void {
                " " 
            } else {
                "+"
            }));

            for x in 0 .. s.width {
                let piece = s.get(x, y);
                
                try!(f.write_str(if piece == s.get(x, y+1) {
                    if piece == s.get(x+1, y) && s.get(x, y+1) == s.get(x+1, y+1) {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                }));
            }

            f.write_str("\n")
        }

        
        try!(print_top_row_border(self, f));

        for y in 0 .. self.height {
            try!(print_row(self, f, y));
        }

        Ok(())
    }

}

#[allow(dead_code)]
impl<'a> Board<'a> {
    pub fn new(w: usize, h: usize) -> Board<'a> {
        Board { height: h,
                width: w,
                board: vec![BoardState::Empty; h*w] }
    }

    pub fn from_file(name: &str) -> Result<Board, Error> {
        let f = try!(File::open(name));

        let buf_file = BufReader::new(&f);

        let mut board_x:usize = 0;
        let mut board_y:usize = 0;
        let mut lines = Vec::new();

        for wline in buf_file.lines() {
            let line = wline.unwrap();
            board_x = cmp::max(board_x, line.len());
            lines.push(line);
            board_y = board_y+1;
        }

        let mut b = Board::new(board_x, board_y);

        let mut y = 0;
        for line in lines {
            // We could treat each space as a void, but trailing
            // spaces may not exist. Instead, mark each space as
            // void and then mark spaces as empty

            for x in 0..board_x {
                let idx = b.to_idx(x, y);
                b.board[idx] = BoardState::Void;
            }

            for (x, c) in line.chars().enumerate() {
                if c != ' ' {
                    let idx = b.to_idx(x, y);
                    b.board[idx] = BoardState::Empty;
                }
            }
            y=y+1;
        }
        Ok(b)
    }

    fn to_idx (&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn erase(&mut self, x: usize, y:usize) {
        self.set(x, y, BoardState::Void);
    }

    fn set(&mut self, x: usize, y: usize, state: BoardState<'a>) {
        let idx = self.to_idx(x, y);
        self.board[idx] = state;
    }

    pub fn get(&self, x: usize, y: usize) -> BoardState<'a> {
        if self.on_board(x, y) {
            return self.board[self.to_idx(x, y)];
        }
        
        BoardState::Void
    }

    pub fn add_polyomino(&mut self, p: &'a Polyomino, ll: Point) -> bool {
        if p.iter().any(|&pt| self.get(pt.x + ll.x, pt.y + ll.y) != BoardState::Empty) {
            return false;
        }

        for pt in p.iter() {
            self.set(pt.x + ll.x, pt.y + ll.y, BoardState::Full(p, ll.x, ll.y));
        }
        
        true
    }

    pub fn remove_polyomino(&mut self, ll: Point) {
        if let BoardState::Full(p, start_x, start_y) = self.get(ll.x, ll.y) {
            for pt in p.iter() {
                self.set(pt.x + start_x, pt.y + start_y, BoardState::Empty);
            }
        }
    }
    
    fn on_board(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }
    
    pub fn row_range(&self) -> Range<usize> {
       0 .. self.height
    }

    pub fn col_range(&self) -> Range<usize> {
       0 .. self.width
    }
}

pub mod board_utils {
    extern crate bit_vec;
    use bit_vec::BitVec;

    use poly::board::Board;
    use poly::board::BoardState;
    use poly::point::Point;
    use poly::polyomino::Polyomino;
    
    use std::collections::HashSet;
    use std::collections::VecDeque;
    
    pub fn get_first_unoccupied(b: &Board) -> Option<Point> {
        for c in b.col_range() {
            for r in b.row_range() {
                if b.get(c, r) == BoardState::Empty {
                    return Some(Point::new(c, r));
                }
            }
        }

        None
    }

    fn get_adjacent(p: Point, b: &Board) -> HashSet<Point> {
        let mut adj = HashSet::new();

        // UP
        if p.y != 0 && b.get(p.x, p.y-1) == BoardState::Empty {
            adj.insert(Point::new(p.x, p.y-1));
        }
        
        // LEFT
        if p.x != 0 && b.get(p.x-1, p.y) == BoardState::Empty {
            adj.insert(Point::new(p.x-1, p.y));
        }

        // DOWN
        if b.get(p.x, p.y+1) == BoardState::Empty {
            adj.insert(Point::new(p.x,p.y+1));
        }
        
        // RIGHT
        if b.get(p.x+1, p.y) == BoardState::Empty {
            adj.insert(Point::new(p.x+1,p.y));
        }

        adj
    }

    #[allow(dead_code)]
    pub fn get_all_adjacent(p: Point, b: &Board) -> HashSet<Point> {
        let mut region = HashSet::new();
        
        if b.get(p.x, p.y) != BoardState::Empty {
            return region;
        }

        let mut pending : VecDeque<Point> = VecDeque::new();

        pending.push_back(p);

        while !pending.is_empty() {
            // Get the first element. If it's not already in the region,
            // add it and add its adj elements to pending
            if let Some(next_p) = pending.pop_front() {
                if !region.contains(&next_p) {
                    region.insert(next_p);

                    for adjp in get_adjacent(next_p, b) {
                        pending.push_back(adjp);
                    }
                }
            }
        }
        
        region
    }

    pub fn fit<'a>(b: &mut Board<'a>, p: &'a Polyomino) -> Option<Point> {
        /* Attempt to fit the polyomino at the first unoccuped spot on the board.
        
        Consider the board with the first unoccupied spot marked with a '.'
        
        +-+-+-+-+-+
        | | | | | |
        +-+-+-+-+-+
        |.| | | | |
        +-+-+-+-+-+
        |X| | | | |
        +-+-+-+-+-+
        |X|X|X| | |
        +-+-+-+-+-+
        
        And the polyomino
        
         x
        xxx
         x
        
        To fit this on the board we don't put the (0,0) point of the polyomino on A, we try to put the
        first occupied point in the polyomino, (0,1), on A. This gives us the best possible fit.
        
        +-+-+-+-+-+
        | |X| | | |
        +-+-+-+-+-+
        |X|X|X| | |
        +-+-+-+-+-+
        |X|X| | | |
        +-+-+-+-+-+
        |X|X|X| | |
        +-+-+-+-+-+
        
        Find the first point of polyomino (this relies on the Point ordering being sane, which it is) and
        shift the polyomino so that it's at the right position
         */
        
        if let Some(target_pt) = get_first_unoccupied(&b) {
            if let Some(poly_pt) = p.iter().next() {
                if poly_pt.x <= target_pt.x && poly_pt.y <= target_pt.y {
                    if b.add_polyomino(p, Point::new(target_pt.x - poly_pt.x, target_pt.y - poly_pt.y))
                    {
                        return Some(target_pt)
                    }
                }
            }
        }
        
        None
    }

    pub fn fill<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>) -> i32 {
        let usable_candidates = BitVec::from_elem(candidates.len(), true);
        
        fill_board(b, candidates, &usable_candidates)
    }

    fn fill_board<'a>(b : &mut Board<'a>, candidates: &'a Vec<HashSet<Polyomino>>, usable_candidates: &BitVec) -> i32 {
        let mut total = 0;
        
        if usable_candidates.none() {
            // println!("{}", b);
            return 1;
        }
        
        let mut uc = usable_candidates.clone();
        
        let polys_with_index = usable_candidates.iter().
            enumerate().
            zip(candidates.iter()).
            filter_map(|((n, b), polys)|
                       if b {
                           Some((n, polys))
                       } else {
                           None
                       });
        
        for (n, polys) in polys_with_index {
            for poly in polys {
                if let Some(point) = fit(b, &poly) {
                    uc.set(n, false);
                    total += fill_board(b, candidates, &uc);
                    uc.set(n, true);
                    b.remove_polyomino(point);
                }
            }
        }
        
        total
    }
}

#[cfg(test)]
mod tests {
    use poly::board::Board;
    use poly::board::BoardState;
    use poly::board::board_utils;
    use poly::point::Point;
    use poly::polyomino::Polyomino;

    fn build_u() -> Polyomino {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(1, 0));
        p.push(Point::new(0, 1));
        p.push(Point::new(0, 2));
        p.push(Point::new(1, 2));

        Polyomino::new(p)
    }

    fn build_x() -> Polyomino {
        let mut p = Vec::new();
        p.push(Point::new(1, 0));
        p.push(Point::new(1, 1));
        p.push(Point::new(1, 2));
        p.push(Point::new(0, 1));
        p.push(Point::new(2, 1));

        Polyomino::new(p)
    }

    fn build_w() -> Polyomino {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(1, 0));
        p.push(Point::new(1, 1));
        p.push(Point::new(2, 1));
        p.push(Point::new(2, 2));
        
        Polyomino::new(p)
    }
    
    fn build_l() -> Polyomino {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(0, 1));
        p.push(Point::new(0, 2));
        p.push(Point::new(0, 3));
        p.push(Point::new(1, 3));
        
        Polyomino::new(p)
    }

    fn build_i() -> Polyomino {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(1, 0));
        p.push(Point::new(2, 0));
        p.push(Point::new(3, 0));
        p.push(Point::new(4, 0));
        
        Polyomino::new(p)
    }
    
    fn build_y() -> Polyomino {
        let mut p = Vec::new();
        p.push(Point::new(0, 1));
        p.push(Point::new(1, 1));
        p.push(Point::new(2, 1));
        p.push(Point::new(3, 1));
        p.push(Point::new(2, 0));
        
        Polyomino::new(p)
    }

    #[test]
    fn test_add_remove() {
        let w = build_w();
        let l = build_l();

        let mut b = Board::new(12,5);
        b.add_polyomino(&w, Point::new(0, 0));
        b.add_polyomino(&l, Point::new(4, 0));
        assert!(b.get(0, 0) == BoardState::Full(&w, 0, 0));
        assert!(b.get(4, 0) == BoardState::Full(&l, 4, 0));
        assert!(b.get(4, 3) == BoardState::Full(&l, 4, 0));
        b.remove_polyomino(Point::new(1, 1));
        assert!(b.get(0, 0) == BoardState::Empty);
        assert!(b.get(4, 0) == BoardState::Full(&l, 4, 0));
        b.remove_polyomino(Point::new(4, 2));
        assert!(b.get(4, 0) == BoardState::Empty);
    }
    
    #[test]
    fn test_point_ordering_for_fit() {
        // Fit does require a particular point ordering, so let's test that here.
        // We also test it in Point
        let p00 = Point::new(0,0);
        let p07 = Point::new(0,7);
        let p10 = Point::new(1,0);

        assert!(p00 < p07);
        assert!(p10 > p07);
    }

    #[test]
    fn test_fit() {
        let u = build_u();
        let x = build_x();
        let l = build_l();
        let l_rot = l.rotate();
        let y = build_y();
        let i = build_i();
    
        let mut b = Board::new(20, 3);
        assert_eq!(board_utils::get_first_unoccupied(&b), Some(Point::new(0,0)));
        assert_eq!(board_utils::fit(&mut b, &y), None);
        assert_eq!(board_utils::fit(&mut b, &u), Some(Point::new(0,0)));
        assert_eq!(board_utils::fit(&mut b, &l_rot), None);
        assert_eq!(board_utils::get_first_unoccupied(&b), Some(Point::new(1,1)));
        assert_eq!(board_utils::fit(&mut b, &x), Some(Point::new(1,1)));
        assert_eq!(board_utils::get_first_unoccupied(&b), Some(Point::new(3,0)));
        assert_eq!(board_utils::fit(&mut b, &i), Some(Point::new(3,0)));
    }

    #[test]
    fn test_read() {
        if let Ok(b) = Board::from_file("data/b8x8holes.txt") {
            assert_eq!(b.get(0, 0), BoardState::Empty);
            assert_eq!(b.get(0, 7), BoardState::Empty);
            assert_eq!(b.get(7, 0), BoardState::Empty);
            assert_eq!(b.get(7, 7), BoardState::Empty);
            assert_eq!(b.get(2, 2), BoardState::Void);
            assert_eq!(b.get(5, 2), BoardState::Void);
            assert_eq!(b.get(2, 5), BoardState::Void);
            assert_eq!(b.get(5, 5), BoardState::Void);
        } else {
            assert!(false, "Unable to read data/b8x8holes.txt");
        }
        
    }
}
