use bevy::{input::keyboard::*, input::mouse::*, winit::WinitWindows, prelude::*};
use winit::dpi::LogicalPosition;

mod third_person_controller;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .init_resource::<InputState>()
        .add_system(input_handling.system())
        .add_system(player_movement_system.system())
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
        // cube
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(Player { speed: 10.0 })
        .with_children(|parent| {
            parent
                .spawn(Camera3dComponents {
                    transform: Transform::from_translation_rotation(Vec3::new(0.0, 5.0, 6.0), Quat::from_rotation_x(-30.0 * std::f32::consts::PI / 180.0)),
                    ..Default::default()
                });
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
    //commands.spawn(FlyCamera::default());
}

struct Player {
    speed: f32,
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transformation) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::W) {
            transformation.translate(Vec3::new(0.0, 0.0, -player.speed * time.delta_seconds));
        }

        if keyboard_input.pressed(KeyCode::S) {
            transformation.translate(Vec3::new(0.0, 0.0, player.speed * time.delta_seconds));
        }

        if keyboard_input.pressed(KeyCode::A) {
            transformation.translate(Vec3::new(-player.speed * time.delta_seconds, 0.0, 0.0));
        }

        if keyboard_input.pressed(KeyCode::D) {
            transformation.translate(Vec3::new(player.speed * time.delta_seconds, 0.0, 0.0));
        }
    }
}

#[derive(Default)]
struct InputState {
    keys: EventReader<KeyboardInput>,
    _cursor: EventReader<CursorMoved>,
    motion: EventReader<MouseMotion>,
    mousebtn: EventReader<MouseButtonInput>,
    _scroll: EventReader<MouseWheel>,
}

fn input_handling(
    windows: ResMut<Windows>,
    winit: ResMut<bevy::winit::WinitWindows>,
    mut state: ResMut<InputState>,
    ev_keys: Res<Events<KeyboardInput>>,
    _ev_cursor: Res<Events<CursorMoved>>,
    ev_motion: Res<Events<MouseMotion>>,
    ev_mousebtn: Res<Events<MouseButtonInput>>,
    _ev_scroll: Res<Events<MouseWheel>>,
) {
    // Keyboard input
    for ev in state.keys.iter(&ev_keys) {
        if ev.state.is_pressed() {
            match ev.key_code {
                Some(key) => match key {
                    KeyCode::Escape => {}
                    _ => {}
                },
                _ => {}
            }
        } else {
            //eprintln!("Just released key: {:?}", ev.key_code);
        }
    }

    // // Absolute cursor position (in window coordinates)
    // for ev in state.cursor.iter(&ev_cursor) {
    //     eprintln!("Cursor at: {}", ev.position);
    // }

    // // Relative mouse motion
    // for ev in state.motion.iter(&ev_motion) {
    //     eprintln!("Mouse moved {} pixels", ev.delta);
    // }

    // // Mouse buttons
    // for ev in state.mousebtn.iter(&ev_mousebtn) {
    //     if ev.state.is_pressed() {
    //         eprintln!("Just pressed mouse button: {:?}", ev.button);
    //     } else {
    //         eprintln!("Just released mouse button: {:?}", ev.button);
    //     }
    // }

    // // scrolling (mouse wheel, touchpad, etc.)
    // for ev in state.scroll.iter(&ev_scroll) {
    //     eprintln!("Scrolled vertically by {} and horizontally by {}.", ev.y, ev.x);
    // }
}
