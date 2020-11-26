use std::time::Duration;
use bevy::prelude::*;
use rand::Rng;

use crate::{
    collider::Collider,
    Materials,
    velocity::Velocity,
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
    game_data::{GameData,GameState},
};

const POPUP_SIZE: f32 = 20.;
const POPUP_START_Y: f32 = WINDOW_HEIGHT / 2.;
const POPUP_SPEED: f32 = 100.;
const BASE_POPUP_INTERVAL: u64 = 20000;

#[derive(Debug)]
pub enum Popup {
    Speed(f32),
    Size(f32),
}

pub struct PopupSpawnTimer(Timer);

impl Default for PopupSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(BASE_POPUP_INTERVAL), true))
    }
}

pub fn spawn_popup (
    mut commands: Commands,
    time: Res<Time>,
    materials: Res<Materials>,
    game_data: Res<GameData>,
    mut timer: Local<PopupSpawnTimer>,
) {
    if game_data.state != GameState::Play {
        return;
    }
    timer.0.tick(time.delta_seconds);
    if timer.0.finished {
        // -1, 1
        let random_x = rand::random::<f32>() * 2. - 1.;
        let x_pos = WINDOW_WIDTH / 2. * random_x;
        

        let mut rng = rand::thread_rng();
        let random_type = rng.gen_range(0, 8);
        let popup = match random_type {
            0..=1 => Popup::Size(2.),
            2 => Popup::Size(0.5),
            3 => Popup::Size(3.),
            4..=6 => Popup::Speed(2.),
            7 => Popup::Speed(3.),
            _ => Popup::Speed(2.),
        };

        println!("Spawning popup {:?} at x: {}", popup, x_pos);

        let (material,  mult, size_x) = match popup {
            Popup::Size(mult) => (materials.popup_material_size.clone(), mult, 2.),
            Popup::Speed(mult) => (materials.popup_material_speed.clone(), mult, 1.),
        };
        commands
            .spawn(SpriteComponents {
                material,
                sprite: Sprite::new(Vec2::new(POPUP_SIZE * mult * size_x, POPUP_SIZE * mult)),
                transform: Transform::from_translation(
                    Vec3::new(x_pos, POPUP_START_Y, 0.)
                ),
                ..Default::default()
            })
            .with(Collider::Popup)
            .with(Velocity { dx: 0., dy: -1., speed: POPUP_SPEED })
            .with(popup);
    }  
}