mod models;

use csv;
use models::game::Game;
use serde;

fn main() {
    let new_game = Game::new(1,String::from("Game Title"));
    println!("{}", new_game);
}
