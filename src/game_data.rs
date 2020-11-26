use bevy::prelude::*;

use crate::{
    Materials,
    paddle::Paddle,
    ball::Ball,
    powerup::Powerup,
    brick::Destroyable,
    level::spawn_level,
};

pub struct LifeLostEvent;
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

pub fn life_lost(
    mut reader: Local<EventReader<LifeLostEvent>>,
    life_lost_events: Res<Events<LifeLostEvent>>,
    mut level_finished_events: ResMut<Events<LevelFinishedEvent>>, 
    mut game_data: ResMut<GameData>,
) {
    if let Some(_event) = reader.iter(&life_lost_events).next() {
        game_data.lives -= 1;
        println!("Fallen down! Current lives: {}", game_data.lives);
        if game_data.lives == 0 {
            level_finished_events.send(LevelFinishedEvent::Lost)
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
    powerups: Query<With<Powerup, Entity>>,
    destroyables: Query<With<Destroyable, Entity>>,
) {
    if let Some(event) = reader.iter(&game_over_events).next() {
        for ent in paddle.iter().chain(balls.iter()).chain(powerups.iter()).chain(destroyables.iter()) {
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