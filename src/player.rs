use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::physics::ColliderPositionSync;
use bevy_rapier3d::prelude::{
    ColliderBundle, ColliderParentComponent, ColliderPositionComponent, ColliderShape,
    MassProperties, RigidBodyBundle, RigidBodyMassProps, RigidBodyPositionComponent,
    RigidBodyVelocity, RigidBodyVelocityComponent,
};
use rand::Rng;

use crate::actions::Actions;
use crate::GameState;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Dice;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_arena)
                .with_system(spawn_dice)
                .with_system(setup_camera_and_light),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_object));
    }
}

fn setup_camera_and_light(mut commands: Commands) {
    // camera
    let mut camera = OrthographicCameraBundle::new_3d();
    camera.orthographic_projection.scale = 5.0;
    camera.transform = Transform::from_xyz(0.0, 10.0, 0.0).looking_at(Vec3::ZERO, Vec3::X);
    commands.spawn_bundle(camera);

    // ambient light
    commands.insert_resource(AmbientLight { color: Color::WHITE, brightness: 0.1 });

    // point light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(5.0, 6.0, -2.0),
        point_light: PointLight {
            intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::rgb(0.7, 0.7, 0.7),
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}

fn spawn_arena(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let size = 5.0;

    let pbr = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    };
    let floor = ColliderBundle {
        shape: ColliderShape::cuboid(size / 2.0, 0.1, size / 2.0).into(),
        position: Vec3::new(0.0, -0.2, 0.0).into(),
        ..Default::default()
    };

    // spawn the floor
    commands.spawn().insert_bundle(pbr).insert_bundle(floor);

    // let size = 100.0;
    // let pbr = PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size })),
    //     material: materials.add(Color::rgb(1.0, 0.7, 0.6).into()),
    //     ..Default::default()
    // };
    // let north = ColliderBundle {
    //     shape: ColliderShape::cuboid(0.1, size / 2.0, size / 2.0).into(),
    //     position: Vec3::new(0.0, 0.0, 0.0).into(),
    //     ..Default::default()
    // };

    // // spawn the north
    // commands.spawn().insert_bundle(pbr).insert_bundle(north).insert(ColliderPositionSync::Discrete);
}

fn spawn_dice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        // Build the rigid body
        let mut rigid_body = RigidBodyBundle {
            position: (random_init_position(&mut rng), random_quat(&mut rng)).into(),
            ..Default::default()
        };

        // cube size
        let size = 0.5;

        // Build the pbr body
        let pbr_body = PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size })),
            material: materials.add(Color::rgb(0.95, 0.95, 0.95).into()),
            ..Default::default()
        };

        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(size, size, size).into(),
            ..ColliderBundle::default()
        };

        commands
            .spawn()
            .insert_bundle(rigid_body)
            .insert_bundle(pbr_body)
            .insert_bundle(collider)
            .insert(ColliderPositionSync::Discrete)
            .insert(Dice);
    }
}

fn move_object(
    actions: Res<Actions>,
    mut dices_query: Query<
        (&mut RigidBodyPositionComponent, &mut RigidBodyVelocityComponent),
        With<Dice>,
    >,
) {
    if actions.reroll {
        let mut rng = rand::thread_rng();
        for (mut rb_pos, mut rb_vel) in dices_query.iter_mut() {
            rb_pos.position.translation = random_init_position(&mut rng).into();
            rb_pos.position.rotation = random_quat(&mut rng).into();
            rb_vel.0 = RigidBodyVelocity::zero();
        }
    }
}

fn random_quat<R: Rng>(rng: &mut R) -> Quat {
    let rad_max = 2.0 * PI;
    let qx = Quat::from_rotation_x(rng.gen_range(0.0..=rad_max));
    let qy = Quat::from_rotation_y(rng.gen_range(0.0..=rad_max));
    let qz = Quat::from_rotation_z(rng.gen_range(0.0..=rad_max));
    qx * qy * qz
}

fn random_init_position<R: Rng>(rng: &mut R) -> Vec3 {
    [rng.gen_range(-5.0..=5.0), rng.gen_range(4.5..=5.5), rng.gen_range(-5.0..=5.0)].into()
}
