mod game;
mod camera2d;

use notan::prelude::*;

#[notan_main]
pub fn start_game() {
    game::initialize().unwrap();
}
