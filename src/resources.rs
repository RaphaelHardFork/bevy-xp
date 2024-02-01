use crate::{ENEMY_SPAWN_TIME, STAR_SPAWN_TIME};
use bevy::ecs::system::Resource;
use bevy::time::{Timer, TimerMode};

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
