use bevy::prelude::*;

use crate::{modifiers::Modifiers, collider::Collider, Materials, WINDOW_HEIGHT, velocity::Velocity};

const PADDLE_WIDTH: f32 = 150.;
const PADDLE_HEIGHT: f32 = 20.;

const PADDLE_START_X: f32 = 0.;
const PADDLE_START_Y: f32 = 50. - WINDOW_HEIGHT as f32 / 2.;

const PADDLE_SPEED: f32 = 400.;

pub struct Paddle;

pub fn spawn_paddle(commands: &mut Commands, materials: &Materials) {
    commands
        .spawn(SpriteComponents {
            material: materials.paddle_material.clone(),
            sprite: Sprite::new(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            transform: Transform::from_translation(Vec3::new(PADDLE_START_X, PADDLE_START_Y , 0.)),
            ..Default::default()
        })
        .with(Paddle)
        .with(Velocity { dx: 0., dy: 0., speed: PADDLE_SPEED })
        .with(Collider::Paddle)
        .with(Modifiers::default());
}

pub fn paddle_movement(input: Res<Input<KeyCode>>, mut q: Query<With<Paddle, &mut Velocity>>) {
    for mut vel in q.iter_mut() {
        if input.pressed(KeyCode::Left) {
            vel.dx = -1.;
        } else if input.pressed(KeyCode::Right) {
            vel.dx = 1.;
        } else {
            vel.dx = 0.;
        }
    }
}