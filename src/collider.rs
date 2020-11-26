use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};

use crate::{
    ball::Ball,
    velocity::Velocity,
    paddle::Paddle,
    modifiers::{Modifiers,Modifier,ModifierStatus,ModifierType},
    powerup::Powerup,
    brick::{DestroyableHitEvent},
    game_data::{LifeLostEvent},
};

#[derive(Debug)]
pub enum Collider {
    Ball,
    Paddle,
    Destroyable,
    WallLeft,
    WallRight,
    Invunerable,
    Powerup,
}

impl Collider {
    pub fn is_solid(&self) -> bool {
        match self {
            Collider::Powerup => false,
            _ => true
        }
    }
}

pub fn paddle_collisions(
    mut commands: Commands,
    mut life_lost_events: ResMut<Events<LifeLostEvent>>, 
    mut paddle_colls: Query<With<Paddle, (&mut Transform, &Sprite, &mut Modifiers)>>,
    // Ball handless all of its own collisions
    all_collisions: Query<Without<Ball, Without<Paddle, (&Transform, &Sprite, &Collider, Entity)>>>,
    powerups: Query<&Powerup>,
) {
    for (mut paddle_transform, paddle_sprite, mut modifiers) in paddle_colls.iter_mut() {
        for (col_transform, col_sprite, collider, collider_entity) in all_collisions.iter() {
            let collision = collide(
                paddle_transform.translation,
                paddle_sprite.size,
                col_transform.translation,
                col_sprite.size,
            );

            if collision.is_some() {
                println!("Paddle collider");
                // Picked powerup
                if let Ok(powerup) = powerups.get(collider_entity) {
                    println!("powerup collider collider");
                    match powerup {
                        Powerup::Speed(mult) => modifiers.replace_or_insert(Modifier{
                            mod_type: ModifierType::Speed,
                            value: *mult,
                            status: ModifierStatus::Unapplied
                        }),
                        Powerup::Size(mult) => modifiers.replace_or_insert(Modifier{
                            mod_type: ModifierType::SizeX,
                            value: *mult,
                            status: ModifierStatus::Unapplied
                        }),
                        Powerup::Bomb => life_lost_events.send(LifeLostEvent),
                    }
                    commands.despawn(collider_entity);
                }   
            }

            // Special walls handling
            // Walls are too thin and sometimes you get through with normal collider
            if let Collider::WallLeft = collider {
                let min_paddle_x = col_transform.translation.x() + col_sprite.size.x() / 2. + paddle_sprite.size.x() / 2.;
                if paddle_transform.translation.x() < min_paddle_x {
                    println!("Hit left wall, fixing from {} to {}", paddle_transform.translation.x(), min_paddle_x);
                    *paddle_transform.translation.x_mut() = min_paddle_x;    
                }
            }
            
            if let Collider::WallRight = collider {
                let max_paddle_x = col_transform.translation.x() - col_sprite.size.x() / 2. - paddle_sprite.size.x() / 2.;
                if paddle_transform.translation.x() > max_paddle_x {
                    println!("Hit right wall, fixing from {} to {}", paddle_transform.translation.x(), max_paddle_x);
                    *paddle_transform.translation.x_mut() = max_paddle_x;
                }
            }
        }
    }
}

pub fn ball_collisions(
    mut destroyable_hit_events: ResMut<Events<DestroyableHitEvent>>,
    mut ball_collisions: Query<With<Ball, (&Transform, &mut Velocity, &Sprite)>>,
    all_collisions: Query<(&Transform, &Sprite, &Collider, Entity)>,
    paddle_q: Query<&Paddle>,
) {
    // Ball collisions
    for (ball_transform, mut ball_vel, ball_sprite) in ball_collisions.iter_mut() {
        for (col_transform, col_sprite, collider, collider_entity) in all_collisions.iter() {
            // TODO: Replace this collider, things often go through
            let collision = collide(
                ball_transform.translation,
                ball_sprite.size,
                col_transform.translation,
                col_sprite.size,
            );

            if let Some(collision) = collision {
                if let Collider::Destroyable = *collider {
                    destroyable_hit_events.send(DestroyableHitEvent { entity: collider_entity });
                }

                // break if this collide is on a solid, otherwise continue check whether a solid is also in collision
                if collider.is_solid() {
                    // reflect the ball when it collides
                    let mut reflect_x = false;
                    let mut reflect_y = false;

                    // only reflect if the ball's velocity is going in the opposite direction of the collision
                    match collision {
                        Collision::Left => reflect_x = ball_vel.dx > 0.0,
                        Collision::Right => reflect_x = ball_vel.dx < 0.0,
                        Collision::Top => reflect_y = ball_vel.dy < 0.0,
                        Collision::Bottom => reflect_y = ball_vel.dy > 0.0,
                    }

                    println!(
                        "ball_pos: {}, col_pos: {}, col_size: {}",
                        ball_transform.translation,
                        col_transform.translation,
                        col_sprite.size
                    );

                    println!(
                        "ball collider: {:?}, collison: {:?}, reflect_x: {}, reflect_y: {}",
                        collider, collision, reflect_x, reflect_y
                    );

                    if reflect_x {
                        ball_vel.dx = -ball_vel.dx;
                    }
    
                    if reflect_y {
                        ball_vel.dy = -ball_vel.dy;
                    }

                    if let Ok(_paddle) = paddle_q.get(collider_entity) {
                        let ball_x = ball_transform.translation.x();
                        let paddle_x = col_transform.translation.x();
                        let paddle_diameter = col_sprite.size.x() / 2.;
                        let collision_position = (ball_x - paddle_x) / paddle_diameter;
                        println!("Collision position: {}", collision_position);
                        ball_vel.dx += collision_position;
                    }
                    break;
                }
            }

            // Special walls handling
            // Walls are too thin and sometimes you get through with normal collider
            if let Collider::WallLeft = collider {
                let min_ball_x = col_transform.translation.x() + col_sprite.size.x() / 2. + ball_sprite.size.x() / 2.;
                if ball_transform.translation.x() < min_ball_x {
                    println!("Hit left wall, fixing from {} to {}", ball_transform.translation.x(), min_ball_x);
                    ball_vel.dx = -ball_vel.dx;  
                }
            }
            
            if let Collider::WallRight = collider {
                let max_ball_x = col_transform.translation.x() - col_sprite.size.x() / 2. - ball_sprite.size.x() / 2.;
                if ball_transform.translation.x() > max_ball_x {
                    println!("Hit right wall, fixing from {} to {}", ball_transform.translation.x(), max_ball_x);
                    ball_vel.dx = -ball_vel.dx;
                }
            }
        }
    }
}