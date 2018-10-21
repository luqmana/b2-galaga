use components;
use game;

use specs::*;

/// Creates the player entity and registers it with our world
pub fn create_player(world: &mut World) {
    // The player starts off in the center of the screen
    let pos = components::Position {
        x: game::GAME_WIDTH as f32 / 2.,
        y: game::GAME_HEIGHT as f32 / 2.,
    };

    world.create_entity().with(pos).build();
}
