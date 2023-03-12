use crate::game_utils::game_state::GameState;

use super::player_decision::PlayerDecision;

pub fn clear_console() {
    print!("=================================================================");
    print!("\n");
}

pub fn print_current_state(game_state: &GameState) {
    println!(
        "[Round #{}/{}] Your kindom has the following stats\nCities: {}\nGold: {}\nSoldiers: {}",
        game_state.current_round,
        game_state.game_length,
        game_state.player.cities,
        game_state.player.gold,
        game_state.player.soldiers
    );
}

pub fn prompt_for_input() {
    println!("What would you like to do? ");
}

pub fn print_decision(decision: PlayerDecision) {
    match decision {
        PlayerDecision::Build => println!("You choose to build cities."),
        PlayerDecision::Mine => println!("You choose to mine some gold."),
        PlayerDecision::Attack => println!("You choose to attack a neighbouring kingdom."),
    }
}

pub fn print_build_result(built: u32, soldiers: u32) {
    println!(
        "You built {} new settlement(s) and recuited {} soldiers.",
        built, soldiers
    );
}

pub fn print_mine_result(gold: u32) {
    println!("You mined {} gold", gold);
}

pub fn print_attack_result(loot: u32, recruited: u32) {
    println!(
        "You looted {} gold and could 'persuade' {} soldiers to join your army instead.",
        loot, recruited
    );
}

pub fn print_unsuccessful_attack() {
    println!("The kingdom you attacked was stronger than you and repelled your advance.");
}
