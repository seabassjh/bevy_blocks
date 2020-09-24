use bevy::prelude::*;
use bevy::{input::mouse::*};

pub struct ThirdPersonControllerPlugin;

impl Plugin for ThirdPersonControllerPlugin {
    fn build(&self, app: &mut AppBuilder) {

	}
}

#[derive(Bundle)]
pub struct ThirdPersonController {
	pub camera: Camera3dComponents,
}

impl Default for ThirdPersonController {
	fn default() -> Self {
		Self {
			camera: Camera3dComponents {
                transform: Transform::from_translation_rotation(Vec3::new(0.0, 5.0, 6.0), Quat::from_rotation_x(-30.0 * std::f32::consts::PI / 180.0)),
                ..Default::default()
            },
		}
	}
}

impl ThirdPersonController {
    pub fn build(&mut self, builder: &mut ChildBuilder) {
        builder
            .spawn(Camera3dComponents {
            transform: Transform::from_translation_rotation(Vec3::new(0.0, 5.0, 6.0), Quat::from_rotation_x(-30.0 * std::f32::consts::PI / 180.0)),
            ..Default::default()
        });
    }
}

pub struct Player {
    speed: f32,
}

impl Default for Player {
	fn default() -> Self {
		Self {
			speed: 10.0,
		}
	}
}

pub fn player_movement_system(
    time: Res<Time>,
    mut state: ResMut<super::InputState>,
    keyboard_input: Res<Input<KeyCode>>,
	mouse_motion_events: Res<Events<MouseMotion>>,
    mut query: Query<(&Player, &mut Transform, &mut GlobalTransform)>,
) {
    let mut delta: Vec2 = Vec2::zero();
	for event in state.motion.iter(&mouse_motion_events) {
		delta += event.delta;
	}

    for (player, mut transform, mut g_transform) in &mut query.iter() {
        
        if keyboard_input.pressed(KeyCode::W) {
            transform.translate(Vec3::new(0.0, 0.0, -player.speed * time.delta_seconds));
        }
        
        if keyboard_input.pressed(KeyCode::S) {
            transform.translate(Vec3::new(0.0, 0.0, player.speed * time.delta_seconds));
        }
        
        if keyboard_input.pressed(KeyCode::A) {
            transform.translate(Vec3::new(-player.speed * time.delta_seconds, 0.0, 0.0));
        }
        
        if keyboard_input.pressed(KeyCode::D) {
            transform.translate(Vec3::new(player.speed * time.delta_seconds, 0.0, 0.0));
        }
        transform.rotate(Quat::from_rotation_y(delta.x() * time.delta_seconds));
    }
}
