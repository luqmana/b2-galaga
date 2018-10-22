use components::{self, *};
use entities;
use systems;

use ggez::graphics::{Font, HorizontalAlign, Layout, Point2, Scale, TextCached};
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

/// Playable area
pub const GAME_AREA: [f32; 4] = [0., 0., GAME_WIDTH as f32, GAME_HEIGHT as f32];

/// Area occupied by sidebar ui
const SIDEBAR_AREA: [f32; 4] = [
    GAME_WIDTH as f32,
    0.,
    SIDEBAR_WIDTH as f32,
    WINDOW_HEIGHT as f32,
];

/// Health bar
const HEALTHBAR_BG: [f32; 4] = [SIDEBAR_AREA[0] + 27., 47., 46., 206.];

/// Max player health
const MAX_PLAYER_HEALTH: f32 = 10.;

/// BG colour of sidebar ui
const SIDEBAR_COLOUR: (u8, u8, u8) = (0x55, 0x55, 0x55);

struct UITexts {
    health_hdr: TextCached,
    score_hdr: TextCached,
    score: TextCached,
    game_over: TextCached,
}

/// Represents current state of the input
/// keys. (i.e. are they currently being pressed)
#[derive(Default)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub shoot: bool,
}

/// How many frames have elapsed
#[derive(Default)]
pub struct Frames(pub u64);

/// Player's current health
#[derive(Default)]
pub struct PlayerHealth(pub f32);

/// Main game state.
pub struct Galaga<'a, 'b> {
    // Whether the game is over yet
    game_over: bool,

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
            game_over: TextCached::new(("GAME\nOVER", Font::default_font()?, Scale::uniform(80.)))?,
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

        ui_texts.game_over.set_bounds(
            [240., f32::INFINITY].into(),
            Some(Layout::default().h_align(HorizontalAlign::Center)),
        );

        // Let's setup our ECS
        let mut world = World::new();

        // Register our components
        components::register_components(&mut world);

        // Create our player entity
        entities::create_player(&mut world);

        // Register our systems
        let dispatcher = DispatcherBuilder::new()
            .with(systems::BaddySpawner, "baddy_spawner", &[])
            .with(systems::BaddyActions, "baddy_actions", &[])
            .with(systems::PlayerControlSystem::new(), "control", &[])
            .with(
                systems::MovementSystem,
                "movement",
                &["baddy_actions", "control"],
            ).with(systems::CollisionSystem, "collision", &["movement"])
            .build();

        // Initialize input state and provide it as resource
        // to be read by any system
        world.add_resource::<InputState>(Default::default());

        // Also provide frame count as resource
        world.add_resource::<Frames>(Default::default());

        // And player health
        world.add_resource::<PlayerHealth>(PlayerHealth(MAX_PLAYER_HEALTH));

        // We play until health goes to 0
        let game_over = false;

        Ok(Galaga {
            game_over,
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

        // Draw the health bar BG
        graphics::set_color(ctx, graphics::BLACK)?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, HEALTHBAR_BG.into())?;

        // Draw the health bar
        let health = self.world.read_resource::<PlayerHealth>();
        let lvl = 250. - 200. * health.0 / MAX_PLAYER_HEALTH;
        let health_rect = [HEALTHBAR_BG[0] + 3., lvl, 40., 250. - lvl].into();
        graphics::set_color(ctx, (0x00, 0xFF, 0x00).into())?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, health_rect)?;

        // Draw GAMEOVER text
        if self.game_over {
            self.ui_texts.game_over.queue(
                ctx,
                [80., 220.].into(),
                Some((0xFF, 0x00, 0x00, 0xFF).into()),
            );
            TextCached::draw_queued(ctx, graphics::DrawParam::default())?;
        }

        Ok(())
    }

    /// Draw all entities that should be rendered
    fn draw_entities(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rendered = self.world.read_storage::<Rendered>();

        for rendered in (&rendered).join() {
            graphics::set_color(ctx, rendered.colour.into())?;
            graphics::rectangle(ctx, graphics::DrawMode::Fill, rendered.area)?;
        }

        Ok(())
    }
}

/// Implmentation for our game mainloop.
impl<'a, 'b> event::EventHandler for Galaga<'a, 'b> {
    /// Called on every tick; where we handle the game logic.
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        // Do nothing if game is over
        if self.game_over {
            return Ok(());
        }

        // Run the systems!
        self.dispatcher.dispatch(&self.world.res);

        // Let any changes get reflected
        self.world.maintain();

        // Check if health has gone to 0
        let health = self.world.read_resource::<PlayerHealth>();
        if health.0 <= 0. {
            self.game_over = true;
        }

        Ok(())
    }

    /// Called after `update` to render game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the old screen
        graphics::clear(ctx);

        // Draw all entities that should be rendered
        self.draw_entities(ctx)?;

        // Draw the UI
        self.draw_ui(ctx)?;

        // Now, actually put everything onto the screen
        graphics::present(ctx);

        // Update frame count
        let mut frames = self.world.write_resource::<Frames>();
        frames.0 += 1;

        Ok(())
    }

    /// Respond to key down event
    fn key_down_event(&mut self, ctx: &mut Context, key: event::Keycode, _: event::Mod, _: bool) {
        let mut input_state = self.world.write_resource::<InputState>();

        match key {
            // Quit on Escape
            event::Keycode::Escape => ctx.quit().expect("Failed to exit somehow?"),

            // Fire a projectile
            event::Keycode::Space => input_state.shoot = true,

            // Move in some direction
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
            // Stop shooting
            event::Keycode::Space => input_state.shoot = false,

            // Stop moving in some direction
            event::Keycode::W => input_state.up = false,
            event::Keycode::A => input_state.left = false,
            event::Keycode::S => input_state.down = false,
            event::Keycode::D => input_state.right = false,

            _ => {}
        }
    }
}
