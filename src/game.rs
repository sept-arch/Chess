use std::collections::HashMap;
/*NOTES
 - many functions in Rust return Result<T,E>
 - Consider using match Ok(T), Err(r), include Result<> and include ? to propogate errors
*/
mod board;
struct Piece {
    color: Color,
    captured: bool,
    piece_type: PieceType,
}

impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Piece {
        Piece {color, piece_type}
    }
}

//creating board first

enum Color {
    White,
    Black,
    Null,
}

enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Null,
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

pub struct Game {
    board: Board,

}

impl Game {
    pub fn new() -> Self {
        let columns = HashMap::from([
            (0, 'A'),
            (1, 'B'),
            (2, 'C'),
            (3, 'D'),
            (4, 'E'),
            (5, 'F'),
            (6, 'G'),
            (7, 'H'),
        ]);
        //Creates the board, first thing to be finally tested in main
        //first, create the pieces
        //needs to be ordered because of use of vectors
        let mut i = 0;
        //creates xy coordinate plane to be treated as board
        while i < 8 {
            board.push(Vec::new());
        }
        //first row
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Rook)), 'A', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Knight)), 'B', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Bishop)), 'C', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::King)), 'D', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Queen)), 'E', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Bishop)), 'F', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Knight)), 'G', 1);
        board[0].push(Space::new(Piece::new(Color::White, PieceType::Rook)), 'H', 1);
        //second row
        while i < 8 {
            let temp = Piece::new(White, PieceType::Pawn);
            board[1].push(Space::new(temp, columns[i], 2));
            i += 1;
        }
        //rows 2-5 are empty, will use "null" piece to indicate empty space
        for n in 2..6 {
            for k in 0..7 {
                board[n][k].push(Space::new(Piece::new(Color::Null, PieceType::Null)), columns[i], n + 1);
            }
        }
        //Copy and pasted for 7-8, except black
        i = 0;
        while i < 8 {
            let temp = Piece::new(Black, PieceType::Pawn);
            board[6].push(Space::new(temp, columns[i], 1));
            i += 1;
        }
        //first row
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Rook)), 'A', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Knight)), 'B', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Bishop)), 'C', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::King)), 'D', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Queen)), 'E', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Bishop)), 'F', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Knight)), 'G', 8);
        board[7].push(Space::new(Piece::new(Color::Black, PieceType::Rook)), 'H', 8);
        //second row


    }
}