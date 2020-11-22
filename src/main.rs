#![warn(clippy::complexity)]
use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

mod paddle;
mod velocity;
mod ball;
mod wall;
mod brick;
mod collider;
mod popup;
mod modifiers;

use velocity::{apply_velocity};
use paddle::{spawn_paddle, paddle_movement};
use ball::{spawn_ball, despawn_fallen};
use wall::{spawn_walls};
use brick::spawn_bricks;
use popup::{spawn_popup};
use collider::{ball_collisions, paddle_collisions};
use modifiers::{apply_modifiers};

const WINDOW_WIDTH: u32 = 1366;
const WINDOW_HEIGHT: u32 = 768;

#[derive(Clone)]
pub struct Materials {
    paddle_material: Handle<ColorMaterial>,
    ball_material: Handle<ColorMaterial>,
    brick_material: Handle<ColorMaterial>,
    wall_material: Handle<ColorMaterial>,
    popup_material_speed: Handle<ColorMaterial>,
    popup_material_size: Handle<ColorMaterial>,
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        paddle_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        ball_material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
        brick_material: materials.add(Color::rgb(0.9, 0.1, 0.1).into()),
        wall_material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        popup_material_speed: materials.add(Color::rgb(0.1, 0.9, 0.1).into()),
        popup_material_size: materials.add(Color::rgb(0.9, 0.1, 0.9).into()),
    });
}
fn main() {
    
    App::build()
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(WindowDescriptor {
            title: "Pong Cursher!".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            ..Default::default()
        })
        .add_startup_system(setup.system())
        .add_startup_stage("spawn_paddle")
        .add_startup_system_to_stage("spawn_paddle", spawn_paddle.system())
        .add_startup_stage("spawn_ball")
        .add_startup_system_to_stage("spawn_ball", spawn_ball.system())
        .add_startup_stage("spawn_brick")
        .add_startup_system_to_stage("spawn_brick", spawn_bricks.system())
        .add_startup_stage("spawn_wall")
        .add_startup_system_to_stage("spawn_wall", spawn_walls.system())
        .add_plugins(DefaultPlugins)
        // Doesn't do anything now
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(apply_velocity.system())
        .add_system(paddle_movement.system())
        .add_system(ball_collisions.system())
        .add_system(paddle_collisions.system())
        .add_system(despawn_fallen.system())
        .add_system(spawn_popup.system())
        .add_system(apply_modifiers.system())
        .run();
        
}
