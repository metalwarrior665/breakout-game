use bevy::prelude::*;

use crate::{velocity::Velocity};

pub enum ModifierStatus {
    Unapplied,
    Applied,
    Removed,
}

#[derive(PartialEq, Eq)]
pub enum ModifierType {
    Speed,
    SizeX,
    Size,
}
pub struct Modifier {
    pub mod_type: ModifierType,
    pub value: f32,
    pub status: ModifierStatus,
}

impl Modifier {
    pub fn update_components(&self, mult: f32, velocity: &mut Velocity, sprite: &mut Sprite) {
        match self.mod_type {
            ModifierType::Speed => {
                let old_speed = velocity.speed;
                velocity.speed *= mult;
                println!("Updated speed from {} to {}", old_speed, velocity.speed);
            },
            ModifierType::SizeX => {
                let old_size = sprite.size;
                *sprite.size.x_mut() *= mult;
                println!("Updated size from {} to {}", old_size, sprite.size);
            },
            ModifierType::Size => {
                let old_size = sprite.size;
                *sprite.size.x_mut() *= mult;
                *sprite.size.y_mut() *= mult;
                println!("Updated size from {} to {}", old_size, sprite.size);
            }
        }
    }
}
#[derive(Default)]
pub struct Modifiers {
    pub modifiers: Vec<Modifier>
}

impl Modifiers {
    pub fn replace_or_insert(&mut self, new_modifier: Modifier) {
        let maybe_modifier = self.modifiers.iter_mut().find(|current_modifier| 
            if new_modifier.mod_type == current_modifier.mod_type {
                true
            } else {
                false
            }
        );
        if let Some(found_modifier) = maybe_modifier {
            found_modifier.status = ModifierStatus::Removed;
        }
        self.modifiers.push(new_modifier);
    }
}

pub fn apply_modifiers(
    mut q: Query<(& mut Modifiers, & mut Velocity, & mut Sprite)>
) {
    for (mut modifiers, mut velocity, mut sprite) in q.iter_mut() {
        let mut modifier_indexes_to_remove: Vec<usize> = vec![];
        for (index, modifier) in modifiers.modifiers.iter_mut().enumerate() {
            match modifier.status {
                ModifierStatus::Applied => {},
                ModifierStatus::Removed => {
                    // We reverse the modifer effect
                    modifier.update_components(1. / modifier.value, &mut velocity, &mut sprite);
                    modifier_indexes_to_remove.push(index);
                },
                ModifierStatus::Unapplied => {
                    // We apply the modifier effect
                    modifier.update_components(modifier.value, &mut velocity, &mut sprite);
                    modifier.status = ModifierStatus::Applied;
                }
            }
        } 

        // Now we clean all the dead modifiers
        for index in modifier_indexes_to_remove {
            println!("Removed old modifier");
            modifiers.modifiers.remove(index);
        }
    }

    
}
