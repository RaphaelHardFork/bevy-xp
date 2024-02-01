pub mod components;
mod error;
pub mod events;
pub mod resources;
pub mod systems;

pub use self::error::{Error, Result};

// region:			--- Game constants

// player
const PLAYER_SIZE: f32 = 64.0;
const PLAYER_SPEED: f32 = 500.0;

// enemies
const NUMBER_OF_ENEMIES: usize = 4;
const ENEMY_SIZE: f32 = 64.0;
const ENEMY_SPEED: f32 = 200.0;
const ENEMY_SPAWN_TIME: f32 = 5.0;

// stars
const NUMBER_OF_STARS: usize = 10;
const STAR_SIZE: f32 = 30.0;
const STAR_SPAWN_TIME: f32 = 1.0;

// endregion:		--- Game constants
