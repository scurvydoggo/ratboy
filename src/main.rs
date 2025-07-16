use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_systems(Startup, setup_physics)
        .add_systems(Update, print_block_altitude)
        .run();
}

fn setup_physics(mut commands: Commands) {
    // Create the ground
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(Transform::from_xyz(0.0, -100.0, 0.0));

    // Create a falling block
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::cuboid(13.0, 20.0))
        .insert(Restitution::coefficient(0.7))
        .insert(Transform::from_xyz(0.0, 400.0, 0.0));
}

fn print_block_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Block altitude: {}", transform.translation.y);
    }
}
