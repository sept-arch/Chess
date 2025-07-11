use std::collections::HashMap;

#[derive(Clone, Eq, Hash, PartialEq)] //easier to duplicate squares
pub struct Square {
    pub file: char,
    pub rank: usize,
    pub piece: Option<Piece>,
}
impl Square {
    pub fn new(file: char, rank: usize, piece: Option<Piece>) -> Square {
        Square {
            file,
            rank,
            piece,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub captured: bool,
    pub piece_type: PieceType,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    //Null,
}


#[derive(Clone, Eq, Hash, PartialEq)]
pub enum Color {
    White,
    Black,
    //Null,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Piece { color, captured: false, piece_type, }
    }
}



pub struct IntToChar {
    int_to_char: HashMap<usize, char>,
}
//just to help with going from int to char in calculations; most likely unnecessary
impl IntToChar {
    pub fn new() -> Self {
        let int_to_char: HashMap<usize, char> = HashMap::from([
            (1, 'a'),
            (2, 'b'),
            (3, 'c'),
            (4, 'd'),
            (5, 'e'),
            (6, 'f'),
            (7, 'g'),
            (8, 'h'),
        ]);
        IntToChar { int_to_char }
    }
}

struct Player {
    check: bool,
    pieces: Vec<Piece>,
    color: Color,
    material_advantage: usize,

}

impl Player {

    pub fn new(color: Color) -> Player {
        let mut pieces : Vec<Piece> = Vec::new();
        for _n in 1..9 {
            pieces.push(Piece::new(color.clone(), PieceType::Pawn));
        }
        for _n in 1..3 {
            pieces.push(Piece::new(color.clone(), PieceType::Rook));
            pieces.push(Piece::new(color.clone(), PieceType::Knight));
            pieces.push(Piece::new(color.clone(), PieceType::Bishop));
        }
        pieces.push(Piece::new(color.clone(), PieceType::Queen));
        pieces.push(Piece::new(color.clone(), PieceType::King));
        Player { check: false, pieces, color, material_advantage: 0, }
    }

    pub fn material_captured(piece_type: PieceType) -> Result<u8, &'static str> {
        match piece_type {
            PieceType::Pawn => Ok(1),
            PieceType::Rook => Ok(5),
            PieceType::Knight => Ok(3),
            PieceType::Bishop => Ok(3),
            PieceType::Queen => Ok(10),
            PieceType::King => Err("king cannot be captured"),
        }
    }
}

pub struct Game {
    pub board: Vec<Vec<Square>>,
    pub turn: Color,
    pub player1: Player,
    pub player2: Player,
    pub piece_list: HashMap<usize, Option<Square>>,
}
//idea, have each number correlate to a piece, and another thing where each number correlates to a square
impl Game {
    pub fn new() -> Self {
        let int_to_char = IntToChar::new();
        let mut board: Vec<Vec<Square>> = Vec::new();
        let mut rank: Vec<Square> = Vec::new();
        rank.push(Square::new('a', 1usize, Some(Piece::new(Color::White, PieceType::Rook))));
        rank.push(Square::new('b', 1usize, Some(Piece::new(Color::White, PieceType::Knight))));
        rank.push(Square::new('c', 1usize, Some(Piece::new(Color::White, PieceType::Bishop))));
        rank.push(Square::new('d', 1usize, Some(Piece::new(Color::White, PieceType::Queen))));
        rank.push(Square::new('e', 1usize, Some(Piece::new(Color::White, PieceType::King))));
        rank.push(Square::new('f', 1usize, Some(Piece::new(Color::White, PieceType::Bishop))));
        rank.push(Square::new('g', 1usize, Some(Piece::new(Color::White, PieceType::Rook))));
        rank.push(Square::new('h', 1usize, Some(Piece::new(Color::White, PieceType::Knight))));
        board.push(rank);
        let mut rank2: Vec<Square> = Vec::new();
        let mut rank3: Vec<Square> = Vec::new();
        let mut rank4: Vec<Square> = Vec::new();
        let mut rank5: Vec<Square> = Vec::new();
        let mut rank6: Vec<Square> = Vec::new();
        let mut rank7: Vec<Square> = Vec::new();
        for n in 1..9 {
            rank2.push(Square::new(int_to_char.int_to_char[&n], 2, Some(Piece::new(Color::White, PieceType::Pawn))));
            rank3.push(Square::new(int_to_char.int_to_char[&n], 3, None));
            rank4.push(Square::new(int_to_char.int_to_char[&n], 4, None));
            rank5.push(Square::new(int_to_char.int_to_char[&n], 5, None));
            rank6.push(Square::new(int_to_char.int_to_char[&n], 6, None));
            rank7.push(Square::new(int_to_char.int_to_char[&n], 2, Some(Piece::new(Color::Black, PieceType::Pawn))));
        }
        let mut rank8: Vec<Square> = Vec::new();
        rank8.push(Square::new('a', 1usize, Some(Piece::new(Color::Black, PieceType::Rook))));
        rank8.push(Square::new('b', 1usize, Some(Piece::new(Color::Black, PieceType::Knight))));
        rank8.push(Square::new('c', 1usize, Some(Piece::new(Color::Black, PieceType::Bishop))));
        rank8.push(Square::new('d', 1usize, Some(Piece::new(Color::Black, PieceType::Queen))));
        rank8.push(Square::new('e', 1usize, Some(Piece::new(Color::Black, PieceType::King))));
        rank8.push(Square::new('f', 1usize, Some(Piece::new(Color::Black, PieceType::Bishop))));
        rank8.push(Square::new('g', 1usize, Some(Piece::new(Color::Black, PieceType::Rook))));
        rank8.push(Square::new('h', 1usize, Some(Piece::new(Color::Black, PieceType::Knight))));
        board.push(rank2);
        board.push(rank3);
        board.push(rank4);
        board.push(rank5);
        board.push(rank6);
        board.push(rank7);
        board.push(rank8);
        let player1 = Player::new(Color::White);
        let player2 = Player::new(Color::Black);
        //piece_list
        let mut piece_list: HashMap<usize, Option<Square>> = HashMap::new();
        for n in 1..9 {
            piece_list.insert(n, Some(board[n - 1][1].clone())); //white pawns on second rank
            piece_list.insert(n + 16, Some(board[n - 1][6].clone()));
            piece_list.insert(8 + n, Some(board[n - 1][0].clone()));
            piece_list.insert(8 + n + 16, Some(board[n - 1][7].clone()));
        }
    Game {
        board,
        player1,
        player2,
        turn: Color::White,
        piece_list,
        }
    }
    //help function for line of sight?
    //before any move, check if the king is in check
    //List of all opponent's legal moves, to see if any of them involve the capture of the king

    pub fn check(&self) -> bool {
        //check whose turn it is; check opposite's sides possible moves
        //go through the game.piece_list at either 1 or 17 (white or black)
        //see if any move involves the capture of 13 (white king) or 29 (black king)
        if self.turn == Color::White {

        }
        else {
            false
        }
    }
    //list of legal moves of all pieces if king is not in check


}

//int_to_piece
pub fn int_to_piece(n: usize) -> Result<Piece, &'static str> {
    let temp = HashMap::from([
        (1, Piece::new(Color::White, PieceType::Pawn)),
        (2, Piece::new(Color::White, PieceType::Pawn)),
        (3, Piece::new(Color::White, PieceType::Pawn)),
        (4, Piece::new(Color::White, PieceType::Pawn)),
        (5, Piece::new(Color::White, PieceType::Pawn)),
        (6, Piece::new(Color::White, PieceType::Pawn)),
        (7, Piece::new(Color::White, PieceType::Pawn)),
        (8, Piece::new(Color::White, PieceType::Pawn)),
        (9, Piece::new(Color::White, PieceType::Rook)),
        (10, Piece::new(Color::White, PieceType::Knight)),
        (11, Piece::new(Color::White, PieceType::Bishop)),
        (12, Piece::new(Color::White, PieceType::Queen)),
        (13, Piece::new(Color::White, PieceType::King)),
        (14, Piece::new(Color::White, PieceType::Bishop)),
        (15, Piece::new(Color::White, PieceType::Knight)),
        (16, Piece::new(Color::White, PieceType::Rook)),
        (17, Piece::new(Color::Black, PieceType::Pawn)),
        (18, Piece::new(Color::Black, PieceType::Pawn)),
        (19, Piece::new(Color::Black, PieceType::Pawn)),
        (20, Piece::new(Color::Black, PieceType::Pawn)),
        (21, Piece::new(Color::Black, PieceType::Pawn)),
        (22, Piece::new(Color::Black, PieceType::Pawn)),
        (23, Piece::new(Color::Black, PieceType::Pawn)),
        (24, Piece::new(Color::Black, PieceType::Pawn)),
        (25, Piece::new(Color::Black, PieceType::Rook)),
        (26, Piece::new(Color::Black, PieceType::Knight)),
        (27, Piece::new(Color::Black, PieceType::Bishop)),
        (28, Piece::new(Color::Black, PieceType::Queen)),
        (29, Piece::new(Color::Black, PieceType::King)),
        (30, Piece::new(Color::Black, PieceType::Bishop)),
        (31, Piece::new(Color::Black, PieceType::Knight)),
        (32, Piece::new(Color::Black, PieceType::Rook)),
    ]);
    if temp.contains_key(&n) {
        Ok(temp[&n].clone())
    }
    else {
        Err("no such piece")
    }
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
        Square::new('d', 4, None);
    }
    #[test]
    fn create_board() {
        let game = Game::new();
        for n in game.piece_list.keys() {
            println!("{}", n);
        }
    }
}
