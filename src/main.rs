use bevy::{input::keyboard::*, input::mouse::*, winit::WinitWindows, prelude::*};
use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};
use winit::dpi::LogicalPosition;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(FlyCameraPlugin)
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
            translation: Translation::new(0.0, 1.0, 0.0),
            ..Default::default()
        })
        .with(Player { speed: 10.0 })
        // sphere
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })),
            material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
            translation: Translation::new(1.5, 1.5, 1.5),
            ..Default::default()
        })
        // light
        .spawn(LightComponents {
            translation: Translation::new(4.0, 8.0, 4.0),
            ..Default::default()
        });
    // camera
    commands.spawn(FlyCamera::default());
}

struct Player {
    speed: f32,
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Translation)>,
) {
    for (player, mut translation) in &mut query.iter() {
        if keyboard_input.pressed(KeyCode::W) {
            *translation.0.x_mut() += player.speed * time.delta_seconds;
        }

        if keyboard_input.pressed(KeyCode::S) {
            *translation.0.x_mut() -= player.speed * time.delta_seconds;
        }

        if keyboard_input.pressed(KeyCode::A) {
            *translation.0.z_mut() -= player.speed * time.delta_seconds;
        }

        if keyboard_input.pressed(KeyCode::D) {
            *translation.0.z_mut() += player.speed * time.delta_seconds;
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
                    KeyCode::Escape => {
                        match windows.get_primary() {
                            Some(bevy_window) => {
                                match winit.get_window(bevy_window.id) {
                                    Some(window) => { 
                                        //window.set_cursor_position(winit::dpi::Position::Physical(winit::dpi::PhysicalPosition::new((window.inner_size().width / 2) as i32, (window.inner_size().height / 2) as i32)));
                                        window.set_cursor_visible(false);
                                    },
                                    
                                    None => {}
                                }
                            }
                            None => { eprintln!("ERROR: cant get primary window"); }
                        }
                    }
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
