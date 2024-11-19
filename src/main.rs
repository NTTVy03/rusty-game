use rusty_engine::prelude::*;
use rand::Rng;

// define a struct to store game-specific data
// high score, player name, health left,...
#[derive(Resource)]
struct GameState {
    current_score: u32,
    ball_index: u32,
    spawn_time: Timer,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_score: 0,
            ball_index: 0,
            spawn_time: Timer::from_seconds(3.0, TimerMode::Repeating),
        }
    }
}
fn main() {
    let mut game = Game::new();

    // music
    game.audio_manager.play_music(MusicPreset::Classy8Bit, 0.2);

    // create sprites
    let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.collision = true;

    // create texts
    let title = game.add_text("title", "Rusty Game");
    title.font = "font/FiraMono-Medium.ttf".to_string();
    title.font_size = 96.0;
    title.translation = Vec2::new(400.0, -325.0);

    let score = game.add_text("score", "Score: {}");
    score.font = "font/FiraMono-Medium.ttf".to_string();
    score.font_size = 50.0;
    score.translation = Vec2::new(400.0, -225.0);

    // register logic functions to run each frame
    game.add_logic(manage_collisions);
    game.add_logic(player_movement);
    game.add_logic(add_balls_by_click);
    game.add_logic(add_balls_by_timer);
    game.add_logic(update_score_text);

    game.run(GameState::default());
}

fn manage_collisions(engine: &mut Engine, game_state: &mut GameState) {
    for event in engine.collision_events.drain(..) {
        // sound effect
        engine.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);

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

fn add_balls_by_click(engine: &mut Engine, game_state: &mut GameState) {
    // just_pressed: one click - one ball
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(location) = engine.mouse_state.location() {
            let ball_index = format!("ball{}", game_state.ball_index);
            let ball = engine.add_sprite(ball_index, SpritePreset::RollingBallRed);
            ball.translation = location;
            ball.collision = true;

            game_state.ball_index += 1;
        }
    }
}

fn add_balls_by_timer(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.spawn_time.tick(engine.delta).just_finished() {
        let ball_index = format!("ball{}", game_state.ball_index);
        let ball = engine.add_sprite(ball_index, SpritePreset::RollingBallRed);
        ball.translation.x = rand::thread_rng().gen_range(-550.0..550.0);
        ball.translation.y = rand::thread_rng().gen_range(-325.0..325.0);
        ball.collision = true;

        game_state.ball_index += 1;
    }
}

fn update_score_text(engine: &mut Engine, game_state: &mut GameState) {
    let score = engine.texts.get_mut("score").unwrap();
    score.value = format!("Score: {}", game_state.current_score);
}
