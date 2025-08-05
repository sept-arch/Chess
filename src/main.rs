use bevy::prelude::*;
mod game;
use crate::game::*;
use Chess::*;

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
        .insert_resource(Selected::default())
        .insert_resource(Game::new())
        .add_systems(Update, (select_piece_system, highlight_moves_system.after(select_piece_system)))
        .run();
}



