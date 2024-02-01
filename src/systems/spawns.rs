use crate::components::{Enemy, Player, Star};
use crate::{NUMBER_OF_ENEMIES, NUMBER_OF_STARS};
use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

pub fn spawn_player(
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

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let x_random = random::<f32>() * window.width();
        let y_random = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_random, y_random, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..Default::default()
            },
            Enemy {
                direction: Vec2::new(x_random, y_random).normalize(),
            },
        ));
    }
}

fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_STARS {
        let x_random = random::<f32>() * window.width();
        let y_random = random::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_random, y_random, 0.0),
                texture: asset_server.load("sprites/star.png"),
                ..Default::default()
            },
            Star {},
        ));
    }
}
