//see if it shows up in GitHub
use crate::*;
use bevy::prelude::*;
use std::collections::HashMap;


const TILE_SIZE: f32 = 90.0;
const BOARD_SIZE: usize = 8;
//to be used in possible_moves as an argument
fn make_board(query: Query<(&Square, Option<&Piece>, &Position)>) -> Vec<(Square, Option<Piece> , Position)> {

query.iter().map(|(square, piece_opt, position)| (square.clone(), piece_opt.clone().map(|p| p.clone()), position.clone())).collect()

}

//legal move check required
//go through ids, and then the squares using a query
//helper function: see if square is occupied: if it is, it returns the piece. If not, None
fn piece_at_position<'a>(
    query: &'a Vec<(Square, Option<Piece>, Position)>,
    target: &'a Position,
) -> Option<&'a Piece> {
    query.iter().find(|(_, _, position)| *position == *target).and_then(|(_, piece, _)| piece.as_ref())
}

//helper function
pub fn possible_moves(team: Team, query: Vec<(Square, Option<Piece>, Position)>) -> HashMap<usize, Vec<(Square, Position)>> {
    let mut possible: HashMap<usize, Vec<(Square, Position)>> = HashMap::new();
    if team == Team::White {
        for (square, piece, position) in query.iter() {
            let mut moves: Vec<(Square, Position)> = Vec::new();
            if let Some(piece) = piece {
                if piece.team == Team::White {
                    match piece.id {
                        //white pawns
                        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
                            //check square in front to see if it is occupied
                            //use helper function to find if a specific square is occupied
                            let front = Position {x: position.x, y: position.y + 1};
                            if piece_at_position(&query, &front) == None {
                                //NOTE PLEASE FIX THE SQUARE: IT IS NOT THE SAME AS FRONT: DO SOME ASCII MATH
                                moves.push((Square {file: char::from_u32(97 + front.x as u32).expect("will always be lowercase"), rank: position.y + 2, id: None }, front));
                            }
                            //As far as I am aware, there should be no reason a pawn is in this file without having made any moves
                            if position.y == 1 && piece_at_position(&query, &Position {x: position.x, y: position.y + 2}).is_none() {
                                //HERE TOO
                                moves.push((Square {file: char::from_u32(97 + position.x as u32).expect("will always be lowercase"), rank: position.y + 3, id: None }, Position{x: position.x, y: position.y + 2}));
                            }
                            /*right diagonal potential capture; must consider if a piece is on the
                            edge of the board*/
                            if position.x != 7 {
                                let enemy = Position {x: position.x + 1, y: position.y + 1};
                                if piece_at_position(&query, &enemy).map_or(false, |p| p.team == Team::Black) {
                                    //DON'T MAKE THE SAME MISTAKE
                                    moves.push((Square {file: char::from_u32(96 + position.x as u32).expect("will always be lowercase"), rank: position.y + 2,  id: piece_at_position(&query, &enemy).map(|p| p.id)},  enemy));
                                }
                            }
                            // left diagonal
                            if position.x != 0 {
                                let enemy = Position {x: position.x - 1, y: position.y + 1};
                                if piece_at_position(&query, &enemy).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(98 + position.x as u32).expect("will always be lowercase"), rank: position.y + 2,  id: piece_at_position(&query, &enemy).map(|p| p.id)},  enemy));
                                }
                            }
                        }
                        //rooks
                        9 | 16 => {
                            //loop horizontal and vertical movement until the edge of the board is reached,
                            //or a piece that can be captured is reached, or a piece that is on the same team
                            //loop for left movement
                            if position.x > 0 {
                                let mut side = Position {x: position.x - 1, y: position.y};
                                //if square is empty, add to list
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x - 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //loop for right movement
                            if position.x < 7 {
                                let mut side = Position {x: position.x + 1, y: position.y};
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x + 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //upward movement
                            if position.y < 7 {
                                let mut side = Position {x: position.x, y: position.y + 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y + 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //downward
                            if position.y > 0 {
                                let mut side = Position {x: position.x, y: position.y - 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y - 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
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
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical upper left
                            if position.x > 0 && position.y < 6 {
                                let potential = Position {x: position.x - 1, y: position.y + 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical upper right
                            if position.x < 7 && position.y < 6 {
                                let potential = Position {x: position.x + 1, y: position.y + 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //horizontal upper right
                            if position.x < 6 && position.y < 7 {
                                let potential = Position {x: position.x + 2, y: position.y + 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //horizontal lower right
                            if position.x < 6 && position.y > 0 {
                                let potential = Position {x: position.x + 2, y: position.y - 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical lower right
                            if position.x < 7 && position.y > 1 {
                                let potential = Position {x: position.x + 1, y: position.y - 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical lower left
                            if position.x > 0 && position.y > 1 {
                                let potential = Position {x: position.x - 1, y: position.y - 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //horizontal lower left
                            if position.x > 1 && position.y > 0 {
                                let potential = Position {x: position.x - 2, y: position.y - 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                        }
                        //bishops
                        11 | 14 => {
                            //up and right
                            if position.x < 7 && position.y < 7
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y + 1 };
                                while temp.x <= 7 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y + 1 };
                                }
                            }
                            //upper left
                            if position.x != 0 && position.y < 7
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y + 1 };
                                while temp.x != 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }

                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y + 1 };
                                }
                                //Copy paste this in respective spots
                                if temp.x == 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));

                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                }
                            }
                            //upper left edge
                            /*if position.x > 0 && position.y < 7 {
                                let mut temp = Position { x: 0, y: position.y + 1 };
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                    break;
                                }

                                //white piece is in the way
                                else {
                                    break;
                                }
                            }*/
                            //lower left
                            if position.x > 0 && position.y > 0
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y - 1 };
                                /*while temp.x > 0 && temp.y > 0 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y - 1 };
                                }*/
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 0 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y - 1 };

                                }

                            }
                            //lower right
                            if position.x < 7 && position.y > 0
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y - 1 };
                                /*while temp.x <= 7 && temp.y > 0 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y - 1 };
                                }*/
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 7 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y - 1 };

                                }


                                /*if (temp.x == 7 && temp.y == 0)
                                {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));

                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                }*/
                            }
                        }
                        //queen
                        12 => {
                            //combine bishop and rook logic
                            //up and right
                            if position.x < 7 && position.y < 7
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y + 1 };
                                while temp.x <= 7 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y + 1 };
                                }
                            }
                            //upper left
                            if position.x > 0 && position.y < 7
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y + 1 };
                                while temp.x > 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y + 1 };
                                }
                                if temp.x == 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));

                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                }
                            }
                            //lower left
                            if position.x > 0 && position.y > 0
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y - 1 };
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 0 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y - 1 };
                                }
                                if temp.x == 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));

                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                }
                            }
                            //lower right
                            if position.x < 7 && position.y > 0
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y - 1 };
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //white piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 7 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y - 1 };

                                }
                            }
                            //rook copy
                            if position.x > 0 {
                                let mut side = Position {x: position.x - 1, y: position.y};
                                //if square is empty, add to list
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x - 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //loop for right movement
                            if position.x < 7 {
                                let mut side = Position {x: position.x + 1, y: position.y};
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x + 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //upward movement
                            if position.y < 7 {
                                let mut side = Position {x: position.x, y: position.y + 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y + 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    //consider adding the &option<piece> to each moves.push
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //downward
                            if position.y > 0 {
                                let mut side = Position {x: position.x, y: position.y - 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y - 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                        }
                        //king
                        13 => {
                            //just like the other pieces, each possible move will be checked manually
                            //t-shaped movement first
                            if position.y < 7 {
                                let side = Position {x: position.x, y: position.y + 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: square.file, rank: square.rank + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));

                                }
                            }
                            if position.y > 0 {
                                let side = Position {x: position.x, y: position.y - 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: square.file, rank: square.rank - 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }

                            }
                            if position.x > 0 {
                                let side = Position {x: position.x - 1, y: position.y};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.x < 7 {
                                let side = Position {x: position.x + 1, y: position.y};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //x-shaped
                            if position.y > 0  && position.x > 0 {
                                let side = Position {x: position.x - 1, y: position.y - 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank - 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.y < 7 && position.x < 7 {
                                let side = Position {x: position.x + 1, y: position.y + 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.y > 0 && position.x < 7 {
                                let side = Position {x: position.x + 1, y: position.y - 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank - 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.y < 7 && position.x > 0 {
                                let side = Position {x: position.x - 1, y: position.y + 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::Black) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("side.x is < 8"), rank: square.rank + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                        }
                        _ => continue
                    }
                }
            possible.insert(piece.id, moves);
            }
        }
    }
    else {
        for (square, piece, position) in query.iter() {
            let mut moves: Vec<(Square, Position)> = Vec::new();
            if let Some(piece) = piece {
                if piece.team == Team::Black {
                    match piece.id {
                        //Black pawns
                        17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 => {

                            let front = Position {x: position.x, y: position.y - 1};
                            if piece_at_position(&query, &front) == None {
                                //NOTE PLEASE FIX THE SQUARE: IT IS NOT THE SAME AS FRONT: DO SOME ASCII MATH
                                moves.push((Square {file: char::from_u32(97 + front.x as u32).expect("will always be lowercase"), rank: position.y - 2, id: None }, front));
                            }
                            //As far as I am aware, there should be no reason a pawn is in this file without having made any moves
                            if position.y == 6 && piece_at_position(&query, &Position {x: position.x, y: position.y - 2}).is_none() {
                                //HERE TOO
                                moves.push((Square {file: char::from_u32(97 + position.x as u32).expect("will always be lowercase"), rank: position.y - 3, id: None }, Position{x: position.x, y: position.y - 2}));
                            }
                            /*right diagonal potential capture; must consider if a piece is on the
                            edge of the board*/
                            if position.x != 7 {
                                let enemy = Position {x: position.x + 1, y: position.y - 1};
                                if piece_at_position(&query, &enemy).map_or(false, |p| p.team == Team::White) {
                                    //DON'T MAKE THE SAME MISTAKE
                                    moves.push((Square {file: char::from_u32(96 + position.x as u32).expect("will always be lowercase"), rank: position.y + 2,  id: piece_at_position(&query, &enemy).map(|p| p.id)},  enemy));
                                }
                            }
                            // left diagonal
                            if position.x != 0 {
                                let enemy = Position {x: position.x - 1, y: position.y - 1};
                                if piece_at_position(&query, &enemy).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(98 + position.x as u32).expect("will always be lowercase"), rank: position.y + 2,  id: piece_at_position(&query, &enemy).map(|p| p.id)},  enemy));
                                }
                            }
                        }
                        //rooks
                        25 | 32 => {
                            if position.x > 0 {
                                let mut side = Position {x: position.x - 1, y: position.y};
                                //if square is empty, add to list
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x - 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //loop for right movement
                            if position.x < 7 {
                                let mut side = Position {x: position.x + 1, y: position.y};
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x + 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //upward movement
                            if position.y < 7 {
                                let mut side = Position {x: position.x, y: position.y + 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y + 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //downward
                            if position.y > 0 {
                                let mut side = Position {x: position.x, y: position.y - 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y - 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                        }
                        //knights
                        26 | 31 => {
                            //here on out, use positions instead of ranks and files
                            //would use a more efficient system, but I want to avoid corner cases screwing up code,
                            //so I will go L by L
                            //horizontal upper left
                            if position.x > 1 && position.y < 7 {
                                let potential = Position {x: position.x - 2, y: position.y + 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical upper left
                            if position.x > 0 && position.y < 6 {
                                let potential = Position {x: position.x - 1, y: position.y + 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical upper right
                            if position.x < 7 && position.y < 6 {
                                let potential = Position {x: position.x + 1, y: position.y + 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //horizontal upper right
                            if position.x < 6 && position.y < 7 {
                                let potential = Position {x: position.x + 2, y: position.y + 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //horizontal lower right
                            if position.x < 6 && position.y > 0 {
                                let potential = Position {x: position.x + 2, y: position.y - 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical lower right
                            if position.x < 7 && position.y > 1 {
                                let potential = Position {x: position.x + 1, y: position.y - 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //vertical lower left
                            if position.x > 0 && position.y > 1 {
                                let potential = Position {x: position.x - 1, y: position.y - 2};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                            //horizontal lower left
                            if position.x > 1 && position.y > 0 {
                                let potential = Position {x: position.x - 2, y: position.y - 1};
                                if piece_at_position(&query, &potential) == None ||
                                    piece_at_position(&query, &potential).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + potential.x as u32).expect("will always be lowercase"), rank: potential.y + 1,  id: piece_at_position(&query, &potential).map(|p| p.id)},  potential));
                                }
                            }
                        }
                        //bishops
                        27 | 30 => {
                            //up and right
                            if position.x < 7 && position.y < 7
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y + 1 };
                                while temp.x <= 7 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y + 1 };
                                }
                            }
                            //upper left
                            if position.x != 0 && position.y < 7
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y + 1 };
                                while temp.x != 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }


                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y + 1 };
                                }
                                //Copy paste this in respective spots
                                if temp.x == 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));


                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                }
                            }
                            //upper left edge
                            /*if position.x > 0 && position.y < 7 {
                                let mut temp = Position { x: 0, y: position.y + 1 };
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                    break;
                                }


                                //Black piece is in the way
                                else {
                                    break;
                                }
                            }*/
                            //lower left
                            if position.x > 0 && position.y > 0
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y - 1 };
                                /*while temp.x > 0 && temp.y > 0 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y - 1 };
                                }*/
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 0 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y - 1 };


                                }
                                //little more complicated here, need cases for the corner and bottom and left edge
                                /*if (temp.x == 0 && temp.y == 0)  //|| (temp.x > 0 && temp.y == 0)
                                    {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));


                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                }*/
                                /*if temp.x <= 7 && temp.y == 0 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));


                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                }*/
                            }
                            //lower right
                            if position.x < 7 && position.y > 0
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y - 1 };
                                /*while temp.x <= 7 && temp.y > 0 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y - 1 };
                                }*/
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 7 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y - 1 };


                                }




                                /*if (temp.x == 7 && temp.y == 0)
                                {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));


                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                }*/
                            }
                        }
                        //queen
                        30 => {
                            //combine bishop and rook logic
                            //up and right
                            if position.x < 7 && position.y < 7
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y + 1 };
                                while temp.x <= 7 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y + 1 };
                                }
                            }
                            //upper left
                            if position.x > 0 && position.y < 7
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y + 1 };
                                while temp.x > 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y + 1 };
                                }
                                if temp.x == 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));


                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                }
                            }
                            //lower left
                            if position.x > 0 && position.y > 0
                            {
                                let mut temp = Position { x: position.x - 1, y: position.y - 1 };
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 0 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x - 1, y: temp.y - 1 };
                                }
                                if temp.x == 0 && temp.y <= 7 {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));


                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                }
                            }
                            //lower right
                            if position.x < 7 && position.y > 0
                            {
                                let mut temp = Position { x: position.x + 1, y: position.y - 1 };
                                loop {
                                    if piece_at_position(&query, &temp) == None {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None }, temp));
                                    } else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::White) {
                                        moves.push((Square { file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id) }, temp));
                                        break;
                                    }
                                    //Black piece is in the way
                                    else {
                                        break;
                                    }
                                    if temp.x == 7 || temp.y == 0 {
                                        break;
                                    }
                                    temp = Position { x: temp.x + 1, y: temp.y - 1 };


                                }
                            }
                            //rook copy
                            if position.x > 0 {
                                let mut side = Position {x: position.x - 1, y: position.y};
                                //if square is empty, add to list
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x - 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //loop for right movement
                            if position.x < 7 {
                                let mut side = Position {x: position.x + 1, y: position.y};
                                while piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: None},  side));
                                    if side.x == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x + 1, y: side.y};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //upward movement
                            if position.y < 7 {
                                let mut side = Position {x: position.x, y: position.y + 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 7 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y + 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    //consider adding the &option<piece> to each moves.push
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //downward
                            if position.y > 0 {
                                let mut side = Position {x: position.x, y: position.y - 1};
                                while piece_at_position(&query, &side) == None {
                                    //y is + 1 because side is position based
                                    moves.push((Square {file: square.file, rank: side.y + 1,  id: None},  side));
                                    if side.y == 0 {
                                        break;
                                    }
                                    side = Position {x: side.x, y: side.y - 1};
                                }
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) {
                                    moves.push((Square {file: square.file, rank: side.y,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                        }
                        //king
                        31 => {
                            //just like the other pieces, each possible move will be checked manually
                            //t-shaped movement first
                            if position.y < 7 {
                                let side = Position {x: position.x, y: position.y + 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: square.file, rank: square.rank + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));


                                }
                            }
                            if position.y > 0 {
                                let side = Position {x: position.x, y: position.y - 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: square.file, rank: square.rank - 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }


                            }
                            if position.x > 0 {
                                let side = Position {x: position.x - 1, y: position.y};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.x < 7 {
                                let side = Position {x: position.x + 1, y: position.y};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            //x-shaped
                            if position.y > 0  && position.x > 0 {
                                let side = Position {x: position.x - 1, y: position.y - 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank - 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.y < 7 && position.x < 7 {
                                let side = Position {x: position.x + 1, y: position.y + 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.y > 0 && position.x < 7 {
                                let side = Position {x: position.x + 1, y: position.y - 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("will always be lowercase"), rank: square.rank - 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                            if position.y < 7 && position.x > 0 {
                                let side = Position {x: position.x - 1, y: position.y + 1};
                                if piece_at_position(&query, &side).map_or(false, |p| p.team == Team::White) ||
                                    piece_at_position(&query, &side) == None {
                                    moves.push((Square {file: char::from_u32(97 + side.x as u32).expect("side.x is < 8"), rank: square.rank + 1,  id: piece_at_position(&query, &side).map(|p| p.id)},  side));
                                }
                            }
                        }
                        _ => continue
                    }
                }
                possible.insert(piece.id, moves);
            }
        }
    }


    possible
}



#[derive(Component, Clone, Eq, Hash, PartialEq)]
enum MoveType {
    Move,
    Capture,
    Castle,
}

//I get the hashmap from possible_moves, use commands to spawn in the moves when clicked, asset_server for the pngs, and query to give me the pieces in the ecs itself
//the two spaces forward move is already taken care of, as well as potential pawn captures. I need to differentiate between captures and non-captures now
//Reminder: Possible moves tracks the square and if there is already a piece on it

//rework to output a hashmap similar to possible, but smaller if there is a check present

//unable to use a clone of a query due to the way its accessed;
//Container just for simulating moves
#[derive(Clone)]
struct SimSquare {
    square: Square,
    piece: Option<Piece>,
    pos: Position,
}

pub fn legal_moves(possible: HashMap<usize, Vec<(Square, Position)>>, game: &Game, board_snapshot_q: Query<(&Square, &Piece, &Position)>) -> HashMap<usize, Vec<(Square, Position)>> {
    //first, we need to find out if we are in check. If any move involves capturing the black king, then it is in check. It does not account for castling; it needs to be added later
    if check(&possible, &game) {
        //copy of the board
        let mut new_possible = HashMap::new();
        let test: Vec<SimSquare> = board_snapshot_q.iter().map(|(square, piece, pos)| SimSquare { square: square.clone(), piece: Some(piece.clone()), pos: pos.clone() }).collect();
        //go through each possible move, on each move, create a temp = to the new possible_moves, rerun check, and then add to list. If list is empty, then game is over
        for (id, move_list) in possible.iter() {
            let mut maybest: Vec<(Square, Position)> = Vec::new();
            for (sq, ps) in move_list {
                let mut temp = test.clone();
                for simulation in temp.iter_mut() {
                    if *id == simulation.piece.clone().unwrap().id {
                        *simulation = SimSquare { square: sq.clone(), piece: Some(simulation.piece.clone().unwrap()), pos: *ps};

                        let temp_simulation: Vec<(Square, Option<Piece>, Position)> = temp.iter().map(|s|(s.square.clone(), Some(s.piece.clone().unwrap()), s.pos.clone())).collect();
                        let temp_possible = possible_moves(game.turn.clone(), temp_simulation);
                        //move is pointless
                        if check(&temp_possible, &game) {}
                        else {
                            maybest.push((sq.clone(), *ps));
                        }
                        //breaks regardless to reset temp
                        break;
                    }
                }
            }
            if maybest.is_empty() {
                continue;
            }
             else {
                 new_possible.insert(id.clone(), maybest);
             }
        }

        new_possible

    }

    else {
        possible

    }

}

//I'm making the check function first
pub fn check(possible: &HashMap<usize, Vec<(Square, Position)>>, game: &Game) -> bool {
    for n in possible.values() {
        for (sq,_pos) in n {
            if game.turn == Team::White {
                if sq.id == Some(13) { return true; }
            }
            else {
                if sq.id == Some(29) { return true;}
            }
        }
    }

    return false;

}

//In order to be able to actually click pieces and see possible moves, I need a new struct
//default potential move marker
#[derive(Component)]
pub struct Marker;

#[derive(Resource, Default)]
pub struct Selected {
    piece: Option<Entity>,
}

//piece and its destination
#[derive(Component)]
pub struct MoveTarget {
    from: Entity,
    to: Position,
}

//function to select pieces
pub fn select_piece_system (mut selected: ResMut<Selected>, windows: Query<&Window>, camera_q: Query<(&Camera, &GlobalTransform)>,
piece_query: Query<(Entity, &Piece, &Position)>, mouse_buttons: Res<Input<MouseButton>>,
) {
    //if left click, then find the respective position on the board
    let window = windows.single();
    if mouse_buttons.just_pressed(MouseButton::Left) {
        let Some(cursor_pos) = window.cursor_position() else {return};
        let (camera, cam_transform) = camera_q.single();
        let world_pos = camera.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();
       // const TILE_SIZE: f32 = 90.0;
        //const BOARD_SIZE: usize = 8;

        let half = TILE_SIZE * (BOARD_SIZE as f32) / 2.0;
        let x = ((world_pos.x + half) / TILE_SIZE.floor()) as isize;
        let y = ((world_pos.y + half) / TILE_SIZE.floor()) as isize;

        if (0..8).contains(&x) && (0..8).contains(&y) {
            let board_pos = Position {x: x as usize, y: y as usize};
            for (entity, _piece, pos) in piece_query.iter() {
                if *pos == board_pos {
                    selected.piece = Some(entity);
                    return;
                }
            }
            //if square is empty
            selected.piece = None;
        }
    }

}

pub fn highlight_moves_system (mut commands: Commands, selected: Res<Selected>, mut cleanup: Query<Entity, With<Marker>>, piece_query: Query<(Entity, &Piece, &Position)>,
board_snapshot_query: Query<(&Square, &Piece, &Position)>, asset_server: Res<AssetServer>, game: Res<Game>) {
    //const TILE_SIZE: f32 = 90.0;
    //const BOARD_SIZE: usize = 8;
    for entity in cleanup.iter() {
        commands.entity(entity).despawn();
    }

    let selected_entity = match selected.piece {
        Some(entity) => entity,
        None => return
    };

    let (from_entity, piece, from_pos) = match piece_query.get(selected_entity) {
        Ok(tuple) => tuple,
        Err(_) => return,
    };

    //vector snapshot to be filled by possible moves
    let board_vec: Vec<(Square, Option<Piece>, Position)> = board_snapshot_query.iter()
        .map(|(_entity, maybe_piece, pos)| { (Square::new((b'a' + pos.x as u8) as char, pos.y + 1, Some(maybe_piece.id)), Some(maybe_piece.clone()), pos.clone()) }).collect();

    //if this is empty, then game is over
    let possible = possible_moves(game.turn.clone(), board_vec);
    let real_moves = legal_moves(possible, &game, board_snapshot_query);



    if let Some(moves) = real_moves.get(&piece.id) {
        for (square, destination_pos) in moves {
            let world_x = (destination_pos.x as f32) * TILE_SIZE - (TILE_SIZE * 8.0 / 2.0) + TILE_SIZE / 2.0;
            let world_y = (destination_pos.y as f32) * TILE_SIZE - (TILE_SIZE * 8.0 / 2.0) + TILE_SIZE / 2.0;


            let mut legal = "";
            //bool to see if the potential move spawns a black or white square, and capture or legal
            if (destination_pos.x + destination_pos.y) % 2 == 0 {
                if square.id.is_some() {
                    legal = "board\\white_capture.png";
                }
                else {
                    legal = "board\\white_legal.png";
                }
            }
            else {
                if square.id.is_some() {
                    legal = "board\\black_capture.png";
                }
                else {
                    legal = "board\\black_legal.png";
                }
            }
            let potential = asset_server.load(legal);
            commands.spawn((
                Marker,
                MoveTarget { from: from_entity, to: *destination_pos },
                SpriteBundle {
                    //NOTE: I want this to be a circle that I have in my assets folder rather than the green highlight
                    /*sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.5), // translucent highlight
                        custom_size: Some(Vec2::splat(TILE_SIZE * 0.6)),
                        ..default()
                    },*/
                    texture: potential,
                    transform: Transform::from_xyz(world_x, world_y, 0.5),
                    ..default()
                },
            ));
        }
    }
}

pub fn commit_move_system(mut commands: Commands, windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>, mut piece_q: Query<(&mut Position, &mut Transform, &Piece, Entity)>,
  marker_q: Query<(&MoveTarget, &Transform, Entity), (With<Marker>, Without<Piece>)>, mut selected: ResMut<Selected>,
  mouse_buttons: Res<Input<MouseButton>>, board_snapshot_q: Query<(&mut Square, &mut Piece, &mut Position, Entity,), Without<Transform>>,
  mut game: ResMut<Game>
) {
    let window = windows.single();
    if mouse_buttons.just_pressed(MouseButton::Left) {
        //default value
        let mut move_t = Position { x: 0, y: 0 };
        //copy and pasted from select piece system
        let Some(cursor_pos) = window.cursor_position() else {return};
        let (camera, cam_transform) = camera_q.single();
        let world_pos = camera.viewport_to_world_2d(cam_transform, cursor_pos).unwrap();

        let half = TILE_SIZE * (BOARD_SIZE as f32) / 2.0;
        let x1 = ((world_pos.x + half) / TILE_SIZE.floor()) as isize;
        let y1 = ((world_pos.y + half) / TILE_SIZE.floor()) as isize;

        let actual_move = Position {x: x1 as usize, y: y1 as usize };


        for (move_target, marker_transform, marker_entity) in marker_q.iter() {

            //modify so that mouseclick must equal move_target to, and the first option is not picked
            if actual_move.x != move_target.to.x || actual_move.y != move_target.to.y {
                continue;
            }
            let from = move_target.from;
            let to = move_target.to;



            for (pos, _trans, pc, ent) in piece_q.iter() {
                if pos.x == to.x && pos.y == to.y {
                    if pc.team != game.turn {
                        commands.entity(ent).despawn();
                        info!("Captured piece at {:?}, {:?}, {:?}", pos.x, pos.y, pc.id );
                        //*pc.captured = true;
                        //run material captured function
                    }
                }
            }
            //updates piece
            if let Ok((mut pos, mut transform, _piece, entity)) = piece_q.get_mut(from) {
                pos.x = to.x;
                pos.y = to.y;
                let half = TILE_SIZE * (8.0 / 2.0);
                transform.translation.x = (to.x as f32) * TILE_SIZE - half + TILE_SIZE / 2.0;
                transform.translation.y = (to.y as f32) * TILE_SIZE - half + TILE_SIZE / 2.0;
            }
            //Once piece is updated, if it is a capture, captured piece needs to despawn
            //added board snapshot_query

            selected.piece = None;
            game.next_turn();
            break;
        }
    }
}

