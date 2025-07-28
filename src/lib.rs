use std::collections::HashMap;
use bevy::prelude::*;

const TILE_SIZE: f32 = 90.0;
const BOARD_SIZE: usize = 8;
const LIGHT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const DARK_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

//for math
#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Clone, Eq, Hash, PartialEq)] //easier to duplicate squares
pub struct Square {
    pub file: char,
    pub rank: usize,
    pub id: Option<usize>,
}
impl Square {
    pub fn new(file: char, rank: usize, id: Option<usize>) -> Square {
        Square {
            file,
            rank,
            id,
        }
    }
}

#[derive(Component, Clone, Eq, Hash, PartialEq)]
enum MoveType {
    Move,
    Capture,
    Castle,
}



#[derive(Component, Clone, Eq, Hash, PartialEq)]
pub struct Piece {
    pub team: Team,
    pub captured: bool,
    pub piece_type: PieceType,
    pub id: usize,
    pub special: bool,
}

#[derive(Component, Clone, Eq, Hash, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    //Null,
}


#[derive(Component, Clone, Eq, Hash, PartialEq)]
pub enum Team {
    White,
    Black,
    //Null,
}

impl Piece {
    pub fn new(team: Team, piece_type: PieceType, id: usize,) -> Self {
        Piece { team, captured: false, piece_type, id, special: false }
    }
}



pub struct IntToChar {
    int_to_char: HashMap<usize, char>,
}
//just to help with going from int to char in calculations; most likely unnecessary
impl IntToChar {
    //dead code
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

#[derive(Component, Clone, Eq, Hash, PartialEq)]
struct Player {
    check: bool,
    team: Team,
    material_advantage: usize,

}

impl Player {

    pub fn new(team: Team) -> Player {
        Player { check: false, team, material_advantage: 0, }
    }

    pub fn material_captured(piece_type: PieceType) -> Result<u8, &'static str> {
        match piece_type {
            PieceType::Pawn => Ok(1),
            PieceType::Rook => Ok(5),
            PieceType::Knight | PieceType::Bishop => Ok(3),
            PieceType::Queen => Ok(10),
            PieceType::King => Err("king cannot be captured"),
        }
    }
}

#[derive(Component)]
pub struct Game {
    pub turn: Team,
    pub player1: Player,
    pub player2: Player,
    //pub piece_list: HashMap<usize, Option<Square>>, //rewrite so it is usize, (char, usize)
}
//idea, have each number correlate to a piece, and another thing where each number correlates to a square
impl Game {
    //dead code
    /*pub fn new() -> Self {
        let int_to_char = IntToChar::new();
        let mut board: Vec<Vec<Square>> = Vec::new();
        let mut rank: Vec<Square> = Vec::new();
        rank.push(Square::new('a', 1usize, Some(9)));
        rank.push(Square::new('b', 1usize, Some(10)));
        rank.push(Square::new('c', 1usize, Some(11)));
        rank.push(Square::new('d', 1usize, Some(12)));
        rank.push(Square::new('e', 1usize, Some(13)));
        rank.push(Square::new('f', 1usize, Some(14)));
        rank.push(Square::new('g', 1usize, Some(15)));
        rank.push(Square::new('h', 1usize, Some(16)));
        board.push(rank);
        let mut rank2: Vec<Square> = Vec::new();
        let mut rank3: Vec<Square> = Vec::new();
        let mut rank4: Vec<Square> = Vec::new();
        let mut rank5: Vec<Square> = Vec::new();
        let mut rank6: Vec<Square> = Vec::new();
        let mut rank7: Vec<Square> = Vec::new();
        for n in 1..9 {
            rank2.push(Square::new(int_to_char.int_to_char[&n], 2, Some( n + 8)));
            rank3.push(Square::new(int_to_char.int_to_char[&n], 3, None));
            rank4.push(Square::new(int_to_char.int_to_char[&n], 4, None));
            rank5.push(Square::new(int_to_char.int_to_char[&n], 5, None));
            rank6.push(Square::new(int_to_char.int_to_char[&n], 6, None));
            rank7.push(Square::new(int_to_char.int_to_char[&n], 7, Some(n + 16)));
        }
        let mut rank8: Vec<Square> = Vec::new();
        rank8.push(Square::new('a', 8usize, Some(25)));
        rank8.push(Square::new('b', 8usize, Some(26)));
        rank8.push(Square::new('c', 8usize, Some(27)));
        rank8.push(Square::new('d', 8usize, Some(28)));
        rank8.push(Square::new('e', 8usize, Some(29)));
        rank8.push(Square::new('f', 8usize, Some(30)));
        rank8.push(Square::new('g', 8usize, Some(31)));
        rank8.push(Square::new('h', 8usize, Some(32)));
        board.push(rank2);
        board.push(rank3);
        board.push(rank4);
        board.push(rank5);
        board.push(rank6);
        board.push(rank7);
        board.push(rank8);
        let player1 = Player::new(Team::White);
        let player2 = Player::new(Team::Black);

    Game {
        player1,
        player2,
        turn: Team::White,
        }
    }*/
    //help function for line of sight?
    //before any move, check if the king is in check
    //List of all opponent's legal moves, to see if any of them involve the capture of the king


    pub fn check(&self) -> bool {
        //check whose turn it is; check opposite's sides possible moves
        //go through the game.piece_list at either 1 or 17 (white or black)
        //see if any move involves the capture of 13 (white king) or 29 (black king)
        //Want to figure out a way if a move would be illegal without moving anything
        if self.turn == Team::White {
            //do a special_rights function here, match for castling and passant
            false
        }
        else {
            false
        }
    }
    //list of legal moves of all pieces if king is not in check
    //id, legal moves
    /*pub fn legal_moves(&self) -> HashMap(usize, vec<Square>) {
        if self.turn == Team::White {
            for n in 1usize..=17 {
                let mut moves: Square = Vec::new();
                if !int_to_piece(n).unwrap().captured {
                    match n {
                         1 => if rank == 2 && self.board[0][2].piece.is_none() {
                             moves.push(Square::new('a', 1));
                         }

                    }
                }
            }
        }
        if self.turn == Team::Black {

        }
    }*/

}







#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn make_piece() {
        Piece::new(Team::Black, PieceType::Pawn, 17);
    }
    #[test]
    fn make_square() {
        //Square::new('d', 4, None);
    }
}
//game/bevy functions
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

