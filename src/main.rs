#![warn(clippy::complexity)]
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod paddle;
mod velocity;
mod ball;
mod wall;
mod brick;
mod collider;
mod popup;
mod modifiers;
mod game_data;
mod text;
mod level;

use velocity::{apply_velocity};
use paddle::{paddle_movement};
use ball::{handle_fallen_down};
use wall::{spawn_walls};
use popup::{spawn_popup};
use collider::{ball_collisions, paddle_collisions};
use modifiers::{apply_modifiers};
use game_data::{GameData,LevelFinishedEvent,level_finished,start_level};
use text::{spawn_text,update_text,paused_text};
use brick::{handle_destroyable_hit,DestroyableHitEvent};

const WINDOW_WIDTH: f32 = 1366.;
const WINDOW_HEIGHT: f32 = 768.;

const SPEED_COLOR: (f32, f32, f32) = (0.1, 0.9, 0.1);
const SIZE_COLOR: (f32, f32, f32) = (0.9, 0.1, 0.9);

#[derive(Clone)]
pub struct Materials {
    paddle_material: Handle<ColorMaterial>,
    ball_material: Handle<ColorMaterial>,
    brick_material: Handle<ColorMaterial>,
    brick_2_material: Handle<ColorMaterial>,
    wall_material: Handle<ColorMaterial>,
    popup_material_speed: Handle<ColorMaterial>,
    popup_material_size: Handle<ColorMaterial>,
    background_material: Handle<ColorMaterial>,
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dComponents::default());
    commands.insert_resource(Materials {
        paddle_material: materials.add(asset_server.load("materials/paddle.png").into()),
        ball_material: materials.add(asset_server.load("materials/ball.png").into()),
        brick_material: materials.add(asset_server.load("materials/bricks/brick_1.png").into()),
        brick_2_material: materials.add(asset_server.load("materials/bricks/brick_2.png").into()),
        wall_material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        popup_material_speed: materials.add(asset_server.load("materials/power-ups/speed.png").into()),
        popup_material_size: materials.add(asset_server.load("materials/power-ups/size.png").into()),
        background_material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
    });
}
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(GameData::default())
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_resource(WindowDescriptor {
            title: "Pong Cursher!".to_string(),
            width: WINDOW_WIDTH as u32,
            height: WINDOW_HEIGHT as u32,
            ..Default::default()
        })
        .add_startup_stage("setup")
        .add_startup_system_to_stage("setup", setup.system())

        //.add_startup_system(setup.system())
        .add_startup_stage("spawn")
        .add_startup_system_to_stage("spawn", spawn_walls.system())
        .add_startup_system_to_stage("spawn", spawn_text.system())
    
        .add_system(apply_velocity.system())
        .add_system(paddle_movement.system())
        .add_system(ball_collisions.system())
        .add_system(paddle_collisions.system())
        .add_system(handle_fallen_down.system())
        .add_system(spawn_popup.system())
        .add_system(apply_modifiers.system())
        .add_system(level_finished.system())
        .add_system(update_text.system())
        .add_system(start_level.system())
        .add_system(handle_destroyable_hit.system())
        .add_system(paused_text.system())
        .add_event::<LevelFinishedEvent>()
        .add_event::<DestroyableHitEvent>()
        .run();
        
}
