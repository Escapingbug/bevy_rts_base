use crate::systems::time::*;
use bevy::{math::Vec3, prelude::*};

#[derive(Component)]
pub struct Unit {
    pub speed: f32,
    pub social_distance: f32,
}
impl Default for Unit {
    fn default() -> Self {
        Self {
            speed: 7.0,
            social_distance: 1.6,
        }
    }
}

#[derive(Default, Component)]
pub struct TargetPosition {
    pub pos: Option<Vec3>,
}

impl TargetPosition {
    pub fn update_to_vec(&mut self, vec: &Vec3) {
        self.pos = Some(*vec);
    }
}

// Moves towards the target while it's not selected
fn unit_movement(
    time: Res<ControlledTime>,
    mut query: Query<(
        &Unit,
        &mut TargetPosition,
        &mut Transform,
        Entity,
        &UnitSize,
    )>,
) {
    // TODO Do something to divide by space or something
    let mut unit_positions = Vec::new();
    for (unit, _, transform, entity, size) in query.iter_mut() {
        unit_positions.push((entity, transform.translation, unit.social_distance * size.0));
    }

    for (unit, mut target, mut transform, entity, size) in query.iter_mut() {
        let translation = transform.translation;
        let mut velocity = Vec3::ZERO;

        // Keep a distance to other units
        // Inspired from https://github.com/JohnPeel/flock-rs
        let mut separation = Vec3::ZERO;
        let mut units_nearby = 0;
        for (other_entity, other_translation, social_distance) in &unit_positions {
            if *other_entity != entity {
                let difference = translation - *other_translation;
                let distance_squared = difference.length_squared();
                let minimum_distance = unit.social_distance * size.0 + social_distance;

                if distance_squared < minimum_distance * minimum_distance {
                    units_nearby += 1;
                    separation += difference.normalize()
                        * (minimum_distance - distance_squared.sqrt())
                        / minimum_distance;
                }
            }
        }

        // Setting vertical displacement to 0 so that big units don't move up
        separation.y = 0.;
        velocity += separation;

        // Move towards target
        if let Some(target_pos) = target.pos {
            let mut direction = target_pos - transform.translation;
            direction.y = 0.;

            if direction.length() > 0.3 + units_nearby as f32 {
                let direction = direction.normalize() * unit.speed * time.delta_seconds;
                velocity += direction;
            } else {
                // When we reach the target, remove it
                target.pos = None;
            }
        }

        // If unit is on the floor, we don't allow going down
        if translation.y <= 1.01 && velocity.y < 0. {
            velocity.y = 0.;
        }

        if time.delta_seconds > 0. {
            transform.translation += velocity;
        }
    }
}

#[derive(Component)]
pub struct UnitSize(pub f32);
impl Default for UnitSize {
    fn default() -> Self {
        Self(1.)
    }
}

pub struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(unit_movement);
    }
}
