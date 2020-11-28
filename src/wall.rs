use bevy::prelude::*;

use crate::{WINDOW_WIDTH, WINDOW_HEIGHT, Materials, collider::Collider};

const WALL_THICKNESS: f32 = 10.;
const WALL_LEFT_X: f32 = WALL_THICKNESS / 2. - WINDOW_WIDTH / 2.;
const WALL_RIGHT_X: f32 = WINDOW_WIDTH / 2. - WALL_THICKNESS / 2.;
const WALL_TOP_Y: f32 = WINDOW_HEIGHT / 2. - WALL_THICKNESS / 2.;

pub fn spawn_walls(mut commands: Commands, materials: Res<Materials>) {
    commands
        // left
        .spawn(SpriteComponents {
            material: materials.wall_material.clone(),
            sprite: Sprite::new(Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(WALL_LEFT_X, 0., 0.)),
            ..Default::default()
        })
        .with(Collider::WallLeft)
        // right
        .spawn(SpriteComponents {
            material: materials.wall_material.clone(),
            sprite: Sprite::new(Vec2::new(WALL_THICKNESS, WINDOW_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(WALL_RIGHT_X, 0., 0.)),
            ..Default::default()
        })
        .with(Collider::WallRight)
        // top
        .spawn(SpriteComponents {
            material: materials.wall_material.clone(),
            sprite: Sprite::new(Vec2::new(WINDOW_WIDTH, WALL_THICKNESS)),
            transform: Transform::from_translation(Vec3::new(0., WALL_TOP_Y, 0.)),
            ..Default::default()
        })
        .with(Collider::WallTop);
}