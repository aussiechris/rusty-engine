#![allow(dead_code, unused_variables)]

use rusty_engine::prelude::*;

struct GameState {
    health_left: i32,
}

fn main() {
    let mut game = Game::new();
    let game_state = GameState { health_left: 42 };
    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    println!("Logical...")
}
