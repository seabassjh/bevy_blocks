use bevy::{input::keyboard::*, input::mouse::*, prelude::*};

mod third_person_controller;

use third_person_controller::ThirdPersonControllerPlugin;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .init_resource::<InputState>()
        .add_system(input_handling.system())
        .add_plugin(ThirdPersonControllerPlugin)
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
        // plane
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
            ..Default::default()
        })
        // sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            transform: Transform::from_translation(Vec3::new(1.5, 1.5, 1.5)),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

#[derive(Default)]
pub struct InputState {
    _keys: EventReader<KeyboardInput>,
    _cursor: EventReader<CursorMoved>,
    motion: EventReader<MouseMotion>,
    _mousebtn: EventReader<MouseButtonInput>,
    _scroll: EventReader<MouseWheel>,
}

fn input_handling(
    _windows: ResMut<Windows>,
    _winit: ResMut<bevy::winit::WinitWindows>,
    mut _state: ResMut<InputState>,
    _ev_keys: Res<Events<KeyboardInput>>,
    _ev_cursor: Res<Events<CursorMoved>>,
    _ev_motion: Res<Events<MouseMotion>>,
    _ev_mousebtn: Res<Events<MouseButtonInput>>,
    _ev_scroll: Res<Events<MouseWheel>>,
) {
}
