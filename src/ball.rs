use bevy::prelude::*;

use crate::{
    modifiers::{Modifier,Modifiers,ModifierType,ModifierStatus},
    collider::Collider,
    WINDOW_HEIGHT,
    Materials,
    velocity::Velocity,
    powerup::Powerup,
    game_data::{GameData, LifeLostEvent},
};

const BALL_START_SPEED: f32 = 400.;
const BALL_START_X: f32 = 0.;
const BALL_START_Y: f32 = 200. - WINDOW_HEIGHT / 2.;
const BALL_DIAMETER: f32 = 30.;

pub struct Ball;

pub fn spawn_ball(commands: & mut Commands, materials: &Materials, speed: f32, size_mult: f32) {
    let random = rand::random::<f32>() - 0.5;

    let mut modifiers = Modifiers {
        modifiers: vec![],
    };

    // Currently just hardcoding this, will be better if modifiers are passed from level setup
    if size_mult != 1. {
        let modifier = Modifier {
            mod_type: ModifierType::Size,
            value: size_mult,
            status: ModifierStatus::Unapplied,
        };
        modifiers.modifiers.push(modifier);
    }
    commands
        .spawn(SpriteComponents {
            material: materials.ball_material.clone(),
            sprite: Sprite::new(Vec2::new(BALL_DIAMETER, BALL_DIAMETER)),
            transform: Transform::from_translation(Vec3::new(BALL_START_X, BALL_START_Y , 0.)),
            ..Default::default()
        })
        .with(Velocity { dx: random, dy: 1., speed: BALL_START_SPEED * speed })
        .with(Ball)
        .with(Collider::Ball)
        .with(modifiers);
}

pub fn handle_fallen_down(
    mut commands: Commands,
    materials: Res<Materials>,
    game_data: ResMut<GameData>,
    mut life_lost_events: ResMut<Events<LifeLostEvent>>, 
    q_ball: Query<With<Ball, (&Transform, Entity)>>,
    q_powerup: Query<With<Powerup, (&Transform, Entity)>>
) {
    for (transform, entity) in q_powerup.iter() {
        let y = transform.translation.y();
        if y < -(WINDOW_HEIGHT / 2.) {
            println!("Powerup despawned");
            commands.despawn(entity);
        }
    }
    for (transform, entity) in q_ball.iter() {
        let y = transform.translation.y();
        if y < -(WINDOW_HEIGHT / 2.) {
            commands.despawn(entity);
            life_lost_events.send(LifeLostEvent);
            spawn_ball(& mut commands, &materials, 1., 1.);
        }
    }
}
