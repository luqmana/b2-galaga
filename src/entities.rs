use components;
use game;

use specs::*;

/// Creates the player entity and registers it with our world
pub fn create_player(world: &mut World) {
    // The player has a position and starts out
    // at the center of the game area
    let pos = components::Position {
        x: game::GAME_WIDTH as f32 / 2.,
        y: game::GAME_HEIGHT as f32 / 2.,
    };

    // It also has a velocity that starts off at 0
    let vel = components::Velocity { x: 0., y: 0. };

    // Actually mark this entity as the player so
    // we can control it
    let player = components::Player;

    world
        .create_entity()
        .with(player)
        .with(pos)
        .with(vel)
        .build();
}
