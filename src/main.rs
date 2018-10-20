extern crate ggez;

use ggez::{conf, event, ContextBuilder, GameError};

/// Main game state structure and game loop
mod game;

fn main() -> Result<(), GameError> {
    // Create a new ggez Context
    let ctx = &mut ContextBuilder::new("Galaga", "Adcoba")
        .window_setup(conf::WindowSetup::default().title("Galaga"))
        .window_mode(conf::WindowMode::default().dimensions(game::GAME_WIDTH, game::GAME_HEIGHT))
        .build()?;

    // Create our main game state
    let state = &mut game::Galaga::new(ctx)?;

    // Kick off the main loop
    event::run(ctx, state)
}
