use bevy::prelude::*;

pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
    pub speed: f32,
}

pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (vel, mut transform) in query.iter_mut() {
        *transform.translation.x_mut() += vel.dx * vel.speed * time.delta_seconds;
        *transform.translation.y_mut() += vel.dy * vel.speed * time.delta_seconds;
    }
}