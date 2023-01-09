#![allow(dead_code, unused_variables)]

use rusty_engine::prelude::*;

const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

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
    let blue_car = engine.add_sprite("player_1", SpritePreset::RacingCarBlue);
    blue_car.translation = Vec2::new(-150.0, -200.0);
    blue_car.rotation = UP;
    blue_car.scale = 1.3;
    blue_car.layer = CHARACTER_LAYER;
    println!("Logical...")
}
