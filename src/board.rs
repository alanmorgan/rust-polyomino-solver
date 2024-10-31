use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::ops::Range;

use crate::point::Point;
use crate::point::SimplePoint;
use crate::polyomino::Polyomino;

#[derive(Clone, PartialEq, Debug)]
pub enum BoardState<'a, P: Polyomino> {
    Void,  // Out of bounds/a hole in the board
    Empty, // A valid part of the board, but no piece is there
    Full(&'a P, usize, i16, i16), // Has a piece
}

impl <'a, P> Copy for BoardState<'a, P> where P: Polyomino {}

impl<'a, P:Polyomino> fmt::Display for BoardState<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.rep())
    }
}

impl<'a, P:Polyomino> BoardState<'a, P> {
    pub fn rep(&self) -> String {
        match *self {
            BoardState::Void => " ".to_string(),
            BoardState::Empty => ".".to_string(),
            BoardState::Full(poly, pt_idx, _, _) => poly.get_nth(pt_idx).unwrap().to_string(),
        }
    }

    pub fn connected_to(&self, other : BoardState<'a, P>) -> bool {
        match *self {
            BoardState::Void => other == BoardState::Void,
            BoardState::Empty => other == BoardState::Empty,
            BoardState::Full(p, _, _, _) => if let BoardState::Full(p1, _, _, _) = other { p == p1 } else { false }
        }
    }
}

pub struct Board<'a, P:Polyomino> {
    height: i16,
    width: i16,
    board: Vec<BoardState<'a, P>>,
}

impl<'a, P:Polyomino> fmt::Display for Board<'a, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn print_top_row_border<P: Polyomino>(s: &Board<P>, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str(if s.get(0, 0) == BoardState::Void {
                " "
            } else {
                "+"
            })?;

            for x in 0..s.width {
                let piece = s.get(x, 0);

                f.write_str(if piece == BoardState::Void {
                    if s.get(x + 1, 0) == BoardState::Void {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                })?;
            }

            f.write_str("\n")
        }

        fn print_row<P:Polyomino>(s: &Board<P>, f: &mut fmt::Formatter, y: i16) -> fmt::Result {
            for x in 0..s.width {
                let piece = s.get(x, y);

                if x == 0 {
                    f.write_str(if piece == BoardState::Void { " " } else { "|" })?;
                }

                f.write_str(&piece.rep())?;

                // Testing equivalence should ignore the particular pt
                f.write_str(if piece.connected_to(s.get(x + 1, y)) { " " } else { "|" })?;
            }

            f.write_str("\n")?;

            print_row_bottom_border(s, f, y)
        }

        fn print_row_bottom_border<P:Polyomino>(s: &Board<P>, f: &mut fmt::Formatter, y: i16) -> fmt::Result {
            f.write_str(if s.get(0, y) == BoardState::Void {
                " "
            } else {
                "+"
            })?;

            for x in 0..s.width {
                let piece = s.get(x, y);

                f.write_str(if piece.connected_to(s.get(x, y + 1)) {
                    if piece.connected_to(s.get(x + 1, y)) && s.get(x, y + 1).connected_to(s.get(x + 1, y + 1)) {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                })?;
            }

            f.write_str("\n")
        }

        print_top_row_border(self, f)?;

        for y in 0..self.height {
            print_row(self, f, y)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl<'a, P:Polyomino> Board<'a, P> {
    pub fn new(w: i16, h: i16) -> Board<'a, P> {
        Board {
            height: h,
            width: w,
            board: vec![BoardState::Empty; (h * w) as usize],
        }
    }

    pub fn from_file(name: &str) -> Result<Board<P>, Error> {
        let f = File::open(name)?;

        let buf_file = BufReader::new(&f);

        let mut board_x: i16 = 0;
        let mut board_y: i16 = 0;
        let mut lines = Vec::new();

        for wline in buf_file.lines() {
            let line = wline.unwrap();
            board_x = cmp::max(board_x, line.len() as i16);
            lines.push(line);
            board_y += 1;
        }

        let mut b = Board::new(board_x, board_y);

        for (y, line) in lines.into_iter().enumerate() {
            // We could treat each space as a void, but trailing
            // spaces may not exist. Instead, mark each space as
            // void and then mark spaces as empty

            for x in 0..board_x {
                let idx = b.to_idx(x, y as i16);
                b.board[idx] = BoardState::Void;
            }

            for (x, c) in line.chars().enumerate() {
                if c != ' ' {
                    let idx = b.to_idx(x as i16, y as i16);
                    b.board[idx] = BoardState::Empty;
                }
            }
        }
        Ok(b)
    }

    fn to_idx(&self, x: i16, y: i16) -> usize {
        (x * self.height + y) as usize
    }

    pub fn erase(&mut self, x: i16, y: i16) {
        self.set(x, y, BoardState::Void);
    }

    fn set(&mut self, x: i16, y: i16, state: BoardState<'a, P>) {
        let idx = self.to_idx(x, y);
        self.board[idx] = state;
    }

    pub fn get(&self, x: i16, y: i16) -> BoardState<'a, P> {
        if self.on_board(x, y) {
            return self.board[self.to_idx(x, y)];
        }

        BoardState::Void
    }

    pub fn add_polyomino<'b>(&mut self, p: &'a P, ll: &'b SimplePoint) -> bool {
        if p.iter().any(|&pt| self.get(pt.x() + ll.x(), pt.y() + ll.y()) != BoardState::Empty)
        {
            return false;
        }

        for (idx, pt) in p.iter().enumerate() {
            self.set(pt.x() + ll.x(), pt.y() + ll.y(), BoardState::Full(p, idx, ll.x(), ll.y()));
        }

        true
    }

    pub fn remove_polyomino(&mut self, ll: &SimplePoint) {
        if let BoardState::Full(p, _, start_x, start_y) = self.get(ll.x(), ll.y()) {
            for pt in p.iter() {
                self.set(pt.x() + start_x, pt.y() + start_y, BoardState::Empty);
            }
        }
    }

    fn on_board(&self, x: i16, y: i16) -> bool {
        x < self.width && y < self.height
    }

    pub fn get_height(&self) -> i16 {
        self.height
    }

    pub fn get_width(&self) -> i16 {
        self.width
    }
    
    pub fn row_range(&self) -> Range<i16> {
        0..self.height
    }

    pub fn col_range(&self) -> Range<i16> {
        0..self.width
    }
}

pub mod board_utils {
    use std::collections::VecDeque;
    
    use rustc_hash::FxHashSet;

    use crate::board::Board;
    use crate::board::BoardState;
    use crate::point::SimplePoint;
    use crate::point::Point;
    use crate::polyomino::Polyomino;

    pub fn get_first_unoccupied<P:Polyomino>(b: &Board<P>) -> Option<SimplePoint> {
        for i in 0..b.board.len() {
            if b.board[i] == BoardState::Empty {
                return Some(SimplePoint::new(
                    i as i16 / b.height,
                    i as i16 % b.height,
                ));
            }
        }

        None
    }

    pub fn get_adjacent<P:Polyomino>(p: SimplePoint, b: &Board<P>) -> FxHashSet<SimplePoint> {
        let mut adj = FxHashSet::default();

        // UP
        if p.y != 0 && b.get(p.x, p.y - 1) == BoardState::Empty {
            adj.insert(SimplePoint::new(p.x, p.y - 1));
        }

        // LEFT
        if p.x != 0 && b.get(p.x - 1, p.y) == BoardState::Empty {
            adj.insert(SimplePoint::new(p.x - 1, p.y));
        }

        // DOWN
        if p.y != b.height - 1 && b.get(p.x, p.y + 1) == BoardState::Empty {
            adj.insert(SimplePoint::new(p.x, p.y + 1));
        }

        // RIGHT
        if p.x != b.width - 1 && b.get(p.x + 1, p.y) == BoardState::Empty {
            adj.insert(SimplePoint::new(p.x + 1, p.y));
        }

        adj
    }

    #[allow(dead_code)]
    pub fn get_all_adjacent<P:Polyomino>(p: SimplePoint, b: &Board<P>) -> FxHashSet<SimplePoint> {
        let mut region = FxHashSet::default();

        if b.get(p.x, p.y) != BoardState::Empty {
            return region;
        }

        let mut pending: VecDeque<SimplePoint> = VecDeque::new();

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

    #[allow(dead_code)]
    pub fn fit<'a, P:Polyomino>(b: &mut Board<'a, P>, p: &'a P) -> Option<SimplePoint> {
        /* Attempt to fit the polyomino at the first unoccuped spot on the board. */

        if let Some(target_pt) = get_first_unoccupied(b) {
            if fit_at(b, p, &target_pt) {
                return Some(target_pt);
            }
        }

        None
    }

    pub fn fit_at<'a, P:Polyomino>(b: &mut Board<'a, P>, p: &'a P, target_pt: &SimplePoint) -> bool {
        /* Attempt to fit the polyomino at the specified spot on the board.

        * This is not quite putting the polyomino's 0,0 point at the target_pt, because that point
        * might not exist in the polyomino (think the `+` pentomino). Instead, take the first point
        * in the pentomino (which is guaranteed to be the smallest point of the form (0, y) because
        * the points are sorted) and put *that* at target_pt
        */
        if let Some(poly_pt) = p.iter().next() {
            if poly_pt.x() <= target_pt.x()
                && poly_pt.y() <= target_pt.y()
                && b.add_polyomino(
                    p,
                    &SimplePoint::new(target_pt.x() - poly_pt.x(), target_pt.y() - poly_pt.y()),
                )
            {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::board::board_utils;
    use crate::board::Board;
    use crate::board::BoardState;
    use crate::point::Point;
    use crate::point::SimplePoint;
    use crate::polyomino::Polyomino;
    use crate::polyomino::SimplePolyomino;

    fn build_u() -> SimplePolyomino<SimplePoint> {
        let mut p = Vec::new();
        p.push(SimplePoint::new(0, 0));
        p.push(SimplePoint::new(1, 0));
        p.push(SimplePoint::new(0, 1));
        p.push(SimplePoint::new(0, 2));
        p.push(SimplePoint::new(1, 2));

        SimplePolyomino::new(p)
    }

    fn build_x() -> SimplePolyomino<SimplePoint> {
        let mut p = Vec::new();
        p.push(SimplePoint::new(1, 0));
        p.push(SimplePoint::new(1, 1));
        p.push(SimplePoint::new(1, 2));
        p.push(SimplePoint::new(0, 1));
        p.push(SimplePoint::new(2, 1));

        SimplePolyomino::new(p)
    }

    fn build_w() -> SimplePolyomino<SimplePoint> {
        let mut p = Vec::new();
        p.push(SimplePoint::new(0, 0));
        p.push(SimplePoint::new(1, 0));
        p.push(SimplePoint::new(1, 1));
        p.push(SimplePoint::new(2, 1));
        p.push(SimplePoint::new(2, 2));

        SimplePolyomino::new(p)
    }

    fn build_l() -> SimplePolyomino<SimplePoint> {
        let mut p = Vec::new();
        p.push(SimplePoint::new(0, 0));
        p.push(SimplePoint::new(0, 1));
        p.push(SimplePoint::new(0, 2));
        p.push(SimplePoint::new(0, 3));
        p.push(SimplePoint::new(1, 3));

        SimplePolyomino::new(p)
    }

    fn build_i() -> SimplePolyomino<SimplePoint> {
        let mut p = Vec::new();
        p.push(SimplePoint::new(0, 0));
        p.push(SimplePoint::new(1, 0));
        p.push(SimplePoint::new(2, 0));
        p.push(SimplePoint::new(3, 0));
        p.push(SimplePoint::new(4, 0));

        SimplePolyomino::new(p)
    }

    fn build_y() -> SimplePolyomino<SimplePoint> {
        let mut p = Vec::new();
        p.push(SimplePoint::new(0, 1));
        p.push(SimplePoint::new(1, 1));
        p.push(SimplePoint::new(2, 1));
        p.push(SimplePoint::new(3, 1));
        p.push(SimplePoint::new(2, 0));

        SimplePolyomino::new(p)
    }

    #[test]
    fn test_add_remove() {
        let w = build_w();
        let l = build_l();

        let mut b = Board::new(12, 5);
        b.add_polyomino(&w, &SimplePoint::new(0, 0));
        b.add_polyomino(&l, &SimplePoint::new(4, 0));
        assert!(b.get(0, 0) == BoardState::Full(&w, 0, 0, 0));
        assert!(b.get(4, 0) == BoardState::Full(&l, 0, 4, 0));
        assert!(b.get(4, 3) == BoardState::Full(&l, 3, 4, 0));
        b.remove_polyomino(&SimplePoint::new(1, 1));
        assert!(b.get(0, 0) == BoardState::Empty);
        assert!(b.get(4, 0) == BoardState::Full(&l, 0, 4, 0));
        b.remove_polyomino(&SimplePoint::new(4, 2));
        assert!(b.get(4, 0) == BoardState::Empty);
    }

    #[test]
    fn test_point_ordering_for_fit() {
        // Fit does require a particular point ordering, so let's test that here.
        // We also test it in SimplePoint
        let p00 = SimplePoint::new(0, 0);
        let p07 = SimplePoint::new(0, 7);
        let p10 = SimplePoint::new(1, 0);

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
        assert_eq!(
            board_utils::get_first_unoccupied(&b),
            Some(SimplePoint::new(0, 0))
        );
        assert_eq!(board_utils::fit(&mut b, &y), None);
        assert_eq!(board_utils::fit(&mut b, &u), Some(SimplePoint::new(0, 0)));
        assert_eq!(board_utils::fit(&mut b, &l_rot), None);
        assert_eq!(
            board_utils::get_first_unoccupied(&b),
            Some(SimplePoint::new(1, 1))
        );
        assert_eq!(board_utils::fit(&mut b, &x), Some(SimplePoint::new(1, 1)));
        assert_eq!(
            board_utils::get_first_unoccupied(&b),
            Some(SimplePoint::new(3, 0))
        );
        assert_eq!(board_utils::fit(&mut b, &i), Some(SimplePoint::new(3, 0)));
    }

    #[test]
    fn test_read() {
        if let Ok(b) = Board::<SimplePolyomino<SimplePoint>>::from_file("data/b8x8holes.board") {
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

    #[test]
    fn test_get_adjacent() {
        // X.X
        // ...
        // X.X
        // X.X
        let mut b = Board::<SimplePolyomino<SimplePoint>>::new(3, 4);

        b.set(0, 0, BoardState::Void);
        b.set(2, 0, BoardState::Void);
        b.set(0, 2, BoardState::Void);
        b.set(2, 2, BoardState::Void);
        b.set(0, 3, BoardState::Void);
        b.set(2, 3, BoardState::Void);

        assert_eq!(board_utils::get_adjacent(SimplePoint::new(1, 1), &b).len(), 4);

        assert_eq!(board_utils::get_adjacent(SimplePoint::new(1, 3), &b).len(), 1);

        assert_eq!(board_utils::get_adjacent(SimplePoint::new(2, 1), &b).len(), 1);
    }
}


















