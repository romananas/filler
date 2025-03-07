

use crate::parsing::AnField;

#[derive(Debug)]
pub struct GameState {
    pub anfield: AnField<'static>,
}

pub fn read_input(player_n :usize,input: &str) -> GameState {
    // use le parsing  de Romann
    let anfield = AnField::parse(player_n,&input);

    // return la structure complète qui servira aux autres modules
    GameState { anfield }
}

