use crate::colors::*;
use crate::helpers::shapes::*;
use crate::systems::selection::*;
use bevy::prelude::*;
use bevy_mod_picking::*;

#[derive(Default, Component)]
pub struct SelectionCircle {
    pub unit_highlighted: bool, // Used to highlight the unit, e.g. when hovering a button
    pub unit_hovered: bool,
    pub unit_selected: bool,
}
impl SelectionCircle {
    pub fn visible(&self) -> bool {
        self.unit_hovered || self.unit_highlighted || self.unit_selected
    }
}

fn move_circle_for_selected_units(
    query: Query<(&Selectable, &Transform)>,
    mut circle_query: Query<(&SelectionCircle, &mut Transform), Without<Selectable>>,
) {
    for (selectable, transform) in query.iter() {
        if let Ok((circle, mut circle_transform)) = circle_query.get_mut(selectable.circle) {
            if circle.visible() {
                let translation = transform.translation;
                circle_transform.translation = Vec3::new(translation.x, 0.1, translation.z);
            }
        }
    }
}

fn set_unit_hovered_for_circles(
    pick_state_query: Query<&PickingCamera>,
    query: Query<(&Selectable, Entity)>,
    mut circle_query: Query<&mut SelectionCircle>,
) {
    let pick_state = pick_state_query.get_single().unwrap();
    for (selectable, entity) in query.iter() {
        if let Ok(mut circle) = circle_query.get_component_mut::<SelectionCircle>(selectable.circle)
        {
            if let Some((top_entity, _)) = pick_state.intersect_top() {
                if entity == top_entity {
                    circle.unit_hovered = true;
                } else {
                    circle.unit_hovered = false;
                }
            } else {
                circle.unit_hovered = false;
            }
        }
    }
}

fn set_unit_selected_for_circles(
    query: Query<&Selectable, Changed<Selectable>>,
    mut circle_query: Query<&mut SelectionCircle>,
) {
    for selectable in query.iter() {
        if let Ok(mut circle) = circle_query.get_component_mut::<SelectionCircle>(selectable.circle)
        {
            circle.unit_selected = selectable.selected;
        }
    }
}

fn change_circle_color(
    resource: Res<SelectionCircleMaterial>,
    mut query: Query<(
        &SelectionCircle,
        &mut Visibility,
        &mut Handle<StandardMaterial>,
    )>,
) {
    for (circle, mut draw, mut material) in query.iter_mut() {
        *material = if circle.unit_highlighted {
            resource.highlighted_material.clone()
        } else if circle.unit_hovered {
            resource.hover_material.clone()
        } else {
            resource.selected_material.clone()
        };

        draw.is_visible = circle.visible();
        if draw.is_visible {
            info!("change circle color");
        }
    }
}

pub struct SelectionCircleMaterial {
    pub circle_mesh: Handle<Mesh>,
    pub circle_material: Handle<StandardMaterial>,
    pub selected_material: Handle<StandardMaterial>,
    pub hover_material: Handle<StandardMaterial>,
    pub highlighted_material: Handle<StandardMaterial>,
}

impl FromWorld for SelectionCircleMaterial {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let circle_material = materials.add(StandardMaterial {
            base_color: Tailwind::BLUE500.into(),
            unlit: true,
            ..default()
        });
        let selected_material = materials.add(StandardMaterial {
            base_color: Tailwind::BLUE500.into(),
            unlit: true,
            ..default()
        });
        let hover_material = materials.add(StandardMaterial {
            base_color: Tailwind::BLUE300.into(),
            unlit: true,
            ..default()
        });
        let highlighted_material = materials.add(StandardMaterial {
            base_color: Tailwind::YELLOW300.into(),
            unlit: true,
            ..default()
        });
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let circle_mesh = meshes.add(circle_mesh());
        SelectionCircleMaterial {
            circle_mesh,
            selected_material,
            circle_material,
            hover_material,
            highlighted_material,
        }
    }
}

pub struct SelectionCirclePlugin;
impl Plugin for SelectionCirclePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectionCircleMaterial>()
            .add_system(change_circle_color)
            .add_system(set_unit_hovered_for_circles)
            .add_system(set_unit_selected_for_circles)
            .add_system(move_circle_for_selected_units);
    }
}
