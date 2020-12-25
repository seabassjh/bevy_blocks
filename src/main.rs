mod first_person_controller;
mod voxel_generator;

use bevy::{prelude::*};
use first_person_controller::FirstPersonControllerPlugin;
use voxel_generator::{
    VoxelGeneratorPlugin,
};

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "My Bevy Game".to_string(),
            width: 1920.0,
            height: 1080.0,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::rgb(0.4, 0.8, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(FirstPersonControllerPlugin)
        .add_plugin(VoxelGeneratorPlugin)
        .run();
}

/// set up a simple 3D scene
fn setup(
    commands: &mut Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Watch for changes
    asset_server.watch_for_changes().unwrap();

    // add entities to the world
    commands
        // sphere
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            transform: Transform::from_translation(Vec3::zero()),
            ..Default::default()
        })
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
            ..Default::default()
        });

}
