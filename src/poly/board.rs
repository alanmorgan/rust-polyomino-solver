#[derive(Clone, Copy)]
pub enum BoardState {
     Void,      // Out of bounds/a hole in the board
     Empty,     // A valid part of the board, but no piece is there
     Full(i32)  // Has a piece
}

fn rep(b : &BoardState) -> char {
    match *b {
        BoardState::Void => ' ',
        BoardState::Empty => '.',
        BoardState::Full(_i) => 'X'
    }
}

#[allow(dead_code)]
pub struct Board {
    height: i32,
    width: i32,
    board: Vec<Vec<BoardState>>
}

#[allow(dead_code)]
impl Board {
    pub fn new(h: i32, w: i32) -> Board {
        
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

    pub fn get(&self, x: usize, y: usize) -> BoardState {
        if let Some(v) = self.board.get(y) {
            if let Some(bs) = v.get(x) {
                return *bs
            }
        }
        BoardState::Void
    }
}
