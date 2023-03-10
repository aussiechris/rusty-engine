#![allow(dead_code, unused_variables)]

use rand::prelude::random;
use rusty_engine::prelude::*;

const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

const MOVE_SPEED: f32 = 20.0;

struct GameState {
    health_left: i32,
    red_car_rotation: f32,
    red_car_translation: Vec2,
    red_car_scale: f32,

    blue_car_rotation: f32,
    blue_car_translation: Vec2,
    blue_car_scale: f32,
    blue_car_timer: Timer,
}
fn main() {
    let mut game = Game::new();
    let game_state = GameState {
        health_left: 42,
        red_car_rotation: UP,
        red_car_translation: Vec2::new(0.0, 0.0),
        red_car_scale: 1.0,

        blue_car_rotation: UP,
        blue_car_translation: Vec2::new(0.0, 0.0),
        blue_car_scale: 1.0,
        blue_car_timer: Timer::from_seconds(0.25, true),
    };
    game.add_logic(game_logic);
    game.run(game_state)
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let _ = engine.add_sprite("player_1", SpritePreset::RacingCarBlue);

    // add blue car
    let blue_car = engine.sprites.get_mut("player_1").unwrap();
    let window_width = engine.window_dimensions.x / 2.0;
    let window_height = engine.window_dimensions.y / 2.0;
    if game_state.blue_car_timer.tick(engine.delta).just_finished() {
        // place the blue car at a random position on screen
        game_state.blue_car_translation = Vec2::new(
            random::<f32>() * engine.window_dimensions.x - window_width,
            random::<f32>() * engine.window_dimensions.y - window_height,
        );
        game_state.blue_car_scale = random::<f32>() * 3.5;
    }

    blue_car.translation = game_state.blue_car_translation;
    blue_car.scale = game_state.blue_car_scale;
    blue_car.rotation = game_state.blue_car_rotation;
    blue_car.layer = CHARACTER_LAYER;
    blue_car.collision = true;

    // add red car {
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        game_state.red_car_rotation = RIGHT;
        if game_state.red_car_translation.x < window_width {
            game_state.red_car_translation.x += MOVE_SPEED;
        }
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        game_state.red_car_rotation = LEFT;
        if game_state.red_car_translation.x + window_width > 0.0 {
            game_state.red_car_translation.x -= MOVE_SPEED;
        }
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        game_state.red_car_rotation = UP;
        if game_state.red_car_translation.y < window_height {
            game_state.red_car_translation.y += MOVE_SPEED;
        }
    } else if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        game_state.red_car_rotation = DOWN;
        if game_state.red_car_translation.y + window_height > 0.0 {
            game_state.red_car_translation.y -= MOVE_SPEED;
        }
    }

    let red_car = engine.add_sprite("player_2", SpritePreset::RacingCarRed);

    red_car.translation = game_state.red_car_translation;
    red_car.rotation = game_state.red_car_rotation;
    red_car.scale = game_state.red_car_scale;
    red_car.layer = CHARACTER_LAYER;
    red_car.collision = true;

    // check for and manage collisions
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
