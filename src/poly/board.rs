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

#[allow(dead_code)]
pub struct Board<'a> {
    height: usize,
    width: usize,
    board: Vec<BoardState<'a>>
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.print_top_row_border(f);

        for y in 0 .. self.height {
            self.print_row(f, y as i32);
        }

        write!(f, "")
    }

}

#[allow(dead_code)]
impl<'a> Board<'a> {
    pub fn new(h: usize, w: usize) -> Board<'a> {
        Board { height: h,
                width: w,
                board: vec![BoardState::Empty; h*w as usize] }
    }

    fn set(&mut self, x: i32, y: i32, polyomino: &'a Polyomino) -> bool {
        if x < 0 || y < 0 {
            return false
        }
        
        let xu = x as usize;
        let yu = y as usize;

        let idx = xu + yu * self.width;
        
        if xu < self.width && yu < self.height && self.board[idx] == BoardState::Empty {
            self.board[idx] = BoardState::Full(polyomino);
            return true
        }

        false
    }

    pub fn get(&self, x: i32, y: i32) -> BoardState<'a> {
        let xu = x as usize;
        let yu = y as usize;
        
        let idx = xu + yu * self.width;
        
        if x >= 0 && xu < self.width && y >= 0 && yu < self.height {
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
    
    fn print_top_row_border(&self, f: &mut fmt::Formatter) {
        let _ = write!(f, "+");

        for x in 0 .. self.width {
            let piece = self.get(x as i32, 0);
            
            let _ = write!(f, "{}", if piece == BoardState::Void {
                if self.get((x+1) as i32, 0) == BoardState::Void {
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
    
    fn print_row(&self, f: &mut fmt::Formatter, y: i32) {
        for x in 0 .. self.width {
            let piece = self.get(x as i32, y);
            
            if x == 0 {
                let _ = write!(f, "{}", if piece == BoardState::Void {
                    " "
                } else {
                    "|"
                });
            }

            let _ = write!(f, "{}", rep(&piece));

            let _ = write!(f, "{}", if piece == self.get((x+1) as i32, y) {
                " "
            } else {
                "|"
            });
        }

        let _ = writeln!(f, "");

        self.print_row_bottom_border(f, y);
    }
    
    fn print_row_bottom_border(&self, f: &mut fmt::Formatter, y: i32) {
        let _ = write!(f, "+");

        for x in 0 .. self.width {
            let piece = self.get(x as i32, y);
            
            let _ = write!(f, "{}", if piece == self.get(x as i32, y+1) {
                if piece == self.get((x as i32)+1, y) && self.get(x as i32, y+1) == self.get((x as i32)+1, y+1) {
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
}
