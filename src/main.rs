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

    // Use Game instance to set up your game
    // and register logic functions to run each frame

    game.run(GameState::default());
}
