use bevy::prelude::*;

use crate::{
    Materials,
    game_data::GameData,
    ball::spawn_ball,
    brick::spawn_bricks,
    paddle::spawn_paddle,
};

// This is not a system
pub fn spawn_level (
    mut commands: &mut Commands,
    materials: &Materials,
    game_data: &GameData,
) {
    spawn_ball(&mut commands, &materials, game_data.level);
    spawn_bricks(&mut commands, &materials, game_data.level);
    spawn_paddle(&mut commands, &materials);
}