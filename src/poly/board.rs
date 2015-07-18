#[derive(Clone)]
pub enum BoardState {
     Void,      // A hole in the board
     Empty,     // A valid part of the board, but no piece is there
     Full(i32)  // Has a piece
}

fn rep(b : &BoardStates) -> char {
    match *b {
        BoardState::Void => ' ',
        BoardState::Empty => '.',
        BoardState::Full(_i) => 'X'
    }
}

pub struct Board {
    height: i32,
    width: i32,
    board: Vec<Vec<BoardState>>
}

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

    pub fn get(x: i32, y: i32) -> BoardState {
    }
}
