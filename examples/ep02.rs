use bevy::{
    app::{App, Startup},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::With,
        system::{Commands, Query, Res},
    },
    sprite::SpriteBundle,
    transform::components::Transform,
    window::{PrimaryWindow, Window},
    DefaultPlugins,
};
use bevy_xp::Result;

/// Here we only display the ball in the center of the window
/// Assets from kenney.nl
fn main() -> Result<()> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .run();

    Ok(())
}

#[derive(Component)]
struct Player {}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        },
        Player {},
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..Default::default()
    });
}
