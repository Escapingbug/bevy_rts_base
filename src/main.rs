#[allow(unused_imports)]
use crate::systems::{
    ability, aliens, attack, axes, bullet, camera, debug, drag_selection, drone, health,
    health_numbers, selection, selection_circle, target_indicator, time, ui, unit, walker,
};
use bevy::{log::LogSettings, prelude::*, render::texture::ImagePlugin};
use bevy_mod_picking::*;

mod bundles;
mod colors;
mod helpers;
mod initialize;
mod systems;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "bavy".to_string(),
            width: 800.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ImagePlugin)
        //.add_plugin(debug::DebugPlugin)
        .add_plugin(time::TimePlugin)
        .add_plugin(PickingPlugin)
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(selection::SelectionPlugin)
        .add_plugin(drag_selection::DragSelectionPlugin)
        .add_plugin(drone::DronePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(unit::UnitPlugin)
        .add_plugin(walker::WalkerPlugin)
        .add_plugin(aliens::AliensPlugin)
        // .add_plugin(axes::AxesPlugin)
        .add_plugin(target_indicator::TargetIndicatorPlugin)
        .add_plugin(attack::AttackPlugin)
        .add_plugin(bullet::BulletPlugin)
        .add_plugin(ui::UIPlugin)
        .add_plugin(ability::AbilityPlugin)
        .add_startup_system(initialize::setup)
        .add_plugin(health::HealthPlugin)
        .add_plugin(health_numbers::HealthNumbersPlugin)
        .add_plugin(selection_circle::SelectionCirclePlugin)
        .run();
}
