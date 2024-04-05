mod models;

use csv;
use models::game::Game;
use serde;
use std::{error::Error, io, process};

fn main() {
    let new_game = Game::new(1,String::from("Game Title"));
    println!("{}", new_game);

    let mut writer = csv::Writer::from_writer(io::stdout());
    writer.serialize(new_game);
    writer.flush();
}
