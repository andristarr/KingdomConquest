mod console_utils;
pub mod game_state;
mod player_decision;

use console_utils::{clear_console, print_current_state, print_decision, prompt_for_input};
use game_state::kingdom::Kingdom;
use game_state::GameState;
use player_decision::PlayerDecision;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::io;

use self::console_utils::{
    print_attack_result, print_build_result, print_mine_result, print_unsuccessful_attack,
};

pub fn run_game() {
    let mut game = generate_game(5);

    while game.current_round < game.game_length {
        advance_round(&mut game)
    }
}

fn generate_game(num_of_kingdoms: u32) -> GameState {
    let mut game_state = GameState::new(8);

    let mut rng = rand::thread_rng();

    game_state.player = create_kingdom(&mut rng);

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

    match action {
        PlayerDecision::Build => {
            print_decision(PlayerDecision::Build);
            build_cities(&mut game_state.player)
        }
        PlayerDecision::Mine => {
            print_decision(PlayerDecision::Mine);
            mine_gold(&mut game_state.player)
        }
        PlayerDecision::Attack => {
            print_decision(PlayerDecision::Attack);
            try_attack(game_state)
        }
    };
}

fn build_cities(kingdom: &mut Kingdom) {
    let mut rng = rand::thread_rng();

    let cities = rng.gen_range(1..3);
    let soldiers = rng.gen_range(10..50);

    kingdom.cities += cities;
    kingdom.soldiers += soldiers;

    print_build_result(cities, soldiers);
}

fn mine_gold(kingdom: &mut Kingdom) {
    let mut rng = rand::thread_rng();

    let gold = rng.gen_range(50..250);

    kingdom.gold += gold;

    print_mine_result(gold);
}

fn try_attack(game_state: &mut GameState) {
    let mut rng = rand::thread_rng();

    let player = &mut game_state.player;
    let target = game_state.kingdoms.choose(&mut rng).unwrap();

    if target.soldiers < player.soldiers || target.gold < player.gold {
        let loot = (target.gold as f64 * 0.1) as u32;
        let recruited = (target.soldiers as f64 * 0.05) as u32;
        player.gold += loot;
        player.soldiers += recruited;
        print_attack_result(loot, recruited);
    } else {
        print_unsuccessful_attack();
    }
}

fn create_kingdom(rng: &mut ThreadRng) -> Kingdom {
    Kingdom {
        cities: rng.gen_range(1..10),
        gold: rng.gen_range(100..1000),
        soldiers: rng.gen_range(50..200),
    }
}
