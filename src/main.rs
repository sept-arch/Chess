use Chess::*;
use bevy::prelude::*;
const TILE_SIZE: f32 = 90.0;
const BOARD_SIZE: usize = 8;
const LIGHT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
const DARK_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);

fn main() {
    //pseudocode setup:
    //run the game::new() function,
    //run game::run() which returns the winner
    //bring up game over screen declaring winner
    //option to play again, or exit application
    //let winner : u8 = game::run();
    /*
    if winner == 0 {
        black won
    }

    if winner == 1 {
        white won
    }

    if winner == 2 {
        stalemate
    }


    */
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_board))
        .run();
}


fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle{
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
}

fn setup_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    let offset = TILE_SIZE * (BOARD_SIZE as f32) / 2.0;

    for row in 0..BOARD_SIZE {
        for col in 0..BOARD_SIZE {
            let is_light = (row + col) % 2 == 0;
            let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

            let x = (col as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
            let y = (row as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;

            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            });
        }
    }
    //pawns
    let white_pawn = asset_server.load("pieces/white_pawn.png");
    let black_pawn = asset_server.load("pieces/black_pawn.png");
    let y_white = 1f32 * TILE_SIZE - offset + TILE_SIZE / 2.0;
    let y_black = 6f32 * TILE_SIZE - offset + TILE_SIZE / 2.0;
    for col in 0..8 {
        let x = (col as f32) * TILE_SIZE - offset + TILE_SIZE / 2.0;
        commands.spawn(SpriteBundle {
            texture: white_pawn.clone(),
            transform: Transform::from_xyz(x, y_white, 0.0),
            ..default()
        });
        commands.spawn(SpriteBundle {
            texture: black_pawn.clone(),
            transform: Transform::from_xyz(x, y_black, 0.0),
            ..default()
        });
    }
    //rest of the pieces
    //rooks
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_rook.png"),
        transform: Transform::from_xyz(TILE_SIZE / 2.0 - offset, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_rook.png"),
        transform: Transform::from_xyz( 7f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_knight.png"),
        transform: Transform::from_xyz( 1f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_knight.png"),
        transform: Transform::from_xyz( 6f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_bishop.png"),
        transform: Transform::from_xyz( 2f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_bishop.png"),
        transform: Transform::from_xyz( 5f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_king.png"),
        transform: Transform::from_xyz( 3f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
    commands.spawn(SpriteBundle {
        texture: asset_server.load("pieces/white_queen.png"),
        transform: Transform::from_xyz( 4f32 * TILE_SIZE - offset + TILE_SIZE / 2.0, TILE_SIZE / 2.0 - offset, 0.0),
        ..default()
    });
}
