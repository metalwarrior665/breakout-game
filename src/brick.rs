use bevy::prelude::*;

use crate::{collider::Collider, Materials};

const BRICK_SIZE_X: f32 = 100.;
const BRICK_SIZE_Y: f32 = 30.;
const BRICK_SPACING_X: f32 = 20.;
const BRICK_SPACING_Y: f32 = 20.;

pub fn spawn_bricks(
    mut commands: Commands,
    materials: Res<Materials>,
) {
    // Spawn few rows of bricks
    for i in -6..6 {
        for j in 3..15 { 
            commands
                .spawn(SpriteComponents {
                    material: materials.brick_material.clone(),
                    sprite: Sprite::new(Vec2::new(BRICK_SIZE_X, BRICK_SIZE_Y)),
                    transform: Transform::from_translation(
                        Vec3::new((BRICK_SIZE_X + BRICK_SPACING_X) * i as f32, (BRICK_SIZE_Y + BRICK_SPACING_Y) * j as f32, 0.)
                    ),
                    ..Default::default()
                })
                .with(Collider::Destroyable);
            }
    }
}
