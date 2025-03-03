

use crate::parsing::AnField;
use std::io::{self, Read};

pub struct GameState {
    pub anfield: AnField<'static>,
}

pub fn read_input() -> GameState {
    // lis toute l'entrée 
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");

    // use le parsing  de Romann
    let anfield = AnField::parse(&input);

    // return la structure complète qui servira aux autres modules
    GameState { anfield }
}

