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
    Move,
    KingsideCastle,
    Capture,
    EnPassant,
    QueenSideCastle,
    Check,
    Checkmate

}


mod game {

}