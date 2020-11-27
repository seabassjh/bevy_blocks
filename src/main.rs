mod mesh_generator;
mod third_person_controller;

use bevy::{math::vec4, prelude::*};
use mesh_generator::{mesh_generator_system, setup_mesh_generator_system, MeshGeneratorState, MeshMaterial};
use third_person_controller::ThirdPersonControllerPlugin;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "My Bevy Game".to_string(),
            width: 1920,
            height: 1080,
            vsync: true,
            resizable: false,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::rgb(0.4, 0.8, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_system(setup_mesh_generator_system.system())
        .add_plugin(ThirdPersonControllerPlugin)
        .add_system(mesh_generator_system.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // add entities to the world
    commands
        // sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            transform: Transform::from_translation(Vec3::zero()),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 0.0)),
            ..Default::default()
        });

    commands.insert_resource(MeshMaterial(
        materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
    ));

    commands.insert_resource(MeshGeneratorState::new());
}
