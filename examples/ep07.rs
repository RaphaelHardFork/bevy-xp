use bevy::{
    app::AppExit, asset::AssetLoader, audio::AudioLoader, prelude::*, window::PrimaryWindow,
};
use bevy_xp::{
    events::GameOver,
    resources::{EnemySpawnTimer, HighScores, Score, StarSpawnTimer},
    systems::{
        collisions::{enemy_hit_player, player_hit_star},
        exit_game, handle_game_over,
        movements::{
            confine_player_movement, enemy_movement, player_movement, update_enemy_direction,
        },
        scores::{high_scores_updated, update_high_scores, update_score},
        spawns::{
            spawn_camera, spawn_enemies, spawn_enemies_over_time, spawn_player, spawn_stars,
            spawn_stars_over_time, tick_enemy_spawn_timer, tick_star_spawn_timer,
        },
    },
    Result,
};
use rand::{random, Rng, RngCore};

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
