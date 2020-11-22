use bevy::prelude::*;

use crate::{modifiers::Modifiers, collider::Collider, WINDOW_HEIGHT, Materials, velocity::Velocity, popup::Popup};

const BALL_START_SPEED: f32 = 400.;
const BALL_START_X: f32 = 0.;
const BALL_START_Y: f32 = 200. - WINDOW_HEIGHT as f32 / 2.;
const BALL_DIAMETER: f32 = 30.;

pub struct Ball;

fn _spawn_ball(commands: & mut Commands, materials: &Materials) {
    let random = rand::random::<f32>() - 0.5;
    println!("Random: {}", random);
    commands
        .spawn(SpriteComponents {
            material: materials.ball_material.clone(),
            sprite: Sprite::new(Vec2::new(BALL_DIAMETER, BALL_DIAMETER)),
            transform: Transform::from_translation(Vec3::new(BALL_START_X, BALL_START_Y , 0.)),
            ..Default::default()
        })
        .with(Velocity { dx: random, dy: 1., speed: BALL_START_SPEED })
        .with(Ball)
        .with(Collider::Ball)
        .with(Modifiers::default());
}

pub fn spawn_ball(mut commands: Commands, materials: Res<Materials>) {
    _spawn_ball(& mut commands, &materials);
}

pub fn despawn_fallen(
    mut commands: Commands,
    materials: Res<Materials>,
    q_ball: Query<With<Ball, (&Transform, Entity)>>,
    q_popup: Query<With<Popup, (&Transform, Entity)>>
) {
    for (transform, entity) in q_popup.iter() {
        let y = transform.translation.y();
        if y < -(WINDOW_HEIGHT as f32 / 2.) {
            println!("Popup despawned");
            commands.despawn(entity);
        }
    }
    for (transform, entity) in q_ball.iter() {
        let y = transform.translation.y();
        if y < -(WINDOW_HEIGHT as f32 / 2.) {
            commands.despawn(entity);

            _spawn_ball(& mut commands, &materials);
        }
    }
}
