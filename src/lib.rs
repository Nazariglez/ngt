mod game;

use notan::prelude::*;

#[notan_main]
pub fn start_game() {
    game::initialize().unwrap();
}