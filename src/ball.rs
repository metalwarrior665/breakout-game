use bevy::prelude::*;

use crate::{
    modifiers::{Modifier,Modifiers,ModifierType,ModifierStatus},
    collider::Collider,
    WINDOW_HEIGHT,
    Materials,
    velocity::Velocity,
    popup::Popup,
    game_data::{GameData, LevelFinishedEvent},
};

const BALL_START_SPEED: f32 = 400.;
const BALL_START_X: f32 = 0.;
const BALL_START_Y: f32 = 200. - WINDOW_HEIGHT as f32 / 2.;
const BALL_DIAMETER: f32 = 30.;

pub struct Ball;

pub fn spawn_ball(commands: & mut Commands, materials: &Materials, level: u16) {
    let random = rand::random::<f32>() - 0.5;

    let mut modifiers = Modifiers {
        modifiers: vec![],
    };

    // Currently just hardcoding this, will be better if modifiers are passed from level setup
    if level == 2 {
        let modifier = Modifier {
            mod_type: ModifierType::Size,
            value: 0.75,
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
        .with(Velocity { dx: random, dy: 1., speed: BALL_START_SPEED })
        .with(Ball)
        .with(Collider::Ball)
        .with(modifiers);
}

pub fn handle_fallen_down(
    mut commands: Commands,
    materials: Res<Materials>,
    mut game_data: ResMut<GameData>,
    mut game_over_events: ResMut<Events<LevelFinishedEvent>>, 
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

            game_data.lives -= 1;
            println!("Fallen down! Current lives: {}", game_data.lives);
            if game_data.lives == 0 {
                game_over_events.send(LevelFinishedEvent::Failure)
            } else {
                spawn_ball(& mut commands, &materials, game_data.level);
            }
        }
    }
}
