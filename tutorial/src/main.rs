#![allow(dead_code, unused_variables)]

use rand::prelude::random;
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
    let _ = engine.add_sprite("player_1", SpritePreset::RacingCarBlue);
    let blue_car = engine.sprites.get_mut("player_1").unwrap();

    // place the blue car at a random position on screen
    blue_car.translation = Vec2::new(
        random::<f32>() * engine.window_dimensions.x - engine.window_dimensions.x / 2.0,
        random::<f32>() * engine.window_dimensions.y - engine.window_dimensions.y / 2.0,
    );
    blue_car.rotation = UP;
    blue_car.scale = random::<f32>() * 3.5;
    blue_car.layer = CHARACTER_LAYER;
    blue_car.collision = true;
    // engine.sprites.remove("player_1"); // delete sprite - will make it invisible if you uncomment

    let red_car = engine.add_sprite("player_2", SpritePreset::RacingCarRed);
    red_car.translation = Vec2::new(0.0, 0.0);
    red_car.rotation = UP;
    red_car.scale = 1.0;
    red_car.layer = CHARACTER_LAYER;
    red_car.collision = true;

    for event in engine.collision_events.drain(..) {
        match event.state {
            CollisionState::Begin => {
                println!("{} and {} collided!", event.pair.0, event.pair.1);
            }
            CollisionState::End => {
                println!(
                    "{} and {} are no longer colliding.",
                    event.pair.0, event.pair.1
                );
            }
        }
    }
}
