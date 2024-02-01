use bevy::{
    app::AppExit, asset::AssetLoader, audio::AudioLoader, prelude::*, window::PrimaryWindow,
};
use bevy_xp::Result;
use rand::{random, Rng, RngCore};

// player
const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 500.0;

// enemies
const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SPAWN_TIME: f32 = 0.25;

// stars
const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;

/// add stars, respawns time to time, update scores
fn main() -> Result<()> {
    App::new()
        // plugins (start the game)
        .add_plugins(DefaultPlugins)
        // resources
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .init_resource::<EnemySpawnTimer>()
        .init_resource::<HighScores>()
        // events
        .add_event::<GameOver>()
        // init
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_enemies)
        .add_systems(Startup, spawn_stars)
        // update
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, enemy_movement)
        .add_systems(Update, update_enemy_direction)
        .add_systems(Update, enemy_hit_player)
        .add_systems(Update, player_hit_star)
        // .add_systems(Update, confine_enemy_movement) // stick balls to the edges
        .add_systems(Update, update_score)
        .add_systems(Update, tick_star_spawn_timer)
        .add_systems(Update, tick_enemy_spawn_timer)
        .add_systems(Update, spawn_stars_over_time)
        .add_systems(Update, spawn_enemies_over_time)
        .add_systems(Update, exit_game)
        .add_systems(Update, handle_game_over)
        .add_systems(Update, update_high_scores)
        .add_systems(Update, high_scores_updated)
        // run
        .run();

    Ok(())
}

// region:			--- Component & Resources

#[derive(Component)]
struct Player {}

#[derive(Component)]
struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
struct Star {}

#[derive(Resource)]
struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Resource, Debug)]
struct HighScores {
    pub scores: Vec<(String, u32)>,
}

impl Default for HighScores {
    fn default() -> Self {
        Self { scores: Vec::new() }
    }
}

#[derive(Resource)]
struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Event)]
struct GameOver {
    pub score: u32,
}

// endregion:		--- Component & Resources

// region:			--- Spawn

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

// endregion:		--- Spawn

// region:			--- Movements

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::Q) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // bound the player x position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

fn update_enemy_direction(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed = false;

        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if translation.y < y_min || translation.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        // play soundFX
        if direction_changed {
            if rand::thread_rng().gen_bool(0.5) {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_001.ogg"),
                    ..Default::default()
                });
            } else {
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/pluck_002.ogg"),
                    ..Default::default()
                });
            }
        }
    }
}

fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0;
    let x_min = 0.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size;
    let y_min = 0.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size;

    for mut transform in enemy_query.iter_mut() {
        let mut translation = transform.translation;

        // bound the enemy x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // bound the enemy y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

// endregion:		--- Movements

// region:			--- Collisions

fn enemy_hit_player(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    mut enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;

            if distance < player_radius + enemy_radius {
                println!("Enemy hit player! GAME OVER!");
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/explosionCrunch_000.ogg"),
                    ..Default::default()
                });
                commands.entity(player_entity).despawn();
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

fn player_hit_star(
    mut commands: Commands,
    mut player_query: Query<&Transform, With<Player>>,
    mut star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);

            if distance < PLAYER_SIZE / 2.0 + STAR_SIZE / 2.0 {
                println!("Player hit star!");
                score.value += 1;
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/glass_002.ogg"),
                    ..Default::default()
                });
                commands.entity(star_entity).despawn();
            }
        }
    }
}

// endregion:		--- Collisions

fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("Score: {}", score.value.to_string())
    }
}

fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
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

fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
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

fn exit_game(keyboard_input: Res<Input<KeyCode>>, mut app_exit_event_writer: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn handle_game_over(mut game_over_event_reader: EventReader<GameOver>) {
    for event in game_over_event_reader.read() {
        println!("Your final score is: {}", event.score.to_string());
    }
}

fn update_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        high_scores.scores.push(("Player".to_string(), event.score));
    }
}

fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High scores: {:?}", high_scores);
    }
}
