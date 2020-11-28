use bevy::prelude::*;

use crate::{
    collider::{Collider,BallHitEvent},
    game_data::LevelFinishedEvent
};

pub struct Destroyable {
    pub hp: u16,
}

pub const BRICK_SIZE_X: f32 = 150.;
const BRICK_SIZE_Y: f32 = 70.;

pub fn spawn_brick (commands: &mut Commands, material: Handle<ColorMaterial>, x: f32, y: f32, hp: u16, size_mult: f32) {
    commands
        .spawn(SpriteComponents {
            material,
            sprite: Sprite::new(Vec2::new(BRICK_SIZE_X * size_mult, BRICK_SIZE_Y * size_mult)),
            transform: Transform::from_translation(
                Vec3::new(x, y, 0.)
            ),
            ..Default::default()
        })
        .with(Collider::Destroyable)
        .with(Destroyable { hp });
}

pub fn handle_destroyable_hit (
    mut commands: Commands,
    mut reader: Local<EventReader<BallHitEvent>>,
    ball_hit_events: Res<Events<BallHitEvent>>,
    mut level_finished_events: ResMut<Events<LevelFinishedEvent>>,
    mut destroyable_q: Query<&mut Destroyable>
) {
    if let Some(BallHitEvent::Destroyable(entity)) = reader.iter(&ball_hit_events).next() {
        // This should be always true
        let mut destroyable = destroyable_q.get_mut(*entity).unwrap();
        destroyable.hp -= 1;
        if destroyable.hp == 0 {
            commands.despawn(*entity);

            // We calculate if there are any destroyables left, if not we won the level
            // We subtract 1 because the last entity despawn will execure after this query
            let destroyables_left = destroyable_q.iter_mut().count() - 1;
            println!("Destroyables left: {}", destroyables_left);
            
            if destroyables_left == 0 {
                level_finished_events.send(LevelFinishedEvent::Won);
            }
        }
    }
}