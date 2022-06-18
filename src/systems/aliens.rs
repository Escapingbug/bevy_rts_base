use std::time::Duration;

use crate::{
    bundles::*,
    colors::Tailwind,
    systems::{attack, faction::*, time::*, unit::*},
};
use bevy::{math::Vec3, prelude::*};

struct SpawnTimer(Timer);
fn create_random_aliens(
    mut commands: Commands,
    time: Res<ControlledTime>,
    mut timer: ResMut<SpawnTimer>,
    resource: Res<AlienMeshResource>,
) {
    timer.0.tick(Duration::from_secs_f32(time.delta_seconds));

    if timer.0.finished() {
        let position = Vec3::new(
            50. * time.seconds_since_startup.to_degrees().sin() as f32,
            1.0,
            50. * time.seconds_since_startup.to_degrees().cos() as f32,
        );
        commands
            .spawn_bundle(PbrBundle {
                mesh: resource.mesh.clone(),
                material: resource.material.clone(),
                transform: Transform::from_translation(position),
                ..Default::default()
            })
            .insert_bundle(UnitBundle {
                target_position: TargetPosition {
                    pos: Some(Vec3::ZERO),
                },
                faction: Faction::new(Factions::Aliens),
                ..UnitBundle::default()
            })
            .insert(attack::Ranged::default());
    }
}

struct AlienMeshResource {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl FromWorld for AlienMeshResource {
    fn from_world(world: &mut World) -> Self {
        let mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .unwrap()
            .add(Mesh::from(shape::Cube { size: 1.0 }));
        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap()
            .add(Tailwind::PURPLE400.into());
        AlienMeshResource { mesh, material }
    }
}

pub struct AliensPlugin;
impl Plugin for AliensPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AlienMeshResource>()
            .insert_resource(SpawnTimer(Timer::from_seconds(3.0, true)))
            .add_system(create_random_aliens);
    }
}
