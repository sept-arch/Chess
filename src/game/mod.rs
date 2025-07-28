//see if it shows up in github
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