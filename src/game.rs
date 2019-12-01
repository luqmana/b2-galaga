use components::{self, *};
use entities;
use systems;

use ggez::graphics::{Align, DrawParam, FilterMode, Font, MeshBuilder, Text, TextFragment};
use ggez::{event, graphics, timer, Context, GameResult};
use specs::{Dispatcher, DispatcherBuilder, Join, World, WorldExt};

use std::collections::HashMap;
use std::f32;

/// The entire game window width
pub const WINDOW_WIDTH: f32 = 500.;

/// The entire game window height
pub const WINDOW_HEIGHT: f32 = 600.;

/// How much of the game window width is taken up by the ui
const SIDEBAR_WIDTH: f32 = 100.;

/// The playable game area width
pub const GAME_WIDTH: f32 = WINDOW_WIDTH - SIDEBAR_WIDTH;

/// The playable game area height
pub const GAME_HEIGHT: f32 = WINDOW_HEIGHT;

/// Playable area
pub const GAME_AREA: [f32; 4] = [0., 0., GAME_WIDTH, GAME_HEIGHT];

/// Area occupied by sidebar ui
const SIDEBAR_AREA: [f32; 4] = [
    GAME_WIDTH,
    0.,
    SIDEBAR_WIDTH,
    WINDOW_HEIGHT,
];

/// Health bar
const HEALTHBAR_BG: [f32; 4] = [SIDEBAR_AREA[0] + 27., 47., 46., 206.];

/// Max player health
const MAX_PLAYER_HEALTH: f32 = 10.;

/// BG colour of sidebar ui
const SIDEBAR_COLOUR: (u8, u8, u8) = (0x55, 0x55, 0x55);

/// Our desired FPS
const DESIRED_FPS: u32 = 60;

struct UITexts {
    health_hdr: Text,
    score_hdr: Text,
    score: Text,
    game_over: Text,
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
/// Note, this doesn't necessarily mean how many frames were
/// rendered to the screen but rather how many frames were computed.
#[derive(Default)]
pub struct Frames(pub u64);

/// Player's current health
#[derive(Default)]
pub struct PlayerHealth(pub f32);

/// Player's current score
#[derive(Default)]
pub struct PlayerScore(pub u32);

/// Main game state.
pub struct Galaga<'a, 'b> {
    // Whether the game is over yet
    game_over: bool,

    // UI text items
    ui_texts: UITexts,

    // Scores that show briefly after killing a baddy
    score_popup_texts: HashMap<u32, Text>,

    // ECS world
    world: World,

    // Runs our various systems
    dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> Galaga<'a, 'b> {
    /// Create new instance of our game state
    pub fn new() -> Galaga<'a, 'b> {
        // Now let's create the various text fragments in our game
        let mut ui_texts = UITexts {
            health_hdr: Text::new("HEALTH"),
            score_hdr: Text::new("SCORE"),
            score: Text::new("000000"),
            game_over: Text::new(("GAME\nOVER", Font::default(), 80.)),
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
                [SIDEBAR_WIDTH, f32::INFINITY],
                Align::Center
            );
        }

        ui_texts.game_over.set_bounds(
            [240., f32::INFINITY],
            Align::Center
        );

        let score_popup_texts = HashMap::new();

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
        world.insert::<InputState>(Default::default());

        // Also provide frame count as resource
        world.insert::<Frames>(Default::default());

        // And player health and score
        world.insert::<PlayerHealth>(PlayerHealth(MAX_PLAYER_HEALTH));
        world.insert::<PlayerScore>(Default::default());

        // We play until health goes to 0
        let game_over = false;

        Galaga {
            game_over,
            ui_texts,
            score_popup_texts,
            world,
            dispatcher,
        }
    }

    // Draw the game's UI
    fn draw_ui(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut ui = MeshBuilder::new();

        // Set the colour and the sidebar UI bg
        ui.rectangle(graphics::DrawMode::fill(), SIDEBAR_AREA.into(), SIDEBAR_COLOUR.into());

        // The health bar BG
        ui.rectangle(graphics::DrawMode::fill(), HEALTHBAR_BG.into(), graphics::BLACK);

        // The health bar
        let health = self.world.read_resource::<PlayerHealth>();
        let lvl = 250. - 200. * health.0 / MAX_PLAYER_HEALTH;
        let health_rect = [HEALTHBAR_BG[0] + 3., lvl, 40., 250. - lvl].into();
        ui.rectangle(graphics::DrawMode::fill(), health_rect, (0x00, 0xFF, 0x00).into());

        // Queue up the text to draw
        graphics::queue_text(ctx, &self.ui_texts.health_hdr, [SIDEBAR_AREA[0], 15.], Some(graphics::WHITE));
        graphics::queue_text(ctx, &self.ui_texts.score_hdr, [SIDEBAR_AREA[0], 315.], Some(graphics::WHITE));
        graphics::queue_text(ctx, &self.ui_texts.score, [SIDEBAR_AREA[0], 335.], Some(graphics::WHITE));

        // Queue draw GAMEOVER text if needed
        if self.game_over {
            graphics::queue_text(
                ctx,
                &self.ui_texts.game_over,
                [80., 220.],
                Some((0xFF, 0x00, 0x00, 0xFF).into()),
            );
        }

        // Draw UI
        let ui = ui.build(ctx)?;
        graphics::draw(ctx, &ui, DrawParam::default())?;

        Ok(())
    }

    /// Draw all entities that should be rendered
    fn draw_entities(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Create meshes for all entities marked with Rendered
        let mut rendered_ents = MeshBuilder::new();
        {
            let rendered = self.world.read_storage::<Rendered>();

            for rendered in (&rendered).join() {
                rendered_ents.rectangle(graphics::DrawMode::fill(), rendered.area, rendered.colour.into());
            }
        }

        // Draw entities
        let rendered_ents = rendered_ents.build(ctx)?;
        graphics::draw(ctx, &rendered_ents, DrawParam::default())?;

        // Draw popup text
        self.draw_text_popups(ctx)?;

        Ok(())
    }

    /// Draw temporary popup text
    fn draw_text_popups(&mut self, ctx: &mut Context) -> GameResult<()> {
        let frames = self.world.read_resource::<Frames>();
        let ent = self.world.entities();
        let score_text = self.world.read_storage::<ScoreText>();
        let position = self.world.read_storage::<Position>();

        // Draw score text
        for (e, score_text, pos) in (&ent, &score_text, &position).join() {
            // We don't want to create a new Text every frame,
            // so we first look it up in the hashmap before just making a new one
            let text = self
                .score_popup_texts
                .entry(score_text.score)
                .or_insert_with(|| Text::new(format!("{}", score_text.score)));

            // Draw the text
            graphics::queue_text(ctx, &text, [pos.x, pos.y], Some((0x99, 0x99, 0x99).into()));

            // We only display the text for 60 frames,
            // remove it after that time
            if frames.0 > score_text.frame + 60 {
                ent.delete(e).expect("unexepected generation error");
            }
        }

        Ok(())
    }
}

/// Implmentation for our game mainloop.
impl<'a, 'b> event::EventHandler for Galaga<'a, 'b> {
    /// Called on every tick; where we handle the game logic.
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Do nothing if game is over
        if self.game_over {
            return Ok(());
        }

        while timer::check_update_time(ctx, DESIRED_FPS) {
            // Read the current score
            let score = self.world.read_resource::<PlayerScore>().0;

            // Run the systems!
            self.dispatcher.dispatch(&self.world);

            // Let any changes get reflected
            self.world.maintain();

            // Check if health has gone to 0
            let health = self.world.read_resource::<PlayerHealth>().0;
            if health <= 0. {
                self.game_over = true;
            }

            // Check if score has changed
            let new_score = self.world.read_resource::<PlayerScore>().0;
            if score != new_score {
                self.ui_texts
                    .score
                    .fragments_mut()[0] = TextFragment::new(format!("{:06}", new_score));
            }

            // Update "frame" count
            let mut frames = self.world.write_resource::<Frames>();
            frames.0 += 1;
        }

        Ok(())
    }

    /// Called after `update` to render game.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Clear the old screen
        graphics::clear(ctx, graphics::BLACK);

        // Draw all entities that should be rendered
        self.draw_entities(ctx)?;

        // Draw the UI
        self.draw_ui(ctx)?;

        // Draw any queued text
        graphics::draw_queued_text(ctx, DrawParam::default(), None, FilterMode::Linear)?;

        // Now, actually put everything onto the screen
        graphics::present(ctx)?;

        Ok(())
    }

    /// Respond to key down event
    fn key_down_event(&mut self, ctx: &mut Context, key: event::KeyCode, _: event::KeyMods, _: bool) {
        let mut input_state = self.world.write_resource::<InputState>();

        match key {
            // Quit on Escape
            event::KeyCode::Escape => event::quit(ctx),

            // Fire a projectile
            event::KeyCode::Space => input_state.shoot = true,

            // Move in some direction
            event::KeyCode::W => input_state.up = true,
            event::KeyCode::A => input_state.left = true,
            event::KeyCode::S => input_state.down = true,
            event::KeyCode::D => input_state.right = true,

            _ => {}
        }
    }

    /// Respond to key up event
    fn key_up_event(&mut self, _: &mut Context, key: event::KeyCode, _: event::KeyMods) {
        let mut input_state = self.world.write_resource::<InputState>();

        match key {
            // Stop shooting
            event::KeyCode::Space => input_state.shoot = false,

            // Stop moving in some direction
            event::KeyCode::W => input_state.up = false,
            event::KeyCode::A => input_state.left = false,
            event::KeyCode::S => input_state.down = false,
            event::KeyCode::D => input_state.right = false,

            _ => {}
        }
    }
}
