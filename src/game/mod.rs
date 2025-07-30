//see if it shows up in GitHub
use crate::*;
use bevy::prelude::*;
use std::collections::HashMap;





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
                            let mut temp = Position {x: position.x + 1, y: position.y + 1};
                            while temp.x < 7 && temp.y < 7 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x + 1, y: temp.y + 1};
                            }
                            //upper left
                            temp = Position {x: position.x - 1, y: position.y + 1};
                            while temp.x > 0 && temp.y < 7 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x - 1, y: temp.y + 1};
                            }
                            //lower left
                            temp = Position {x: position.x - 1, y: position.y - 1};
                            while temp.x > 0 && temp.y > 0 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x - 1, y: temp.y - 1};
                            }
                            //lower right
                            temp = Position {x: position.x + 1, y: position.y - 1};
                            while temp.x < 7 && temp.y > 0 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x + 1, y: temp.y - 1};
                            }
                        }
                        //queen
                        12 => {
                            //combine bishop and rook logic
                            //up and right
                            let mut temp = Position {x: position.x + 1, y: position.y + 1};
                            while temp.x < 7 && temp.y < 7 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x + 1, y: temp.y + 1};
                            }
                            //upper left
                            temp = Position {x: position.x - 1, y: position.y + 1};
                            while temp.x > 0 && temp.y < 7 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x - 1, y: temp.y + 1};
                            }
                            //lower left
                            temp = Position {x: position.x - 1, y: position.y - 1};
                            while temp.x > 0 && temp.y > 0 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x - 1, y: temp.y - 1};
                            }
                            //lower right
                            temp = Position {x: position.x + 1, y: position.y - 1};
                            while temp.x < 7 && temp.y > 0 {
                                if piece_at_position(&query, &temp) == None {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: None}, temp));
                                }
                                else if piece_at_position(&query, &temp).map_or(false, |p| p.team == Team::Black) {
                                    moves.push((Square {file: char::from_u32(97 + temp.x as u32).expect("will always be lowercase"), rank: temp.y + 1, id: piece_at_position(&query, &temp).map(|p| p.id)}, temp));
                                    break;
                                }
                                //white piece is in the way
                                else {
                                    break;
                                }
                                temp = Position {x: temp.x + 1, y: temp.y - 1};
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
    possible
}

//default potential move marker
#[derive(Component)]
struct Marker;

#[derive(Component, Clone, Eq, Hash, PartialEq)]
enum MoveType {
    Move,
    Capture,
    Castle,
}

//I get the hashmap from possible_moves, use commands to spawn in the moves when clicked, asset_server for the pngs, and query to give me the pieces in the ecs itself
//the two spaces forward move is already taken care of, as well as potential pawn captures. I need to differentiate between captures and non-captures now
//Reminder: Possible moves tracks the square and if there is already a piece on it

pub fn legal_moves(possible: HashMap<usize, Vec<(Square, Position)>>, mut commands: Commands, asset_server: Res<AssetServer>, query: Query<(Entity, &Position), With<Piece>>, game: &Game) {
    //first, we need to find out if we are in check. If any move involves capturing the black king, then it is in check. It does not account for castling; it needs to be added later
    if check(&possible, &game) {
        let mut test = possible.clone();
    }

    else {
        //do the command.spawns and carry on as planned

    }

}

//I'm making the check function first
pub fn check(possible: &HashMap<usize, Vec<(Square, Position)>>, game: &Game) -> bool {
    for n in possible.values() {
        for (sq, pos) in n {
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