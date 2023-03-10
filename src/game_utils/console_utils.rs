use crate::game_utils::game_state::GameState;

pub fn clear_console() {
    print!("{}[2J", 27 as char);
}

pub fn print_current_state(game_state: &mut GameState) {
    println!(
        "[Round #{}/{}] Your kindom has the following stats\nCities: {}\nGold: {}\nSoldiers: {}",
        game_state.current_round,
        game_state.game_length,
        game_state.kingdoms.first().take().unwrap().cities,
        game_state.kingdoms.first().take().unwrap().gold,
        game_state.kingdoms.first().take().unwrap().soldiers
    );
}

pub fn prompt_for_input() {
    println!("What would you like to do? ");
}
