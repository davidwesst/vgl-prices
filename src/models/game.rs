use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    id: u8,
    title: String
}

impl Game {
    pub fn new(id: u8, title: String) -> Game {
        return Game { id: id, title: title }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} ({})", self.title, self.id)
    }
}
