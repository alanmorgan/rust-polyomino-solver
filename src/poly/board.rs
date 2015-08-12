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

    pub fn set(&mut self, x: usize, y: usize, polyomino: &'a Polyomino) -> bool {
        if x < self.width && y < self.height && self.board[x][y] == BoardState::Empty {
            self.board[x][y] = BoardState::Full(polyomino);
            return true
        }

        false
    }

    pub fn get(&self, x: usize, y: usize) -> BoardState<'a> {
        if x < self.width && y < self.height {
            return self.board[x][y]
        }
        
        BoardState::Void
    }

    pub fn add(&self, p: Polyomino, ll: Point) -> bool {
        if p.points.iter().any(|&pt| self.get((pt.x + ll.x) as usize, (pt.y + ll.y) as usize) != BoardState::Empty) {
            return false
        }

        true
    }
}
