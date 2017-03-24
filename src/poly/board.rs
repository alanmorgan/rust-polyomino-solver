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

        fn print_top_row_border(s: &Board, f: &mut fmt::Formatter) -> fmt::Result {
            try!(f.write_str("+"));

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

                try!(f.write_str(&rep(&piece).to_string()));

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
            try!(f.write_str("+"));

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

        f.write_str("")
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
        
        if self.on_board(x, y) && self.board[idx] == BoardState::Empty {
            self.board[idx] = BoardState::Full(polyomino);
            return true
        }

        false
    }

    pub fn get(&self, x: usize, y: usize) -> BoardState<'a> {
        if self.on_board(x, y) {
            return self.board[x + y * self.width];
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

    pub fn on_board(&self, x: usize, y: usize) -> bool {
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
    use poly::board::Board;
    use poly::board::BoardState;
    use poly::point::Point;
    
    use std::collections::HashSet;
    use std::collections::VecDeque;
    
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
}
