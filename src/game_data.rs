use bevy::prelude::*;

use crate::{
    Materials,
    paddle::Paddle,
    ball::Ball,
    popup::Popup,
    brick::Brick,
    level::spawn_level,
};

pub enum LevelFinishedEvent {
    Success,
    Failure,
}
#[derive(PartialEq)]
pub enum GameState {
    Start,
    Play,
}
pub struct GameData {
    pub lives: u16,
    pub level: u16,
    pub state: GameState,
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            lives: 3,
            level: 1,
            state: GameState::Start,
        }
    }
}

pub fn start_level(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut game_data: ResMut<GameData>,
) {
    if game_data.state != GameState::Start {
        return;
    }

    if input.pressed(KeyCode::Space) {
        spawn_level(&mut commands, &materials, &game_data);
        game_data.state = GameState::Play;
    }
}

// Despawn everything and show Game Over text
pub fn level_finished(
    mut commands: Commands,
    mut reader: Local<EventReader<LevelFinishedEvent>>,
    game_over_events: Res<Events<LevelFinishedEvent>>,
    mut game_data: ResMut<GameData>,
    paddle: Query<With<Paddle, Entity>>,
    balls: Query<With<Ball, Entity>>,
    popups: Query<With<Popup, Entity>>,
    bricks: Query<With<Brick, Entity>>,
) {
    if let Some(event) = reader.iter(&game_over_events).next() {
        for ent in paddle.iter().chain(balls.iter()).chain(popups.iter()).chain(bricks.iter()) {
            commands.despawn(ent);
        }
        if let LevelFinishedEvent::Success = event {
            game_data.level += 1;
        }
        game_data.lives = 3;
        game_data.state = GameState::Start;
    }
}