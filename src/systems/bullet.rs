use crate::colors::Tailwind;
use crate::systems::{faction::*, health::Health, time::*, unit::*};
use bevy::{math::Vec3, prelude::*};

const BULLET_SPEED: f32 = 30.;

#[derive(Component)]
pub struct Bullet {
    /// The entity that this bullet towards
    pub towards: Entity,
}
impl Bullet {
    pub fn spawn(
        commands: &mut Commands,
        resource: &BulletMeshResource,
        origin: Vec3,
        towards: Entity,
        faction: Factions,
    ) {
        commands
            .spawn_bundle(PbrBundle {
                mesh: resource.mesh.clone(),
                material: resource.material.clone(),
                transform: Transform::from_translation(origin),
                ..Default::default()
            })
            .insert(Bullet { towards })
            .insert(Faction::new(faction));
    }
}

fn move_bullet(
    mut commands: Commands,
    time: Res<ControlledTime>,
    mut query: Query<(&Bullet, &mut Transform, Entity)>,
    target_query: Query<&Transform, Without<Bullet>>,
) {
    for (bullet, mut transform, bullet_entity) in query.iter_mut() {
        let target_transform = target_query.get(bullet.towards);
        if let Ok(target_transform) = target_transform {
            let direction = (target_transform.translation - transform.translation).normalize();
            transform.translation += BULLET_SPEED * direction * time.delta_seconds;
        } else {
            // target dies but bullet hasn't arrived, remove the bullet
            commands.entity(bullet_entity).despawn();
        }
    }
}

/*
fn kill_after_lifetime_over(
    mut commands: Commands,
    time: Res<ControlledTime>,
    query: Query<(&Bullet, Entity)>,
) {
    for (bullet, entity) in query.iter() {
        if time.seconds_since_startup >= bullet.should_despawn_at {
            commands.entity(entity).despawn();
        }
    }
}
*/

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(&Bullet, &Transform, &Faction, Entity)>,
    mut unit_query: Query<(&Unit, &Transform, &mut Health, &Faction)>,
) {
    for (_, bullet_transform, faction, bullet_entity) in bullet_query.iter() {
        let bullet_translation = bullet_transform.translation;

        for (_, enemy_transform, mut health, enemy_faction) in unit_query.iter_mut() {
            // Skip units in same faction
            if enemy_faction.faction == faction.faction {
                continue;
            }

            let enemy_translation = enemy_transform.translation;
            let distance = (bullet_translation - enemy_translation).length();

            if distance < 1.0 {
                health.damage(1);

                commands.entity(bullet_entity).despawn();
            }
        }
    }
}

pub struct BulletMeshResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for BulletMeshResource {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .unwrap()
            .add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.3,
            }));
        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap()
            .add(StandardMaterial {
                base_color: Tailwind::BLACK.into(),
                unlit: true,
                ..default()
            });
        BulletMeshResource { mesh, material }
    }
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BulletMeshResource>()
            .add_system(move_bullet)
            //.add_system(kill_after_lifetime_over)
            .add_system(bullet_collision);
    }
}
