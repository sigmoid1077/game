use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new()
        ))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(-200., 100., -200.).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });
        })
        .run();
}
