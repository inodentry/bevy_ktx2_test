use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .run()
}

fn setup(
    ass: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteBundle {
        texture: ass.load("logo_rust.ktx2"),
        transform: Transform::from_xyz(-512.0, -256.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: ass.load("logo_rust_linear.ktx2"),
        transform: Transform::from_xyz(-512.0, 256.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: ass.load("logo_rust_grey.ktx2"),
        transform: Transform::from_xyz(512.0, -256.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: ass.load("logo_rust_grey_linear.ktx2"),
        transform: Transform::from_xyz(512.0, 256.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: ass.load("logo_bevy.ktx2"),
        transform: Transform::from_xyz(0.0, -256.0, 0.0),
        ..Default::default()
    });
    commands.spawn_bundle(SpriteBundle {
        texture: ass.load("logo_bevy_linear.ktx2"),
        transform: Transform::from_xyz(0.0, 256.0, 0.0),
        ..Default::default()
    });
}
