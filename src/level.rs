use serde::{Deserialize};
use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    type_registry::TypeUuid,
    utils::BoxedFuture,
};

use crate::{
    Materials,
    ball::spawn_ball,
    brick::{spawn_brick, BRICK_SIZE_X},
    paddle::spawn_paddle,
};

fn get_correct_brick_mat (hp: u32, materials: &Materials) -> Handle<ColorMaterial> {
    match hp {
        1 => materials.brick_material.clone(),
        2 => materials.brick_2_material.clone(),
        _ => materials.brick_material.clone(),
    }
}

// This is not a system
pub fn spawn_level (
    mut commands: &mut Commands,
    level_config: &LevelConfig,
    materials: &Materials,
) {
    for ball in &level_config.balls {
        spawn_ball(&mut commands, &materials, ball.speed, ball.size_mult);
    }

    for brick in &level_config.bricks {
        let material = get_correct_brick_mat(brick.hp, &materials);
        spawn_brick(&mut commands, material, brick.x, brick.y, brick.hp as u16, brick.size_mult);
    }

    for brick_row in &level_config.brick_rows {
        for i in 0..brick_row.brick_count {
            let material = get_correct_brick_mat(brick_row.brick_hp, &materials);
            let x = brick_row.x_left + i as f32 * BRICK_SIZE_X * brick_row.brick_size_mult + i as f32* brick_row.brick_interval;
            spawn_brick(&mut commands, material, x, brick_row.y_top, brick_row.brick_hp as u16, brick_row.brick_size_mult);
        }
    }
    
    spawn_paddle(&mut commands, &materials);
}
#[derive(Debug, Deserialize)]
struct BrickConfig {
    x: f32,
    y: f32,
    size_mult: f32,
    hp: u32,
}
impl Default for BrickConfig {
    fn default() -> Self {
        BrickConfig {
            x: 0.,
            y: 0.,
            size_mult: 1.,
            hp: 1
        }
    }
}
#[derive(Debug, Deserialize)]
struct BallConfig {
    speed: f32,
    size_mult: f32,
}
impl Default for BallConfig {
    fn default() -> Self {
        BallConfig {
            speed: 1.,
            size_mult: 1.,
        }
    }
}
#[derive(Debug, Deserialize)]
struct BrickRow {
    x_left: f32,
    y_top: f32,
    brick_count: u32,
    brick_interval: f32,
    brick_size_mult: f32,
    brick_hp: u32,
}
impl Default for BrickRow {
    fn default() -> Self {
        BrickRow {
            x_left: 100.,
            y_top: 100.,
            brick_count: 5,
            brick_interval: 10.,
            brick_size_mult: 1.,
            brick_hp: 1,
        }
    }
}
#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
pub struct LevelConfig {
    brick_rows: Vec<BrickRow>,
    bricks: Vec<BrickConfig>,
    balls: Vec<BallConfig>,
}
#[derive(Default)]
pub struct LevelConfigLoader;

impl AssetLoader for LevelConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = serde_json::from_slice::<LevelConfig>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}