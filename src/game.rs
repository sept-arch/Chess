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
        Piece {color, false, piece_type}
    }
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
            (0, "A"),
            (1, "B"),
            (2, "C"),
            (3, "D"),
            (4, "E"),
            (5, "F"),
            (6, "G"),
            (7, "H"),
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
        board[0].push(Piece::new(White, PieceType::Rook));
        board[0].push(Piece::new(White, PieceType::Knight));
        board[0].push(Piece::new(White, PieceType::Bishop));
        board[0].push(Piece::new(White, PieceType::King));
        board[0].push(Piece::new(White, PieceType::Queen));
        board[0].push(Piece::new(White, PieceType::Bishop));
        board[0].push(Piece::new(White, PieceType::Knight));
        board[0].push(Piece::new(White, PieceType::Rook));
        i = 0;
        //second row
        while i < 8 {
            let temp = Piece::new(White, PieceType::Pawn);
            board[1].push(temp);
            i += 1;
        }
        //rows 2-5 are empty, will use "null" piece to indicate empty space
        //Copy and pasted for 7-8, except black
        //first row
        board[7].push(Piece::new(White, PieceType::Rook));
        board[7].push(Piece::new(White, PieceType::Knight));
        board[7].push(Piece::new(White, PieceType::Bishop));
        board[7].push(Piece::new(White, PieceType::King));
        board[7].push(Piece::new(White, PieceType::Queen));
        board[7].push(Piece::new(White, PieceType::Bishop));
        board[7].push(Piece::new(White, PieceType::Knight));
        board[7].push(Piece::new(White, PieceType::Rook));
        i = 0;
        //second row
        while i < 8 {
            let temp = Piece::new(White, PieceType::Pawn);
            board[6].push(temp);
            i += 1;
        }

    }
}