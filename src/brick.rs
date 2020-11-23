use bevy::prelude::*;

use crate::{collider::Collider, Materials};

pub struct Brick {
    hp: u16,
}

const BRICK_SIZE_X: f32 = 150.;
const BRICK_SIZE_Y: f32 = 70.;
const BRICK_SPACING_X: f32 = 20.;
const BRICK_SPACING_Y: f32 = 20.;

fn spawn_brick (commands: &mut Commands, material: Handle<ColorMaterial>, x: i32, y: i32, ) {
    commands
        .spawn(SpriteComponents {
            material,
            sprite: Sprite::new(Vec2::new(BRICK_SIZE_X, BRICK_SIZE_Y)),
            transform: Transform::from_translation(
                Vec3::new((BRICK_SIZE_X + BRICK_SPACING_X) * x as f32, (BRICK_SIZE_Y + BRICK_SPACING_Y) * y as f32, 0.)
            ),
            ..Default::default()
        })
        .with(Collider::Destroyable)
        .with(Brick { hp: 1 });
}   

pub fn spawn_bricks(
    mut commands: &mut Commands,
    materials: &Materials,
    level: u16,
) {
    // Spawn few rows of bricks
    if level == 1 {
        for x in -3..=3 {
            for y in 1..=2 { 
                let material = materials.brick_material.clone();
                spawn_brick(&mut commands, material, x, y);
            }   
        }
    }

    // Spawn few rows of bricks
    if level == 2 {
        for x in -6..6 {
            for y in 1..5 { 
                let material = materials.brick_material.clone();
                spawn_brick(&mut commands, material, x, y);
            }   
        }
    }
}
