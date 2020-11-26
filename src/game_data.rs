use bevy::prelude::*;

use crate::{
    Materials,
    paddle::Paddle,
    ball::Ball,
    popup::Popup,
    brick::Destroyable,
    level::spawn_level,
};

pub enum LevelFinishedEvent {
    Won,
    Lost,
}
#[derive(PartialEq, Eq)]
pub enum PausedState {
    Start,
    Won,
    Lost,
}
#[derive(PartialEq, Eq)]
pub enum GameState {
    Paused(PausedState),
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
            state: GameState::Paused(PausedState::Start),
        }
    }
}

pub fn start_level(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    materials: Res<Materials>,
    mut game_data: ResMut<GameData>,
) {
    if let GameState::Paused(_) = game_data.state {
        if input.pressed(KeyCode::Space) {
            spawn_level(&mut commands, &materials, &game_data);
            game_data.state = GameState::Play;
        }
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
    destroyables: Query<With<Destroyable, Entity>>,
) {
    if let Some(event) = reader.iter(&game_over_events).next() {
        for ent in paddle.iter().chain(balls.iter()).chain(popups.iter()).chain(destroyables.iter()) {
            commands.despawn(ent);
        }
        game_data.lives = 3;
        if let LevelFinishedEvent::Won = event {
            game_data.level += 1;
            game_data.state = GameState::Paused(PausedState::Won);
        } else {
            game_data.state = GameState::Paused(PausedState::Lost);
            game_data.level = 1;
        }   
    }
}