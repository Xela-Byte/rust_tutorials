#!/bin/bash

# Script to switch between basic and enhanced versions of the 3D game

if [ "$1" = "enhanced" ]; then
    echo "Switching to enhanced version..."
    cp src/enhanced_main.rs src/main.rs
    echo "Enhanced version activated!"
elif [ "$1" = "basic" ]; then
    echo "Switching to basic version..."
    git checkout HEAD -- src/main.rs 2>/dev/null || echo "Creating basic version..."
    cat > src/main.rs << 'EOF'
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_cube, move_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn a cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingCube,
    ));

    // Spawn a plane (ground)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(0.6, 0.3, 0.8))),
        Transform::from_xyz(2.0, 0.5, 0.0),
    ));

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController,
    ));
}

#[derive(Component)]
struct RotatingCube;

#[derive(Component)]
struct CameraController;

fn rotate_cube(mut query: Query<&mut Transform, With<RotatingCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
        transform.rotate_x(time.delta_secs() * 0.3);
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<CameraController>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut query {
        let mut velocity = Vec3::ZERO;
        let speed = 5.0;

        if input.pressed(KeyCode::KeyW) {
            velocity += *transform.forward();
        }
        if input.pressed(KeyCode::KeyS) {
            velocity += *transform.back();
        }
        if input.pressed(KeyCode::KeyA) {
            velocity += *transform.left();
        }
        if input.pressed(KeyCode::KeyD) {
            velocity += *transform.right();
        }
        if input.pressed(KeyCode::Space) {
            velocity += *transform.up();
        }
        if input.pressed(KeyCode::ShiftLeft) {
            velocity += *transform.down();
        }

        transform.translation += velocity * speed * time.delta_secs();
    }
}
EOF
    echo "Basic version activated!"
else
    echo "Usage: $0 [basic|enhanced]"
    echo "  basic    - Switch to the basic 3D game version"
    echo "  enhanced - Switch to the enhanced 3D game version with physics and effects"
fi
