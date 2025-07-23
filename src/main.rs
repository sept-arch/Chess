use Chess::*;
use bevy::prelude::*;
//bevy is somewhat complicated but the best engine to use to have animations and move history

fn main() {
    //pseudocode setup:
    //run the game::new() function,
    //run game::run() which returns the winner
    //bring up game over screen declaring winner
    //option to play again, or exit application
    App::new().add_systems(Update, Game::new()).run();

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
}