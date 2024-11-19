use rusty_engine::prelude::*;

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

    let red_ball = game.add_sprite("red_ball", SpritePreset::RollingBallRed);
    red_ball.translation = Vec2::new(300.0, 0.0);
    red_ball.collision = true;

    // register logic functions to run each frame
    game.add_logic(manage_collisions);
    game.add_logic(player_movement);

    game.run(GameState::default());
}

fn manage_collisions(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
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
}

fn player_movement(engine: &mut Engine, game_state: &mut GameState) {
    let player = engine.sprites.get_mut("player").unwrap();
    const MOVEMENT_SPEED: f32 = 100.0;

    if engine.keyboard_state.pressed(KeyCode::Up) {
        player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
    } else if engine.keyboard_state.pressed(KeyCode::Down) {
        player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
    } else if engine.keyboard_state.pressed(KeyCode::Left) {
        player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
    } else if engine.keyboard_state.pressed(KeyCode::Right) {
        player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
    }
}
