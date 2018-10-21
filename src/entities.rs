use components;
use game;

use specs::*;

// Size of player square
pub const PLAYER_SIZE: f32 = 20.;

// Width of player's projectile
pub const PLAYER_PROJ_WIDTH: f32 = 4.;
// Height of player's projectile
pub const PLAYER_PROJ_HEIGHT: f32 = 8.;

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

    // The player is also visible on screen
    let look = components::Look {
        width: PLAYER_SIZE,
        height: PLAYER_SIZE,
        colour: (0xAA, 0xAA, 0xAA),
    };

    world
        .create_entity()
        .with(player)
        .with(pos)
        .with(vel)
        .with(look)
        .build();
}

/// Create a projectile entity shot by the player
pub fn create_player_projectile(e: Entity, p_pos: components::Position, update: &LazyUpdate) {
    // Set projectile's position based on player's position
    let pos = components::Position {
        x: p_pos.x + PLAYER_SIZE / 2. - PLAYER_PROJ_WIDTH / 2.,
        y: p_pos.y - PLAYER_PROJ_HEIGHT,
    };

    // Set the projectile's velocity
    let vel = components::Velocity { x: 0., y: -8. };

    // Set the projectile's look
    let look = components::Look {
        width: PLAYER_PROJ_WIDTH,
        height: PLAYER_PROJ_HEIGHT,
        colour: (0x00, 0x00, 0xFF),
    };

    update.insert(e, pos);
    update.insert(e, vel);
    update.insert(e, look);
}
