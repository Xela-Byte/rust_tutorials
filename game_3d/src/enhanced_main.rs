use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "3D Game Tutorial".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (rotate_cube, move_camera, bounce_sphere, cycle_colors),
        )
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
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            metallic: 0.2,
            perceptual_roughness: 0.8,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingCube,
    ));

    // Spawn a plane (ground)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.8, 0.3),
            metallic: 0.0,
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Spawn a bouncing sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.3, 0.8),
            metallic: 0.5,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(2.0, 2.0, 0.0),
        BouncingSphere {
            velocity: Vec3::new(0.0, 0.0, 0.0),
        },
    ));

    // Spawn multiple colored cubes
    for i in 0..5 {
        let x = (i as f32 - 2.0) * 2.0;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.5, 0.5, 0.5))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb((i as f32 * 0.2) % 1.0, 0.5, ((i as f32 * 0.3) % 1.0)),
                metallic: 0.1,
                perceptual_roughness: 0.7,
                ..default()
            })),
            Transform::from_xyz(x, 0.25, -3.0),
            ColorCycler {
                timer: Timer::from_seconds(1.0 + i as f32 * 0.5, TimerMode::Repeating),
            },
        ));
    }

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 5_000_000.0,
            range: 50.0,
            color: Color::srgb(1.0, 0.9, 0.8),
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.2, 0.2, 0.3),
        brightness: 200.0,
    });

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-4.0, 6.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        CameraController,
    ));
}

#[derive(Component)]
struct RotatingCube;

#[derive(Component)]
struct CameraController;

#[derive(Component)]
struct BouncingSphere {
    velocity: Vec3,
}

#[derive(Component)]
struct ColorCycler {
    timer: Timer,
}

fn rotate_cube(mut query: Query<&mut Transform, With<RotatingCube>>, time: Res<Time>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_secs() * 0.5);
        transform.rotate_x(time.delta_secs() * 0.3);
        transform.rotate_z(time.delta_secs() * 0.1);
    }
}

fn bounce_sphere(mut query: Query<(&mut Transform, &mut BouncingSphere)>, time: Res<Time>) {
    for (mut transform, mut sphere) in &mut query {
        // Apply gravity
        sphere.velocity.y -= 9.8 * time.delta_secs();

        // Update position
        transform.translation += sphere.velocity * time.delta_secs();

        // Bounce off the ground
        if transform.translation.y <= 0.5 {
            transform.translation.y = 0.5;
            sphere.velocity.y = sphere.velocity.y.abs() * 0.8; // Damping
        }

        // Bounce off walls
        if transform.translation.x.abs() > 4.5 {
            sphere.velocity.x *= -0.8;
            transform.translation.x = transform.translation.x.signum() * 4.5;
        }
        if transform.translation.z.abs() > 4.5 {
            sphere.velocity.z *= -0.8;
            transform.translation.z = transform.translation.z.signum() * 4.5;
        }
    }
}

fn cycle_colors(
    mut query: Query<(&mut MeshMaterial3d<StandardMaterial>, &mut ColorCycler)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (material_handle, mut cycler) in &mut query {
        cycler.timer.tick(time.delta());

        if cycler.timer.just_finished() {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let hue = (time.elapsed_secs() * 0.5) % 1.0;
                material.base_color = Color::hsl(hue * 360.0, 0.8, 0.6);
            }
        }
    }
}

fn move_camera(
    mut query: Query<&mut Transform, With<CameraController>>,
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut query {
        let mut velocity = Vec3::ZERO;
        let speed = 8.0;

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
