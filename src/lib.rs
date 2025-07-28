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

#[derive(Component)]
pub struct Select;

pub fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let offset = TILE_SIZE * (BOARD_SIZE as f32) / 2.0;

    for row in 2..BOARD_SIZE - 2 {
        for col in 'a'..='h' {
            let is_light = (row + (col as usize - 'a' as usize)) % 2 == 0;
            let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

            let x = ((col as u8 - b'a') as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
            let y = (row as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
            let char_to_usize = col as usize - 97;
            let pos = Position {x: char_to_usize, y: row};
            commands.spawn((SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
               Square::new(col, row + 1, None), pos
            ));
        }
    }
    //pawn squares; need to put ids
    //sure there was some way to do this more efficiently
    //white pawn squares
    for n in 'a'..='h' {
        let is_light = (1 + (n as usize - 'a' as usize)) % 2 == 0;
        let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

        let x = ((n as u8 - b'a') as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
        let y = (1 as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;

        let char_to_usize = n as usize - 97;
        let pos = Position {x: char_to_usize,y:  1};
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
            },
            Square::new(n, 2, Some(n as usize - 'a' as usize)), pos
        ));
    }
    //black pawns
    for n in 'a'..='h' {
        let is_light = ((n as usize - 'a' as usize)) % 2 == 0;
        let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

        let x = ((n as u8 - b'a') as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
        let y = (0 as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;

        let char_to_usize = n as usize - 97;
        let pos = Position {x: char_to_usize, y: 0};
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
            Square::new(n, 1, Some((n as usize - 'a' as usize) + 8)), pos
        ));
    }
    for n in 'a'..='h' {
        let is_light = (6 + (n as usize - 'a' as usize)) % 2 == 0;
        let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

        let x = ((n as u8 - b'a') as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
        let y = (6 as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;


        let char_to_usize = n as usize - 97;
        let pos = Position {x: char_to_usize,y:  6};
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
            Square::new(n, 7, Some(n as usize - 'a' as usize)), pos
        ));
    }
    for n in 'a'..='h' {
        let is_light = (7 + (n as usize - 'a' as usize)) % 2 == 0;
        let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

        let x = ((n as u8 - b'a') as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
        let y = (7 as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;

        let char_to_usize = n as usize - 97;
        let pos = Position {x: char_to_usize, y: 7};

        commands.spawn((SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::splat(TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 0.0),
            ..default()
        },
            Square::new(n, 8, Some(n as usize - 'a' as usize)), pos
        ));
    }
    //pawns
    let white_pawn = asset_server.load("pieces/white_pawn.png");
    let black_pawn = asset_server.load("pieces/black_pawn.png");
    let y_white = 1f32 * TILE_SIZE - offset + TILE_SIZE / 2.0;
    let y_black = 6f32 * TILE_SIZE - offset + TILE_SIZE / 2.0;
    for col in 0..8 {
        let x = (col as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
        let usize_to_char: Option<char> = char::from_u32(97 + col as u32);
        commands.spawn((SpriteBundle {
            texture: white_pawn.clone(),
            transform: Transform::from_xyz(x, y_white, 0.0),
            ..default()
        }, Square::new(usize_to_char.expect("WTF"),1usize, Some(col + 1)), Piece::new(Team::White, PieceType::Pawn, col + 1), Select, Position { x: col, y: 1 }
        ));
        commands.spawn((SpriteBundle {
            texture: black_pawn.clone(),
            transform: Transform::from_xyz(x, y_black, 0.0),
            ..default()
        }, Square::new(usize_to_char.expect("WTF"), 7usize, Some(col + 1)), Piece::new(Team::Black, PieceType::Pawn, col + 9), Select, Position { x: col, y: 6 }
        ));
    }
    //rest of the pieces
    //rooks
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_rook.png"),
        transform: Transform::from_xyz(TILE_SIZE / 2.0 - offset, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('h', 1usize, Some(16)), Piece::new(Team::White, PieceType::Rook, 16), Select, Position{x: 0, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_rook.png"),
        transform: Transform::from_xyz( 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('a', 1usize, Some(9)), Piece::new(Team::White, PieceType::Rook, 9), Select, Position{x: 7, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_knight.png"),
        transform: Transform::from_xyz( 1f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('g', 1usize, Some(15)), Piece::new(Team::White, PieceType::Knight, 15), Select, Position{x: 1, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_knight.png"),
        transform: Transform::from_xyz( 6f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('b', 1usize, Some(10)), Piece::new(Team::White, PieceType::Knight, 10), Select, Position{x: 6, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_bishop.png"),
        transform: Transform::from_xyz( 2f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('g', 1usize, Some(11)), Piece::new(Team::White, PieceType::Bishop, 11), Select, Position{x:2, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_bishop.png"),
        transform: Transform::from_xyz( 5f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('c', 1usize, Some(14)), Piece::new(Team::White, PieceType::Bishop, 14), Select, Position{x: 5, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_king.png"),
        transform: Transform::from_xyz( 4f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('e', 1usize, Some(13)), Piece::new(Team::White, PieceType::King, 13), Select, Position{x:4, y: 0}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/white_queen.png"),
        transform: Transform::from_xyz( 3f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    }, Square::new('d', 1usize, Some(12)), Piece::new(Team::White, PieceType::Queen, 12), Select, Position{x:3, y: 0}
    ));
    //black pieces
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_rook.png"),
        transform: Transform::from_xyz(TILE_SIZE / 2.0 - offset, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('h', 8usize, Some(32)), Piece::new(Team::Black, PieceType::Rook, 32), Select, Position{x:7, y: 7}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_rook.png"),
        transform: Transform::from_xyz( 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('a', 8usize, Some(25)), Piece::new(Team::Black, PieceType::Rook, 25), Select, Position{x:0, y: 7}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_knight.png"),
        transform: Transform::from_xyz( 1f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('g', 8usize, Some(31)), Piece::new(Team::Black, PieceType::Knight, 31), Select, Position{x:6, y: 7}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_knight.png"),
        transform: Transform::from_xyz( 6f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('b', 8usize, Some(26)), Piece::new(Team::Black, PieceType::Knight, 26), Select, Position{x:1, y: 7}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_bishop.png"),
        transform: Transform::from_xyz( 2f32 * TILE_SIZE - offset + TILE_SIZE / 2.0,7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('f', 8usize, Some(30)), Piece::new(Team::Black, PieceType::Bishop, 30), Select, Position{x:5, y: 7}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_bishop.png"),
        transform: Transform::from_xyz( 5f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('c', 8usize, Some(27)), Piece::new(Team::Black, PieceType::Bishop, 27), Select, Position{x:2, y: 7}
    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_king.png"),
        transform: Transform::from_xyz( 4f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('e', 8usize, Some(29)), Piece::new(Team::Black, PieceType::King, 29), Select, Position{x:4, y: 7}

    ));
    commands.spawn((SpriteBundle {
        texture: asset_server.load("pieces/black_queen.png"),
        transform: Transform::from_xyz( 3f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, 0.0),
        ..default()
    }, Square::new('d', 8usize, Some(28)), Piece::new(Team::Black, PieceType::Queen, 28), Select, Position{x:3, y: 7}
    ));
}

//legal move check required
//go through ids, and then the squares using a query
//helper function: see if square is occupied: if it is, it returns the piece. If not, None
fn piece_at_position(
    query: &Vec<(Square, Option<Piece>, Position)>,
    target: Position,
) -> Option<&Option<Piece>> {
    query.iter().find(|(_, _, position)| *position == target).map(|_, piece, _| piece)
}
//helper function
pub fn possible_moves(game: &Game, query: Vec<(Square, Option<Piece>, Position)>) -> HashMap<usize, Vec<(Square, Position)>> {
    let possible: HashMap<usize, Vec<(Square, Position)>> = HashMap::new();
    if game.turn == Team::White {
        for (square, piece, position) in query.iter() {
            let mut moves: Square = Vec::new();
            if let Some(piece) = piece {
                if piece.team == Team::White {
                    match piece.id {
                        //white pawns
                        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                            //check square in front to see if it is occupied
                            //use helper function to find if a specific square is occupied
                            let front = Position {x: position.x, y: position.y + 1};
                            if piece_at_position(&query, &front) == None {
                                //**NOTE** PLEASE FIX THE SQUARE: IT IS NOT THE SAME AS FRONT: DO SOME ASCII MATH
                                moves.push(Square {file: char::from_u32(97 + position.x), rank: position.y + 2, id: None }, front);
                            }
                            //As far as I am aware, there should be no reason a pawn is in this file without having made any moves
                            if (square.file == 2 && piece_at_position(&query, Position {x: position.x, y: position.y + 2})) {
                                //HERE TOO
                                moves.push(Square {file: char::from_u32(97 + position.x), rank: position.y + 3, id: None }, Position{x: position.x, y: position.y + 2});
                            }
                            /*right diagonal potential capture; must consider if a piece is on the
                            edge of the board*/
                            if piece.rank != 'h' {
                                let enemy = Position {x: position.x + 1, y: position.y + 1};
                                if piece_at_position(&query, &enemy).unwrap().team == Team::Black {
                                //DON'T MAKE THE SAME MISTAKE
                                    moves.push(Square {file: char::from_u32(98 + position.x), rank: position.y + 2,  id: piece_at_position(&query, &enemy).map(|p| p.id)},  enemy);
                                }
                            }
                            // left diagonal
                            if piece.rank != 'a' {
                                let enemy = Position {x: position.x - 1, y: position.y + 1};
                                if piece_at_position(&query, &enemy).unwrap().team == Team::Black {
                                    moves.push(Square {file: char::from_u32(96 + position.x), rank: position.y + 2,  id: piece_at_position(&query, &enemy).map(|p| p.id)},  enemy);
                                }
                            }
                        }
                        //rooks
                        9 | 16 => {
                            //loop horizontal and vertical movement until the edge of the board is reached,
                            //or a piece that can be captured is reached, or a piece that is on the same team
                            //loop for left movement
                            if piece.file != 'a' {
                                let mut side = Position {x: position.x - 1, y: position.y};
                                //if square is empty, add to list
                                while piece_at_position(&query, &side) == None {
                                    moves.push(Square {file: char::from_u32(97 + side.x), rank: side.y,  id: None},  side);
                                    if side.x == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x - 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + side.x), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side);
                                }
                            }
                            //loop for right movement
                            if piece.file != 'h' {
                                let mut side = Position {x: position.x + 1, y: position.y};
                                while piece_at_position(&query, &side) == None {
                                    moves.push(Square {file: char::from_u32(97 + side.x), rank: side.y,  id: None},  side);
                                    if side.x == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x + 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + side.x), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side);
                                }
                            }
                            //upward movement
                            if piece.rank != 8 {
                                let mut side = Position {x: position.x, y: position.y + 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push(Square {file: square.file, rank: side.y + 1,  id: None},  side);
                                    if side.y == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y + 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side);
                                }
                            }
                            //downward
                            if piece.rank != 1 {
                                let mut side = Position {x: position.x, y: position.y - 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push(Square {file: square.file, rank: side.y + 1,  id: None},  side);
                                    if side.y == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y - 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side);
                                }
                            }
                        }
                        //knights
                        10 | 15 => {
                            //here on out, use positions instead of ranks and files
                            //would use a more efficient system, but I want to avoid corner cases screwing up code,
                            //so I will go L by L
                            //horizontal upper left
                            if position.x > 1 && position.y < 7 {
                                let potential = Position {x: position.x - 2, y: position.y + 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //vertical upper left
                            if position.x > 0 && position.y < 6 {
                                let potential = Position {x: position.x - 1, y: position.y + 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //vertical upper right
                            if position.x < 7 && position.y < 6 {
                                let potential = Position {x: position.x + 1, y: position.y + 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //horizontal upper right
                            if position.x < 6 && position.y < 7 {
                                let potential = Position {x: position.x + 2, y: position.y + 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //horizontal lower right
                            if position.x < 6 && position.y > 0 {
                                let potential = Position {x: position.x + 2, y: position.y - 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //vertical lower right
                            if position.x < 7 && position.y > 1 {
                                let potential = Position {x: position.x + 1, y: position.y - 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //vertical lower left
                            if position.x > 0 && position.y > 1 {
                                let potential = Position {x: position.x - 1, y: position.y - 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                            //horizontal lower left
                            if position.x > 1 && position.y > 0 {
                                let potential = Position {x: position.x - 2, y: position.y - 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + potential.x), rank: potential.y + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  potential);
                                }
                            }
                        }
                        //bishops
                        11 | 14 => {
                            //up and right
                            let mut temp = Position {x: position.x + 1, y: position.y + 1};
                            while temp.x < 7 && temp.y < 7 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push(Square {file: char::from_u32(97 + temp.x), rank: temp.y + 1, id: None}, temp);
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push(Square {file: char::from_u32(97 + temp.x), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp);
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x + 1, y: temp.y + 1};
                            }
                            //upper left
                        }
                        //queen
                        12 => {

                        }
                        //king
                        13 => {

                        }
                    }
                }
            }
            possible.insert(piece.id, moves);
        }
    }
    else {

    }
}