use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)] //easier to duplicate squares
pub struct Square {
    pub file: char,
    pub rank: u8
}
impl Square {
    pub fn new(file: char, rank: u8) -> Square {
        Square {file, rank }
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

pub struct Game {
    pub board: HashMap<Square, Option<Piece>>,
}

impl Game {
    pub fn new() -> Self {
        let int_to_char = HashMap::from([
            (1, 'a'),
            (2, 'b'),
            (3, 'c'),
            (4, 'd'),
            (5, 'e'),
            (6, 'f'),
            (7, 'g'),
            (8, 'h'),
        ]);
        let mut board = HashMap::new();
        //pawns and null
        for n in 1..9 {
            board.insert(Square::new(int_to_char[&n], 2), Piece::new(Color::White, PieceType::Pawn));
            board.insert(Square::new(int_to_char[&n], 7), Piece::new(Color::Black, PieceType::Pawn));
            board.insert(Square::new(int_to_char[&n], 3), None);
            board.insert(Square::new(int_to_char[&n], 4), None);
            board.insert(Square::new(int_to_char[&n], 5), None);
            board.insert(Square::new(int_to_char[&n], 6), None);
        }
        //white back file
        board.insert(Square::new('a', 1), Piece::new(Color::White, PieceType::Rook));
        board.insert(Square::new('h', 1), Piece::new(Color::White, PieceType::Rook));
        board.insert(Square::new('b', 1), Piece::new(Color::White, PieceType::Knight));
        board.insert(Square::new('g', 1), Piece::new(Color::White, PieceType::Knight));
        board.insert(Square::new('c', 1), Piece::new(Color::White, PieceType::Bishop));
        board.insert(Square::new('f', 1), Piece::new(Color::White, PieceType::Bishop));
        board.insert(Square::new('e', 1), Piece::new(Color::White, PieceType::King));
        board.insert(Square::new('d', 1), Piece::new(Color::White, PieceType::Queen));
        //black back file
        board.insert(Square::new('a', 8), Piece::new(Color::Black, PieceType::Rook));
        board.insert(Square::new('h', 8), Piece::new(Color::Black, PieceType::Rook));
        board.insert(Square::new('b', 8), Piece::new(Color::Black, PieceType::Knight));
        board.insert(Square::new('g', 8), Piece::new(Color::Black, PieceType::Knight));
        board.insert(Square::new('c', 8), Piece::new(Color::Black, PieceType::Bishop));
        board.insert(Square::new('f', 8), Piece::new(Color::Black, PieceType::Bishop));
        board.insert(Square::new('e', 8), Piece::new(Color::Black, PieceType::King));
        board.insert(Square::new('d', 8), Piece::new(Color::Black, PieceType::Queen));
    Game {
       board,
        }
    }
    //only for testing purposes
}

#[derive(Clone)]
pub enum Color {
    White,
    Black,
    //Null,
}

#[derive(Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    //Null,
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_piece() {
        Piece::new(Color::Black, PieceType::Pawn);
    }
    #[test]
    fn make_square() {
        Square::new('D', 4);
    }
    #[test]
    fn create_board() {
        Game::new();
}
}