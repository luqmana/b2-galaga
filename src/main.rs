// Hide the console when launched on Windows
// Ignored on !Windows
#![windows_subsystem = "windows"]

#[macro_use]
extern crate specs_derive;

use ggez::{conf, event, ContextBuilder, GameError};

/// The various components the entities in our game can have
mod components;

/// The various entities in the game
mod entities;

/// The systems that can act on our entities
mod systems;

/// Main game state structure and game loop
mod game;

fn main() -> Result<(), GameError> {
    // Create a new ggez Context
    let (ctx, evt_loop) = &mut ContextBuilder::new("Galaga", "Adcoba")
        .window_setup(conf::WindowSetup::default().title("Galaga"))
        .window_mode(
            conf::WindowMode::default().dimensions(game::WINDOW_WIDTH, game::WINDOW_HEIGHT),
        ).build()?;

    // Create our main game state
    let state = &mut game::Galaga::new();

    // Kick off the main loop
    event::run(ctx, evt_loop, state)
}
