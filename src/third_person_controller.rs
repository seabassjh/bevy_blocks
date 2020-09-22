use bevy::prelude::*;

#[derive(Bundle)]
pub struct ThirdPersonController {
	pub camera: Camera3dComponents,
	pub player: Player,
	pub transform: Transform,
}

impl Default for ThirdPersonController {
	fn default() -> Self {
		Self {
			camera: Camera3dComponents {
                transform: Transform::from_translation_rotation(Vec3::new(0.0, 5.0, 6.0), Quat::from_rotation_x(-30.0 * std::f32::consts::PI / 180.0)),
                ..Default::default()
            },
			player: Default::default(),
			transform: Default::default(),
		}
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