use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::ops::Range;

use crate::point::Point;
use crate::point::Pt;
use crate::polyomino::Polyomino;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BoardState<'a, T:Pt> {
    Void,  // Out of bounds/a hole in the board
    Empty, // A valid part of the board, but no piece is there
    Full(&'a Polyomino<T>, &'a T, i16, i16), // Has a piece
}

impl<'a, T:Pt> fmt::Display for BoardState<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.rep())
    }
}

impl<'a, T:Pt> BoardState<'a, T> {
    pub fn rep(&self) -> String {
        match *self {
            BoardState::Void => " ".to_string(),
            BoardState::Empty => ".".to_string(),
            BoardState::Full(_p, pt, _x, _y) => pt.to_string(),
        }
    }

    pub fn connected_to(&self, other : BoardState<'a, T>) -> bool {
        match *self {
            BoardState::Void => other == BoardState::Void,
            BoardState::Empty => other == BoardState::Empty,
            BoardState::Full(p, _, _, _) => if let BoardState::Full(p1, _, _, _) = other { p == p1 } else { false }
        }
    }
}

pub struct Board<'a, T:Pt> {
    height: i16,
    width: i16,
    board: Vec<BoardState<'a, T>>,
}

impl<'a, T:Pt> fmt::Display for Board<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn print_top_row_border<T:Pt>(s: &Board<T>, f: &mut fmt::Formatter) -> fmt::Result {
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

        fn print_row<T:Pt>(s: &Board<T>, f: &mut fmt::Formatter, y: i16) -> fmt::Result {
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

        fn print_row_bottom_border<T:Pt>(s: &Board<T>, f: &mut fmt::Formatter, y: i16) -> fmt::Result {
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

        print_top_row_border::<T>(self, f)?;

        for y in 0..self.height {
            print_row::<T>(self, f, y)?;
        }

        Ok(())
    }
}

#[allow(dead_code)]
impl<'a, T:Pt> Board<'a, T> {
    pub fn new(w: i16, h: i16) -> Board<'a, T> {
        Board {
            height: h,
            width: w,
            board: vec![BoardState::Empty; (h * w) as usize],
        }
    }

    pub fn from_file(name: &str) -> Result<Board<T>, Error> {
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

    fn set(&mut self, x: i16, y: i16, state: BoardState<'a, T>) {
        let idx = self.to_idx(x, y);
        self.board[idx] = state;
    }

    pub fn get(&self, x: i16, y: i16) -> BoardState<'a, T> {
        if self.on_board(x, y) {
            return self.board[self.to_idx(x, y)];
        }

        BoardState::Void
    }

    pub fn add_polyomino<'b>(&mut self, p: &'a Polyomino<T>, ll: &'b Point) -> bool {
        if p.iter().any(|&pt| self.get(pt.x() + ll.x(), pt.y() + ll.y()) != BoardState::Empty)
        {
            return false;
        }

        for pt in p.iter() {
            self.set(pt.x() + ll.x(), pt.y() + ll.y(), BoardState::Full(p, pt, ll.x(), ll.y()));
        }

        true
    }

    pub fn remove_polyomino(&mut self, ll: &Point) {
        if let BoardState::Full(p, _, start_x, start_y) = self.get(ll.x(), ll.y()) {
            for pt in p.iter() {
                self.set(pt.x() + start_x, pt.y() + start_y, BoardState::Empty);
            }
        }
    }

    fn on_board(&self, x: i16, y: i16) -> bool {
        x < self.width && y < self.height
    }

    pub fn row_range(&self) -> Range<i16> {
        0..self.height
    }

    pub fn col_range(&self) -> Range<i16> {
        0..self.width
    }
}

pub mod board_utils {
    use crate::board::Board;
    use crate::board::BoardState;
    use crate::point::Point;
    use crate::point::Pt;
    use crate::polyomino::Polyomino;

    use rustc_hash::FxHashSet;
    use std::collections::VecDeque;

    pub fn get_first_unoccupied<T:Pt>(b: &Board<T>) -> Option<Point> {
        for i in 0..b.board.len() {
            if b.board[i] == BoardState::Empty {
                return Some(Point::new(
                    i as i16 / b.height,
                    i as i16 % b.height,
                ));
            }
        }

        None
    }

    pub fn get_adjacent<T:Pt>(p: Point, b: &Board<T>) -> FxHashSet<Point> {
        let mut adj = FxHashSet::default();

        // UP
        if p.y != 0 && b.get(p.x, p.y - 1) == BoardState::Empty {
            adj.insert(Point::new(p.x, p.y - 1));
        }

        // LEFT
        if p.x != 0 && b.get(p.x - 1, p.y) == BoardState::Empty {
            adj.insert(Point::new(p.x - 1, p.y));
        }

        // DOWN
        if p.y != b.height - 1 && b.get(p.x, p.y + 1) == BoardState::Empty {
            adj.insert(Point::new(p.x, p.y + 1));
        }

        // RIGHT
        if p.x != b.width - 1 && b.get(p.x + 1, p.y) == BoardState::Empty {
            adj.insert(Point::new(p.x + 1, p.y));
        }

        adj
    }

    #[allow(dead_code)]
    pub fn get_all_adjacent<T:Pt>(p: Point, b: &Board<T>) -> FxHashSet<Point> {
        let mut region = FxHashSet::default();

        if b.get(p.x, p.y) != BoardState::Empty {
            return region;
        }

        let mut pending: VecDeque<Point> = VecDeque::new();

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
    pub fn fit<'a, T:Pt>(b: &mut Board<'a, T>, p: &'a Polyomino<T>) -> Option<Point> {
        /* Attempt to fit the polyomino at the first unoccuped spot on the board. */

        if let Some(target_pt) = get_first_unoccupied(b) {
            if fit_at(b, p, &target_pt) {
                return Some(target_pt);
            }
        }

        None
    }

    pub fn fit_at<'a, T:Pt>(b: &mut Board<'a, T>, p: &'a Polyomino<T>, target_pt: &Point) -> bool {
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
                    &Point::new(target_pt.x() - poly_pt.x(), target_pt.y() - poly_pt.y()),
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
    use board::board_utils;
    use board::Board;
    use board::BoardState;
    use point::Point;
    use polyomino::Polyomino;

    fn build_u() -> Polyomino<Point> {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(1, 0));
        p.push(Point::new(0, 1));
        p.push(Point::new(0, 2));
        p.push(Point::new(1, 2));

        Polyomino::new(p)
    }

    fn build_x() -> Polyomino<Point> {
        let mut p = Vec::new();
        p.push(Point::new(1, 0));
        p.push(Point::new(1, 1));
        p.push(Point::new(1, 2));
        p.push(Point::new(0, 1));
        p.push(Point::new(2, 1));

        Polyomino::new(p)
    }

    fn build_w() -> Polyomino<Point> {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(1, 0));
        p.push(Point::new(1, 1));
        p.push(Point::new(2, 1));
        p.push(Point::new(2, 2));

        Polyomino::new(p)
    }

    fn build_l() -> Polyomino<Point> {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(0, 1));
        p.push(Point::new(0, 2));
        p.push(Point::new(0, 3));
        p.push(Point::new(1, 3));

        Polyomino::new(p)
    }

    fn build_i() -> Polyomino<Point> {
        let mut p = Vec::new();
        p.push(Point::new(0, 0));
        p.push(Point::new(1, 0));
        p.push(Point::new(2, 0));
        p.push(Point::new(3, 0));
        p.push(Point::new(4, 0));

        Polyomino::new(p)
    }

    fn build_y() -> Polyomino<Point> {
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

        let mut b = Board::new(12, 5);
        b.add_polyomino(&w, &Point::new(0, 0));
        b.add_polyomino(&l, &Point::new(4, 0));
        assert!(b.get(0, 0) == BoardState::Full(&w, w.get_nth(0).unwrap(), 0, 0));
        assert!(b.get(4, 0) == BoardState::Full(&l, l.get_nth(0).unwrap(), 4, 0));
        assert!(b.get(4, 3) == BoardState::Full(&l, l.get_nth(3).unwrap(), 4, 0));
        b.remove_polyomino(&Point::new(1, 1));
        assert!(b.get(0, 0) == BoardState::Empty);
        assert!(b.get(4, 0) == BoardState::Full(&l, l.get_nth(0).unwrap(), 4, 0));
        b.remove_polyomino(&Point::new(4, 2));
        assert!(b.get(4, 0) == BoardState::Empty);
    }

    #[test]
    fn test_point_ordering_for_fit() {
        // Fit does require a particular point ordering, so let's test that here.
        // We also test it in Point
        let p00 = Point::new(0, 0);
        let p07 = Point::new(0, 7);
        let p10 = Point::new(1, 0);

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
            Some(Point::new(0, 0))
        );
        assert_eq!(board_utils::fit(&mut b, &y), None);
        assert_eq!(board_utils::fit(&mut b, &u), Some(Point::new(0, 0)));
        assert_eq!(board_utils::fit(&mut b, &l_rot), None);
        assert_eq!(
            board_utils::get_first_unoccupied(&b),
            Some(Point::new(1, 1))
        );
        assert_eq!(board_utils::fit(&mut b, &x), Some(Point::new(1, 1)));
        assert_eq!(
            board_utils::get_first_unoccupied(&b),
            Some(Point::new(3, 0))
        );
        assert_eq!(board_utils::fit(&mut b, &i), Some(Point::new(3, 0)));
    }

    #[test]
    fn test_read() {
        if let Ok(b) = Board::<Point>::from_file("data/b8x8holes.board") {
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
        let mut b = Board::<'_, Point>::new(3, 4);

        b.set(0, 0, BoardState::Void);
        b.set(2, 0, BoardState::Void);
        b.set(0, 2, BoardState::Void);
        b.set(2, 2, BoardState::Void);
        b.set(0, 3, BoardState::Void);
        b.set(2, 3, BoardState::Void);

        assert_eq!(board_utils::get_adjacent(Point::new(1, 1), &b).len(), 4);

        assert_eq!(board_utils::get_adjacent(Point::new(1, 3), &b).len(), 1);

        assert_eq!(board_utils::get_adjacent(Point::new(2, 1), &b).len(), 1);
    }
}
