pub mod kingdom;

use kingdom::Kingdom;

pub struct GameState {
    pub player: Kingdom,
    pub kingdoms: Vec<Kingdom>,
    pub game_length: u32,
    pub current_round: u32,
}

impl GameState {
    pub fn new(game_length: u32) -> GameState {
        GameState {
            player: Kingdom {
                cities: 0,
                gold: 0,
                soldiers: 0,
            },
            kingdoms: Vec::new(),
            game_length,
            current_round: 0,
        }
    }
}
