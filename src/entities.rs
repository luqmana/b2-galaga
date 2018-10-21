use components;
use game;

use rand::{self, Rng};
use specs::*;

// Size of player square
pub const PLAYER_SIZE: f32 = 20.;

// Size of Noob baddy square
pub const NOOB_SIZE: f32 = 20.;

// Size of Wave baddy square
pub const WAVER_SIZE: f32 = 10.;

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
    let rendered = components::Rendered {
        area: [pos.x, pos.y, PLAYER_SIZE, PLAYER_SIZE].into(),
        colour: (0xAA, 0xAA, 0xAA),
    };

    world
        .create_entity()
        .with(player)
        .with(pos)
        .with(vel)
        .with(rendered)
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

    // Set the projectile's size and colour
    let rendered = components::Rendered {
        area: [pos.x, pos.y, PLAYER_PROJ_WIDTH, PLAYER_PROJ_HEIGHT].into(),
        colour: (0x00, 0x00, 0xFF),
    };

    update.insert(e, pos);
    update.insert(e, vel);
    update.insert(e, rendered);
}

/// Creates a new `Noob` baddy
pub fn create_noob_baddy(e: Entity, update: &LazyUpdate) {
    let mut rng = rand::thread_rng();

    // Mark it as a Noob
    let noob = components::NoobBaddy;

    // Choose the Noob's starting position
    let pos = components::Position {
        x: if rng.gen::<bool>() {
            1. - NOOB_SIZE
        } else {
            game::GAME_WIDTH as f32 - 1.
        },
        y: rng.gen_range(0., 300.),
    };

    // Noobs only move side to side
    let vel = components::Velocity {
        x: rng.gen_range(1u8, 4) as f32,
        y: 0.,
    };

    // Set the noob's size and colour
    let rendered = components::Rendered {
        area: [pos.x, pos.y, NOOB_SIZE, NOOB_SIZE].into(),
        colour: (0xDD, 0x66, 0x33),
    };

    // Set how many times it oscillates
    let oscs = components::Oscillates(rng.gen_range(1, 4));

    update.insert(e, noob);
    update.insert(e, pos);
    update.insert(e, vel);
    update.insert(e, rendered);
    update.insert(e, oscs);
}

/// Creates a new `Waver` baddy
pub fn create_waver_baddy(e: Entity, base: Option<components::WaverBaddy>, update: &LazyUpdate) {
    let mut rng = rand::thread_rng();

    let start_left = rng.gen::<bool>();

    // Choose the Waver's starting position and velocity
    // (possibly using the base passed in)
    let (rank, pos, vel) = base.map(|w| (w.rank - 1, w.pos, w.vel)).unwrap_or_else(|| {
        (
            10,
            components::Position {
                x: if start_left {
                    1. - WAVER_SIZE
                } else {
                    game::GAME_WIDTH as f32 - 1.
                },
                y: rng.gen_range(50u8, 149) as f32,
            },
            components::Velocity {
                x: if start_left { 4. } else { -4. },
                y: 8.,
            },
        )
    });

    // Set the Waver's size and colour
    let rendered = components::Rendered {
        area: [pos.x, pos.y, WAVER_SIZE, WAVER_SIZE].into(),
        colour: (0xFF, 0x00, 0xFF),
    };

    // Set Waver's age
    let age = components::BaddyAge(0);

    // Mark it as a Waver
    let waver = components::WaverBaddy { rank, pos, vel };

    update.insert(e, pos);
    update.insert(e, vel);
    update.insert(e, rendered);
    update.insert(e, age);
    update.insert(e, waver);
}
