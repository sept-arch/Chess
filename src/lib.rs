#[derive(Clone)] //easier to duplicate squares
pub struct Square {
    pub piece: Piece,
    pub file: char,
    pub rank: u8
}
impl Square {
    pub fn new(piece: Piece, file: char, rank: u8) -> Square {
        Square { piece, file, rank }
    }
}

#[derive(Clone)]
pub struct Piece {
    pub color: Color,
    pub captured: bool,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, captured: false, piece_type, }
    }
}

#[derive(Clone)]
pub enum Color {
    White,
    Black,
    Null,
}

#[derive(Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Null,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_piece() {
        Piece::new(Color::Black, PieceType::Pawn);
    }
}