use bevy::prelude::*;
use bevy::window::{CursorGrabMode, CursorOptions};
use bevy::input::mouse::AccumulatedMouseMotion;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (scene.spawn(), setup, grab_cursor))
        .add_systems(Update, (move_player, mouse_look, release_cursor))
        .run();
}

/// set up a simple 3D scene
fn scene() -> impl SceneList {
    bsn_list! {
        #Cube
        Mesh3d(asset_value(Cuboid::new(1.0, 1.0, 1.0)))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::srgb_u8(124, 144, 255)))
        Transform::from_xyz(0.0, 0.5, 0.0)
        --
        PointLight {
            shadow_maps_enabled: true,
        }
        Transform::from_xyz(4.0, 8.0, 4.0)
    }
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands) {
    commands.spawn((
        Player,
        Transform::from_xyz(0.0, 0.5, 0.0),
        children![(
            Camera3d::default(),
            Transform::from_xyz(0.0, 0.7, 0.0), // eye height, looking straight ahead (-Z)
        )],
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut Transform, With<Player>>,
) {
    // Relative to where the player faces (only the player yaws, so these stay flat)
    let forward = player.forward().as_vec3();
    let right = player.right().as_vec3();

    let mut direction = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) { direction += forward; }
    if keys.pressed(KeyCode::KeyS) { direction -= forward; }
    if keys.pressed(KeyCode::KeyA) { direction -= right; }
    if keys.pressed(KeyCode::KeyD) { direction += right; }

    let speed = 5.0; // units per second
    player.translation += direction.normalize_or_zero() * speed * time.delta_secs();
}

fn grab_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

// Esc gives the cursor back, so you can reach the window's close button
fn release_cursor(keys: Res<ButtonInput<KeyCode>>, mut cursor: Single<&mut CursorOptions>) {
    if keys.just_pressed(KeyCode::Escape) {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    }
}

fn mouse_look(
    mouse: Res<AccumulatedMouseMotion>,
    mut player: Single<&mut Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    let delta = mouse.delta;       // how far the mouse moved this frame, in pixels
    if delta == Vec2::ZERO { return; }

    let sensitivity = 0.002;

    // Yaw: turn the whole player, so movement directions turn too
    player.rotate_y(-delta.x * sensitivity);

    // Pitch: tilt only the camera, clamped so you can't flip upside down
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let pitch = (pitch - delta.y * sensitivity).clamp(-1.54, 1.54); // about ±88°
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
}