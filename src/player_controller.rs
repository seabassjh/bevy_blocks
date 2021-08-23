use crate::voxel_terrain::{constants::{CHUNK_SIZE, MAX_CHUNK_HEIGHT}, generator::GenerateAtTag};
use bevy::{prelude::*, render::camera::PerspectiveProjection};
use bevy_prototype_character_controller::{
    controller::{BodyTag, CameraTag, CharacterController, HeadTag, YawTag},
    look::{LookDirection, LookEntity},
    rapier::*,
};
use bevy_rapier3d::{
    physics::{PhysicsInterpolationComponent, RapierConfiguration, RapierPhysicsPlugin},
    rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder},
};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, builder: &mut AppBuilder) {
        builder
            .init_resource::<CharacterSettings>()
            .add_startup_system(setup_player_system.system())
            .add_plugin(RapierDynamicForceCharacterControllerPlugin);
    }
}

pub struct CharacterSettings {
    pub scale: Vec3,
    pub head_scale: f32,
    pub head_yaw: f32,
    pub follow_offset: Vec3,
    pub focal_point: Vec3,
}

impl Default for CharacterSettings {
    fn default() -> Self {
        Self {
            scale: Vec3::new(0.5, 1.7, 0.3),
            head_scale: 0.2,
            head_yaw: 0.0,
            focal_point: -Vec3::unit_z(), // Relative to head
            follow_offset: Vec3::zero(),  // Relative to head
        }
    }
}

fn setup_player_system(commands: &mut Commands, character_settings: Res<CharacterSettings>) {
    let y_spawn_offset = (CHUNK_SIZE * MAX_CHUNK_HEIGHT / 2)  as f32;
    let box_y = 1.0;
    let body = commands
        .spawn((
            GlobalTransform::identity(),
            Transform::identity(),
            CharacterController::default(),
            RigidBodyBuilder::new_dynamic()
                .translation(
                    0.0,
                    0.5 * (box_y + character_settings.scale.y) + y_spawn_offset,
                    0.0,
                )
                .principal_angular_inertia(
                    bevy_rapier3d::rapier::na::Vector3::zeros(),
                    bevy_rapier3d::rapier::na::Vector3::repeat(false),
                ),
            ColliderBuilder::capsule_y(
                0.5 * character_settings.scale.y,
                0.5 * character_settings.scale.x.max(character_settings.scale.z),
            )
            .density(200.0),
            PhysicsInterpolationComponent::new(
                0.5 * (box_y + character_settings.scale.y) * Vec3::unit_y(),
                Quat::identity(),
            ),
            BodyTag,
            GenerateAtTag,
        ))
        .current_entity()
        .expect("Failed to spawn body");
    let yaw = commands
        .spawn((GlobalTransform::identity(), Transform::identity(), YawTag))
        .current_entity()
        .expect("Failed to spawn yaw");
    let head = commands
        .spawn((
            GlobalTransform::identity(),
            Transform::from_matrix(Mat4::from_scale_rotation_translation(
                Vec3::one(),
                Quat::from_rotation_y(character_settings.head_yaw),
                Vec3::new(
                    0.0,
                    0.5 * (box_y - character_settings.head_scale) + character_settings.scale.y
                        - 1.695,
                    0.0,
                ),
            )),
            HeadTag,
        ))
        .current_entity()
        .expect("Failed to spawn head");
    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::face_toward(
                character_settings.follow_offset,
                character_settings.focal_point,
                Vec3::unit_y(),
            )),
            perspective_projection: PerspectiveProjection {
                fov: std::f32::consts::PI / 3.0,
                near: 0.25,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_bundle((LookDirection::default(), CameraTag))
        .current_entity()
        .expect("Failed to spawn camera");
    let light = commands.spawn(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    }).current_entity().expect("Coudln't spawn player light");

    commands
        .insert_one(body, LookEntity(camera))
        .push_children(body, &[yaw])
        .push_children(body, &[light])
        .push_children(yaw, &[head])
        .push_children(head, &[camera]);
}
