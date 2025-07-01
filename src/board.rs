use std::collections::HashMap;
struct Square {
    piece: Piece,
    file: char,
    rank: u8,
}
impl Square {
    fn new(piece: Piece, file: char, rank: u8) -> Square {
        //Used to update squares
        Square {piece, file, rank,}
    }
}

struct Board {
    board: Vec<Vec<Square>>,
}
