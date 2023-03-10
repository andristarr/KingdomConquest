mod console_utils;
pub mod game_state;
mod player_decision;

use console_utils::{clear_console, print_current_state};
use game_state::kingdom::Kingdom;
use game_state::GameState;
use player_decision::PlayerDecision;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::io;

use self::console_utils::prompt_for_input;

pub fn run_game() {
    let mut game = generate_game(5);

    while game.current_round < game.game_length {
        advance_round(&mut game)
    }
}

fn generate_game(num_of_kingdoms: u32) -> GameState {
    let mut game_state = GameState::new(8);

    let mut rng = rand::thread_rng();

    for _i in 0..num_of_kingdoms {
        game_state.kingdoms.push(create_kingdom(&mut rng))
    }

    return game_state;
}

fn get_action() -> PlayerDecision {
    loop {
        let mut decision = String::new();

        io::stdin()
            .read_line(&mut decision)
            .expect("Read line error");

        let decision: PlayerDecision = match decision.try_into() {
            Ok(val) => val,
            Err(_) => continue,
        };

        return decision;
    }
}

fn advance_round(game_state: &mut GameState) {
    game_state.current_round += 1;
    clear_console();
    print_current_state(game_state);
    prompt_for_input();
    let action = get_action();

    let kingdom = game_state.kingdoms.first_mut().unwrap();

    match action {
        PlayerDecision::Build => build_cities(kingdom),
        PlayerDecision::Mine => mine_gold(kingdom),
        PlayerDecision::Attack => try_attack(game_state),
    };
}

fn build_cities(kingdom: &mut Kingdom) {
    let mut rng = rand::thread_rng();

    kingdom.cities += rng.gen_range(1..3);
    kingdom.soldiers += rng.gen_range(10..50);
}

fn mine_gold(kingdom: &mut Kingdom) {
    let mut rng = rand::thread_rng();

    kingdom.gold += rng.gen_range(50..250);
}

fn try_attack(game_state: &mut GameState) {
    let mut rng = rand::thread_rng();
    let length = game_state.kingdoms.len();

    let (player, remainder) = game_state.kingdoms.split_first_mut().unwrap();

    let target = remainder.get(rng.gen_range(1..length) as usize).unwrap();

    if target.soldiers < player.soldiers || target.gold < player.gold {
        player.gold += (target.gold as f64 * 0.1) as u32;
        player.soldiers += (target.soldiers as f64 * 0.05) as u32;
    }
}

fn create_kingdom(rng: &mut ThreadRng) -> Kingdom {
    Kingdom {
        cities: rng.gen_range(1..10),
        gold: rng.gen_range(100..1000),
        soldiers: rng.gen_range(50..200),
    }
}
