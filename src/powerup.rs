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

const POWERUP_SIZE: f32 = 20.;
const POWERUP_START_Y: f32 = WINDOW_HEIGHT / 2.;
const POWERUP_SPEED: f32 = 100.;
const BASE_POWERUP_INTERVAL: u64 = 20000;

#[derive(Debug)]
pub enum Powerup {
    Speed(f32),
    Size(f32),
    Bomb,
}

pub struct PowerupSpawnTimer(Timer);

impl Default for PowerupSpawnTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_millis(BASE_POWERUP_INTERVAL), true))
    }
}

pub fn spawn_powerup (
    mut commands: Commands,
    time: Res<Time>,
    materials: Res<Materials>,
    game_data: Res<GameData>,
    mut timer: Local<PowerupSpawnTimer>,
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
        let random_type = rng.gen_range(0, 10);
        let powerup = match random_type {
            0..=1 => Powerup::Size(2.),
            2 => Powerup::Size(0.5),
            3 => Powerup::Size(3.),
            4..=6 => Powerup::Speed(2.),
            7 => Powerup::Speed(3.),
            8..=10 => Powerup::Bomb,
            _ => Powerup::Speed(2.),
        };

        println!("Spawning powerup {:?} at x: {}", powerup, x_pos);

        let (material, size, size_x, speed) = match powerup {
            Powerup::Size(mult) => (materials.powerup_material_size.clone(), mult, 2., 1.),
            Powerup::Speed(mult) => (materials.powerup_material_speed.clone(), mult, 1., 1.),
            Powerup::Bomb => (materials.powerup_material_bomb.clone(), 3., 1., 2.),
        };
        commands
            .spawn(SpriteComponents {
                material,
                sprite: Sprite::new(Vec2::new(POWERUP_SIZE * size * size_x, POWERUP_SIZE * size)),
                transform: Transform::from_translation(
                    Vec3::new(x_pos, POWERUP_START_Y, 0.)
                ),
                ..Default::default()
            })
            .with(Collider::Powerup)
            .with(Velocity { dx: 0., dy: -1., speed: POWERUP_SPEED * speed })
            .with(powerup);
    }  
}