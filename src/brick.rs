use bevy::prelude::*;

use crate::{collider::Collider, Materials,game_data::LevelFinishedEvent};

pub struct Destroyable {
    pub hp: u16,
}

pub struct DestroyableHitEvent {
    pub entity: Entity,
}

const BRICK_SIZE_X: f32 = 150.;
const BRICK_SIZE_Y: f32 = 70.;
const BRICK_SPACING_X: f32 = 20.;
const BRICK_SPACING_Y: f32 = 20.;

fn spawn_brick (commands: &mut Commands, material: Handle<ColorMaterial>, x: i32, y: i32, level: u16) {
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
        .with(Destroyable { hp: if level == 2 { 2 } else { 1 }  });
}

pub fn handle_destroyable_hit (
    mut commands: Commands,
    mut reader: Local<EventReader<DestroyableHitEvent>>,
    destroyable_hit_events: Res<Events<DestroyableHitEvent>>,
    mut level_finished_events: ResMut<Events<LevelFinishedEvent>>,
    mut destroyable_q: Query<&mut Destroyable>
) {
    if let Some(event) = reader.iter(&destroyable_hit_events).next() {
        // This should be always true
        let mut destroyable = destroyable_q.get_mut(event.entity).unwrap();
        destroyable.hp -= 1;
        if destroyable.hp == 0 {
            commands.despawn(event.entity);

            // We calculate if there are any destroyables left, if not we won the level
            // We subtract 1 because the last entity despawn will execure after this query
            let destroyables_left = destroyable_q.iter_mut().count() - 1;
            println!("Destroyables left: {}", destroyables_left);
            
            if destroyables_left == 0 {
                level_finished_events.send(LevelFinishedEvent::Success);
            }
        }
    }
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
                spawn_brick(&mut commands, material, x, y, level);
            }   
        }
    }

    // Spawn few rows of bricks
    if level == 2 {
        for x in -4..=4 {
            for y in 1..=3 { 
                let material = materials.brick_material.clone();
                spawn_brick(&mut commands, material, x, y, level);
            }   
        }
    }
}
