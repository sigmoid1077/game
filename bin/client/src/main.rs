use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use voxel_world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            voxel_world::VoxelWorldPlugin
        ))
        .add_systems(Startup, |mut commands: Commands, voxel_world: voxel_world::VoxelWorldSystemParam| {
            commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(-200., 100., -200.).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            });

            voxel_world.load_chunk();
        })
        .run();
}
