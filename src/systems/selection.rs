use crate::{systems::{
    ability::*,
    selection_circle::*,
    unit::{TargetPosition, UnitSize},
}, colors::Tailwind};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::*;

#[derive(Component)]
pub struct Selectable {
    pub selected: bool,
    pub previously_selected: bool,
    pub circle: Entity,
    pub entity: Entity,
}

impl Selectable {
    pub fn set_selected(&mut self, selected: bool) {
        self.previously_selected = self.selected;
        self.selected = selected;
    }
}

#[derive(Default, Component)]
pub struct SelectableBuilder;
fn selectable_builder(
    mut commands: Commands,
    resource: Res<SelectionCircleMaterial>,
    query: Query<(Entity, &SelectableBuilder, &UnitSize)>,
) {
    for (entity, _, size) in &mut query.iter() {
        let circle = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1.0, 1.0)),
                    color: Tailwind::BLUE500.into(),
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.1, 0.0),
                    scale: Vec3::splat(0.03 * size.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(MaterialMesh2dBundle {
                material: resource.circle_material.clone(),
                mesh: resource.circle_mesh.clone().into(),
                ..default()
            })
            .insert(SelectionCircle::default())
            .id();

        commands.entity(entity).insert(Selectable {
            selected: false,
            previously_selected: false,
            circle,
            entity,
        });
        commands.entity(entity).remove::<SelectableBuilder>();
    }
}

/// Selects units
fn select_units(
    ability: Res<CurrentAbility>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    mut query: Query<&mut Selectable>,
    pick_state_query: Query<&PickingCamera>,
) {
    if ability.ability != Ability::Select {
        return;
    }

    // Only run when control is not pressed and we just clicked the left button
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    let pick_state = pick_state_query.get_single().unwrap();

    if let Some((top_entity, _intersection)) = pick_state.intersect_top() {
        if !keyboard_input.pressed(KeyCode::LControl) {
            // Deselect all units
            for mut selectable in query.iter_mut() {
                selectable.set_selected(false);
            }
        }

        // Select the top pick
        if let Ok(mut selectable) = query.get_mut(top_entity) {
            selectable.set_selected(true);
        }
    }
}

fn set_target_for_selected(
    pick_state_query: Query<&PickingCamera>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    ability: Res<CurrentAbility>,
    mut query: Query<(&Selectable, &mut TargetPosition)>,
) {
    if ability.ability != Ability::Select {
        return;
    }

    let pick_state = pick_state_query.get_single().unwrap();

    if mouse_button_inputs.just_pressed(MouseButton::Right) {
        // Get the world position
        if let Some((_top_entity, intersection)) = pick_state.intersect_top() {
            let pos = Vec3::from(intersection.position());

            for (selectable, mut target) in query.iter_mut() {
                if selectable.selected {
                    target.update_to_vec(&pos);
                }
            }
        }
    }
}

pub struct SelectionPlugin;
impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(selectable_builder)
            .add_system(select_units)
            .add_system(set_target_for_selected);
    }
}
