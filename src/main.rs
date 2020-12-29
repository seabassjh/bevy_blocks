mod debug_fly_controller;
mod player_controller;
mod voxel_terrain;

use bevy::prelude::*;
use bevy_rapier3d::physics::{RapierConfiguration, RapierPhysicsPlugin};
use debug_fly_controller::DebugFlyControllerPlugin;
use player_controller::PlayerControllerPlugin;
use voxel_terrain::generator::VoxelTerrainGeneratorPlugin;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Neveround".to_string(),
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_resource(Msaa { samples: 4 })
        .add_resource(ClearColor(Color::rgb(0.4, 0.8, 1.0)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_resource(CursorState::new())
        .add_system(toggle_cursor_system.system())
        .add_plugin(RapierPhysicsPlugin)
        .add_resource(RapierConfiguration {
            time_dependent_number_of_timesteps: true,
            ..Default::default()
        })
        //.add_plugin(DebugFlyControllerPlugin)
        .add_plugin(PlayerControllerPlugin)
        .add_plugin(VoxelTerrainGeneratorPlugin)
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
            transform: Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
            ..Default::default()
        });
}

pub struct CursorState {
    cursor_locked: bool,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            cursor_locked: false,
        }
    }
}

fn toggle_cursor_system(
    mut state: ResMut<CursorState>,
    input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    if input.just_pressed(KeyCode::Escape) {
        state.cursor_locked = !state.cursor_locked;
        let lock_mode = state.cursor_locked;
        let visibility = !state.cursor_locked;
        window.set_cursor_lock_mode(lock_mode);
        window.set_cursor_visibility(visibility);
    }
}
