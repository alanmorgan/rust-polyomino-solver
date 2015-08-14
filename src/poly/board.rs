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
    board: Vec<Vec<BoardState<'a>>>
}

#[allow(dead_code)]
impl<'a> Board<'a> {
    pub fn new(h: usize, w: usize) -> Board<'a> {
        let mut board = Vec::new();

        for _ in 0 .. h {
            board.push(vec![BoardState::Empty; w as usize]);
        }

        Board { height: h,
                width: w,
                board: board }
    }

    pub fn print(&self) {
        for row in self.board.iter() {
            for piece in row.iter() {
                print!("{}", rep(piece));
            }
            println!("");
        }
    }

    pub fn print_row(&self, y: i32) {
        let ref row = self.board[y as usize];

        for (x, &piece) in row.iter().enumerate() {
            if x == 0 {
                print!("{}", if piece == BoardState::Void {
                    " "
                } else {
                    "|"
                });
            }

            print!("{}", if let BoardState::Full(_p) = piece {
                "X"
            } else {
                " "
            });

            print!("{}", if piece == self.get((x+1) as i32, y) {
                " "
            } else {
                "|"
            });
        }

        println!("");
    }
    
    fn set(&mut self, x: i32, y: i32, polyomino: &'a Polyomino) -> bool {
        if x < 0 || y < 0 {
            return false
        }
        
        let xu = x as usize;
        let yu = y as usize;
        
        if xu < self.width && yu < self.height && self.board[yu][xu] == BoardState::Empty {
            self.board[yu][xu] = BoardState::Full(polyomino);
            return true
        }

        false
    }

    pub fn get(&self, x: i32, y: i32) -> BoardState<'a> {
        if x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height {
            return self.board[y as usize][x as usize]
        }
        
        BoardState::Void
    }

    pub fn add_polyomino(&mut self, p: &'a Polyomino, ll: Point) -> bool {
        if p.piter().any(|&pt| self.get(pt.x + ll.x, pt.y + ll.y) != BoardState::Empty) {
            return false;
        }

        for pt in p.piter() {
           self.set(pt.x + ll.x, pt.y + ll.y, p); 
        }
        
        true
    }
}
