use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
    )
    .add_systems(
        Startup,
        setup
    );
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let triangle = meshes.add(Triangle2d::new(
        Vec2::new(0.0, 50.0),
        Vec2::new(-50.0, -50.0),
        Vec2::new(50.0, -50.0),
    ));
    let colour = Color::srgba_u8(255, 120, 0, 255);

    commands.spawn((Mesh2d(triangle), MeshMaterial2d(materials.add(colour))));
}