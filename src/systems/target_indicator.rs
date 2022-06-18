use crate::helpers::shapes::*;
use crate::systems::{selection::*, unit::TargetPosition};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::{math::Vec3, prelude::*};

#[derive(Component)]
pub struct TargetIndicator;
fn show_target_indicator(
    mut indicator_query: Query<(&TargetIndicator, &mut Transform, &mut Visibility)>,
    selectable_query: Query<(&Selectable, &TargetPosition)>,
) {
    let mut selections_with_target_exist = false;
    for (selectable, target) in selectable_query.iter() {
        // We only want selected items
        if !selectable.selected {
            continue;
        }

        // Set the Indicator to the Target position
        if let Some(target_position) = target.pos {
            selections_with_target_exist = true;

            for (_, mut transform, _) in indicator_query.iter_mut() {
                transform.translation = Vec3::new(target_position.x, 0.3, target_position.z);
            }
        }
    }

    // Toggle drawability according to if there is anything selected
    for (_, _, mut draw) in &mut indicator_query.iter_mut() {
        draw.is_visible = selections_with_target_exist;
    }
}

pub fn create_target_indicator(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 0.8),
                unlit: true,
                ..default()
            }),
            mesh: meshes.add(circle_mesh()).into(),
            transform: Transform::from_scale(Vec3::splat(0.01)),
            ..default()
        })
        .insert(TargetIndicator);
}

pub struct TargetIndicatorPlugin;
impl Plugin for TargetIndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_target_indicator)
            .add_system(show_target_indicator);
    }
}
