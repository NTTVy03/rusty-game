use bevy::{scene::serde::ENTITY_FIELD_COMPONENTS, utils::label};
use bevy_ecs::event::event_update_condition;
use rusty_engine::{game, prelude::*};

// define a struct to store game-specific data
// high score, player name, health left,...
#[derive(Resource)]
struct GameState {
    high_score: u32,
    current_score: u32,
    enemy_labels: Vec<String>,
    spawn_time: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            high_score: 0,
            current_score: 0,
            enemy_labels: Vec::new(),
            spawn_time: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}
fn main() {
    let mut game = Game::new();

    // create sprites
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.collision = true;

    let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
    car1.translation = Vec2::new(300.0, 0.0);
    car1.collision = true;

    // register logic functions to run each frame
    game.add_logic(game_logic);

    game.run(GameState::default());
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        // println!("{:?}", event);

        // remove the sprite that the player collided with
        let collided_sprited = if event.pair.0 == "player" {
            event.pair.1
        } else {
            event.pair.0
        };
        engine.sprites.remove(&collided_sprited);

        // inscrease score for player
        if event.state == CollisionState::Begin {
            game_state.current_score += 1;
            println!("Current score: {}", game_state.current_score);
        }
    }

    // make player's car move right
    let player = engine.sprites.get_mut("player").unwrap();
    player.translation.x += 100.0 * engine.delta_f32;
}
