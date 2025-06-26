use std::collections::HashMap;

mod board;
struct Piece {
    color: Color,
    position: Square, // will read position in x, y, and translate to algebraic notation
    piece_type: PieceType,
}

//creating board first

enum Color {
    White,
    Black,
}

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}



enum MoveType {
    BasicMove,
    KingsideCastle,
    Capture,
    EnPassant,
    QueenSideCastle,
    Check,
    Checkmate

}

//functions to move pieces
impl Piece {

    fn legal_moves(&self, board: &Board) -> Vec<Square> {}
        //TODO: Create a vector of squares a piece can move to

    fn turn(&self, color: Color, legal: Vec<Square>) -> Piece {
    //TODO: call legal_moves for legal, and if move does not match list then force player to re-move
    }
}

mod game {
    fn begin() {
        //TODO: Create signatures and construct initial board
    }
}