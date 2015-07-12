enum BoardStates {
     Void,      // A hole in the board
     Empty,     // A valid part of the board, but no piece is there
     Full(i32)  // Has a piece
}

