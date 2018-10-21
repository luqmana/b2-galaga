use components::{self, *};
use entities;
use systems;

use ggez::graphics::{HorizontalAlign, Layout, Point2, TextCached};
use ggez::{event, graphics, Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, Join, World};

use std::f32;

/// The entire game window width
pub const WINDOW_WIDTH: u32 = 500;

/// The entire game window height
pub const WINDOW_HEIGHT: u32 = 600;

/// How much of the game window width is taken up by the ui
const SIDEBAR_WIDTH: u32 = 100;

/// The playable game area width
pub const GAME_WIDTH: u32 = WINDOW_WIDTH - SIDEBAR_WIDTH;

/// The playable game area height
pub const GAME_HEIGHT: u32 = WINDOW_HEIGHT;

// Area occupied by sidebar ui
const SIDEBAR_AREA: [f32; 4] = [
    GAME_WIDTH as f32,
    0.,
    SIDEBAR_WIDTH as f32,
    WINDOW_HEIGHT as f32,
];

/// Health bar
const HEALTHBAR_BG: [f32; 4] = [SIDEBAR_AREA[0] + 27., 47., 46., 206.];

// BG colour of sidebar ui
const SIDEBAR_COLOUR: (u8, u8, u8) = (0x55, 0x55, 0x55);

struct UITexts {
    health_hdr: TextCached,
    score_hdr: TextCached,
    score: TextCached,
}

/// Represents current state of the input
/// keys. (i.e. are they currently being pressed)
#[derive(Default)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

/// Main game state.
pub struct Galaga<'a, 'b> {
    // UI text items
    ui_texts: UITexts,

    // ECS world
    world: World,

    // Runs our various systems
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Galaga<'a, 'b> {
    /// Create new instance of our game state
    pub fn new(ctx: &mut Context) -> GameResult<Galaga<'a, 'b>> {
        // Let's set the background colour to black
        graphics::set_background_color(ctx, graphics::BLACK);

        // Now let's create the various text fragments in our game
        let mut ui_texts = UITexts {
            health_hdr: TextCached::new("HEALTH")?,
            score_hdr: TextCached::new("SCORE")?,
            score: TextCached::new("000000")?,
        };

        // Center the text in the sidebar by setting the width to
        // the width of the entire sidebar and telling it to align
        // horizontally center.
        for txt in [
            &mut ui_texts.health_hdr,
            &mut ui_texts.score_hdr,
            &mut ui_texts.score,
        ]
            .iter_mut()
        {
            txt.set_bounds(
                Point2::new(SIDEBAR_WIDTH as f32, f32::INFINITY),
                Some(Layout::default().h_align(HorizontalAlign::Center)),
            );
        }

        // Let's setup our ECS
        let mut world = World::new();

        // Register our components
        components::register_components(&mut world);

        // Create our player entity
        entities::create_player(&mut world);

        // Register our systems
        let dispatcher = DispatcherBuilder::new()
            .with(systems::PlayerControlSystem, "control", &[])
            .with(systems::MovementSystem, "movement", &["control"])
            .build();

        // Initialize input state and provide it as resource
        // to be read by any system
        world.add_resource::<InputState>(Default::default());

        Ok(Galaga {
            ui_texts,
            world,
            dispatcher,
        })
    }

    // Draw the game's UI
    fn draw_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Set the colour and draw the sidebar ui bg
        graphics::set_color(ctx, SIDEBAR_COLOUR.into())?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, SIDEBAR_AREA.into())?;

        // Queue up the text to draw
        self.ui_texts.health_hdr.queue(
            ctx,
            Point2::new(SIDEBAR_AREA[0], 15.),
            Some(graphics::WHITE),
        );
        self.ui_texts.score_hdr.queue(
            ctx,
            Point2::new(SIDEBAR_AREA[0], 315.),
            Some(graphics::WHITE),
        );
        self.ui_texts.score.queue(
            ctx,
            Point2::new(SIDEBAR_AREA[0], 335.),
            Some(graphics::WHITE),
        );

        // Draw all queued text
        TextCached::draw_queued(ctx, graphics::DrawParam::default())?;

        // Draw the health bar
        graphics::set_color(ctx, graphics::BLACK)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, HEALTHBAR_BG.into())?;

        Ok(())
    }

    /// Draw all entities w/ Position
    fn draw_entities(&mut self, ctx: &mut Context) -> GameResult<()> {
        let position = self.world.read_storage::<Position>();
        let look = self.world.read_storage::<Look>();

        for (pos, look) in (&position, &look).join() {
            let rect = graphics::Rect::new(pos.x, pos.y, look.width, look.height);
            graphics::set_color(ctx, look.colour.into())?;
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?;
        }

        Ok(())
    }
}

/// Implmentation for our game mainloop.
impl<'a, 'b> event::EventHandler for Galaga<'a, 'b> {
    /// Called on every tick; where we handle the game logic.
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        // Run the systems!
        self.dispatcher.dispatch(&self.world.res);

        Ok(())
    }

    /// Called after `update` to render game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the old screen
        graphics::clear(ctx);

        // Draw the UI
        self.draw_ui(ctx)?;

        // Draw all entities w/ Position
        self.draw_entities(ctx)?;

        // Now, actually put everything onto the screen
        graphics::present(ctx);

        Ok(())
    }

    /// Respond to key down event
    fn key_down_event(&mut self, ctx: &mut Context, key: event::Keycode, _: event::Mod, _: bool) {
        let mut input_state = self.world.write_resource::<InputState>();

        match key {
            // Quit on Escape
            event::Keycode::Escape => ctx.quit().expect("Failed to exit somehow?"),

            // Fire a projectile
            //event::Keycode::Space => entities::create_player_projectile(&mut self.world),

            // Update the input state
            event::Keycode::W => input_state.up = true,
            event::Keycode::A => input_state.left = true,
            event::Keycode::S => input_state.down = true,
            event::Keycode::D => input_state.right = true,

            _ => {}
        }
    }

    /// Respond to key up event
    fn key_up_event(&mut self, _: &mut Context, key: event::Keycode, _: event::Mod, _: bool) {
        let mut input_state = self.world.write_resource::<InputState>();

        match key {
            // Update the input state
            event::Keycode::W => input_state.up = false,
            event::Keycode::A => input_state.left = false,
            event::Keycode::S => input_state.down = false,
            event::Keycode::D => input_state.right = false,

            _ => {}
        }
    }
}
