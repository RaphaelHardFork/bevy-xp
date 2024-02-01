use bevy::ecs::event::Event;

#[derive(Event)]
struct GameOver {
    pub score: u32,
}
