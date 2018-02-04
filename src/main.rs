extern crate rand;

pub mod core;

use core::game::Game;
use core::player::{Player, PlayerID};

fn main() {
    let mut players = (0..10).map(|i| {
        Player::new(PlayerID(i), i.to_string())
    }).collect();

    let mut game = Game::new(players).unwrap();
    game.start();

    for player in game.players {
        println!("{:?}", player.secret_role.unwrap());
    }
}
