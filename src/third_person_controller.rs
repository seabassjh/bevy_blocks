use bevy::input::mouse::*;
use bevy::prelude::*;

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .add_startup_system(setup.system())
            .init_resource::<InputState>()
            .add_system(player_movement_system.system());
    }
}

#[derive(Default)]
pub struct InputState {
    motion: EventReader<MouseMotion>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrComponents {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .with(PlayerController::default())
        .with(GlobalTransform::from_translation(Vec3::new(0.0, 0.0, 0.0)))
        .with_children(|parent| {
            parent.spawn(Camera3dComponents {
                transform: Transform {
                    translation: Vec3::new(0.0, 5.0, 6.0),
                    rotation: Quat::from_rotation_x(-30.0 * std::f32::consts::PI / 180.0),
                    scale: Vec3::one(),
                },
                ..Default::default()
            });
        });
}

#[derive(Bundle)]
struct PlayerController {
    speed: f32,
    yaw: f32,
    rotation_speed: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            speed: 10.0,
            yaw: 0.0,
            rotation_speed: 0.1,
        }
    }
}

fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::unit_z()).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = forward_vector(rotation);
    let f_flattened = Vec3::new(f.x(), 0.0, f.z()).normalize();
    f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
    // Rotate it 90 degrees to get the strafe direction
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}

fn player_movement_system(
    time: Res<Time>,
    mut state: ResMut<InputState>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut query: Query<(&mut PlayerController, &mut Transform, &mut GlobalTransform)>,
) {
    let mut delta: Vec2 = Vec2::zero();
    for event in state.motion.iter(&mouse_motion_events) {
        delta += event.delta;
    }

    for (mut player_controller, mut transform, mut _g_transform) in &mut query.iter_mut() {
        player_controller.yaw -= delta.x() * time.delta_seconds * player_controller.rotation_speed;

        let mut axis_h = 0.0;
        let mut axis_v = 0.0;

        if keyboard_input.pressed(KeyCode::W) {
            axis_v -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::S) {
            axis_v += 1.0;
        }

        if keyboard_input.pressed(KeyCode::A) {
            axis_h -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::D) {
            axis_h += 1.0;
        }

        let delta_forward = forward_walk_vector(&transform.rotation)
            * axis_v
            * player_controller.speed
            * time.delta_seconds;

        let delta_strafe = strafe_vector(&transform.rotation)
            * axis_h
            * player_controller.speed
            * time.delta_seconds;

        transform.translation += delta_forward + delta_strafe;
        transform.rotation = Quat::from_rotation_y(player_controller.yaw);
    }
}
