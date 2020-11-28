#![warn(clippy::complexity)]
use bevy::prelude::*;
use bevy::render::pass::ClearColor;

mod paddle;
mod velocity;
mod ball;
mod wall;
mod brick;
mod collider;
mod powerup;
mod modifiers;
mod game_data;
mod text;
mod level;
mod music;

use velocity::{apply_velocity};
use paddle::{paddle_movement};
use ball::{handle_fallen_down};
use wall::{spawn_walls};
use powerup::{spawn_powerup};
use collider::{ball_collisions, paddle_collisions,BallHitEvent};
use modifiers::{apply_modifiers};
use game_data::{GameData,LevelFinishedEvent,LifeLostEvent,level_finished,start_level,life_lost};
use text::{UIPlugin};
use brick::{handle_destroyable_hit};
use level::{LevelConfig,LevelConfigLoader};
use music::{SoundPlugin};

const WINDOW_WIDTH: f32 = 1280.;// 1366.;
const WINDOW_HEIGHT: f32 = 720.; //768.;

const SPEED_COLOR: (f32, f32, f32) = (0.1, 0.9, 0.1);
const SIZE_COLOR: (f32, f32, f32) = (0.9, 0.1, 0.9);

#[derive(Clone)]
pub struct Materials {
    paddle_material: Handle<ColorMaterial>,
    ball_material: Handle<ColorMaterial>,
    brick_material: Handle<ColorMaterial>,
    brick_2_material: Handle<ColorMaterial>,
    wall_material: Handle<ColorMaterial>,
    powerup_material_speed: Handle<ColorMaterial>,
    powerup_material_size: Handle<ColorMaterial>,
    powerup_material_bomb: Handle<ColorMaterial>,
    background_material: Handle<ColorMaterial>,
    sound_button: Handle<ColorMaterial>,
}

pub struct Levels {
    level_1: Handle<LevelConfig>,
    level_2: Handle<LevelConfig>,
    level_3: Handle<LevelConfig>
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dComponents::default());
    let materials_res = Materials {
        paddle_material: materials.add(asset_server.load("materials/paddle.png").into()),
        ball_material: materials.add(asset_server.load("materials/ball.png").into()),
        brick_material: materials.add(asset_server.load("materials/bricks/brick_1.png").into()),
        brick_2_material: materials.add(asset_server.load("materials/bricks/brick_2.png").into()),
        wall_material: materials.add(Color::rgb(0.1, 0.1, 0.1).into()),
        powerup_material_speed: materials.add(asset_server.load("materials/power-ups/speed.png").into()),
        powerup_material_size: materials.add(asset_server.load("materials/power-ups/size.png").into()),
        powerup_material_bomb: materials.add(asset_server.load("materials/power-ups/bomb.png").into()),
        background_material: materials.add(Color::rgb(0.04, 0.04, 0.04).into()),
        sound_button: materials.add(asset_server.load("materials/music_on.png").into()),
    };
    commands.insert_resource(materials_res);

    commands.insert_resource(Levels {
        level_1: asset_server.load("levels/level_1.json"),
        level_2: asset_server.load("levels/level_2.json"),
        level_3: asset_server.load("levels/level_3.json"),
    });
    
    println!("Levels resource inserted");
    
}
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(GameData::default())
        .add_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_asset::<LevelConfig>()
        .init_asset_loader::<LevelConfigLoader>()
        
        .add_startup_stage("setup")
        .add_startup_system_to_stage("setup", setup.system())

        // "spawn" stage is used after Materials are loaded in the "setup"
        .add_startup_stage("spawn")
        .add_startup_system_to_stage("spawn", spawn_walls.system())
        .add_plugin(UIPlugin)
        .add_plugin(SoundPlugin)
        .add_system(apply_velocity.system())
        .add_system(paddle_movement.system())
        .add_system(ball_collisions.system())
        .add_system(paddle_collisions.system())
        .add_system(handle_fallen_down.system())
        .add_system(spawn_powerup.system())
        .add_system(apply_modifiers.system())
        .add_system(level_finished.system())
        .add_system(start_level.system())
        .add_system(handle_destroyable_hit.system())
        .add_system(life_lost.system())
        .add_event::<LevelFinishedEvent>()
        .add_event::<LifeLostEvent>()
        .add_event::<BallHitEvent>()
        .run();
        
}
