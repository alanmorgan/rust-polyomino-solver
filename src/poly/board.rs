use std::ops::Range;
use std::fmt;

use poly::point::Point;
use poly::polyomino::Polyomino;

#[derive(Clone, Copy, PartialEq)]
pub enum BoardState<'a> {
     Void,      // Out of bounds/a hole in the board
     Empty,     // A valid part of the board, but no piece is there
     Full(& 'a Polyomino)  // Has a piece
}

fn rep(b : &BoardState) -> char {
    match *b {
        BoardState::Void => ' ',
        BoardState::Empty => '.',
        BoardState::Full(_p) => 'X'
    }
}

pub struct Board<'a> {
    height: usize,
    width: usize,
    board: Vec<BoardState<'a>>
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        fn print_top_row_border(s: &Board, f: &mut fmt::Formatter) {
            let _ = write!(f, "+");

            for x in 0 .. s.width {
                let piece = s.get(x, 0);
                
                let _ = write!(f, "{}", if piece == BoardState::Void {
                    if s.get(x+1, 0) == BoardState::Void {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                });
            }

            let _ = writeln!(f, "");
        }
        
        fn print_row(s: &Board, f: &mut fmt::Formatter, y: usize) {
            for x in 0 .. s.width {
                let piece = s.get(x, y);
                
                if x == 0 {
                    let _ = write!(f, "{}", if piece == BoardState::Void {
                        " "
                    } else {
                        "|"
                    });
                }

                let _ = write!(f, "{}", rep(&piece));

                let _ = write!(f, "{}", if piece == s.get(x+1, y) {
                    " "
                } else {
                    "|"
                });
            }

            let _ = writeln!(f, "");

            print_row_bottom_border(s, f, y);
        }
        
        fn print_row_bottom_border(s: &Board, f: &mut fmt::Formatter, y: usize) {
            let _ = write!(f, "+");

            for x in 0 .. s.width {
                let piece = s.get(x, y);
                
                let _ = write!(f, "{}", if piece == s.get(x, y+1) {
                    if piece == s.get(x+1, y) && s.get(x, y+1) == s.get(x+1, y+1) {
                        "  "
                    } else {
                        " +"
                    }
                } else {
                    "-+"
                });
            }

            let _ = writeln!(f, "");
        }

        
        print_top_row_border(self, f);

        for y in 0 .. self.height {
            print_row(self, f, y);
        }

        write!(f, "")
    }

}

#[allow(dead_code)]
impl<'a> Board<'a> {
    pub fn new(h: usize, w: usize) -> Board<'a> {
        Board { height: h,
                width: w,
                board: vec![BoardState::Empty; h*w] }
    }

    fn set(&mut self, x: usize, y: usize, polyomino: &'a Polyomino) -> bool {
        let idx = x + y * self.width;
        
        if x < self.width && y < self.height && self.board[idx] == BoardState::Empty {
            self.board[idx] = BoardState::Full(polyomino);
            return true
        }

        false
    }

    pub fn get(&self, x: usize, y: usize) -> BoardState<'a> {
        let idx = x + y * self.width;
        
        if x < self.width && y < self.height {
            return self.board[idx]
        }
        
        BoardState::Void
    }

    pub fn add_polyomino(&mut self, p: &'a Polyomino, ll: Point) -> bool {
        if p.iter().any(|&pt| self.get(pt.x + ll.x, pt.y + ll.y) != BoardState::Empty) {
            return false;
        }

        for pt in p.iter() {
           self.set(pt.x + ll.x, pt.y + ll.y, p); 
        }
        
        true
    }

    pub fn row_range(&self) -> Range<usize> {
       0 .. self.height
    }

    pub fn col_range(&self) -> Range<usize> {
       0 .. self.width
    }
}

mod board_utils {
    use poly::board::Board;
    use poly::board::BoardState;
    use poly::point::Point;
    
    pub fn get_first_unoccupied(b: &Board) -> Option<Point> {
        for r in b.row_range() {
            for c in b.col_range() {
                if b.get(r, c) == BoardState::Empty {
                    return Some(Point::new(r, c));
                }
            }
        }

        None
    }
}
