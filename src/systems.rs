use components::*;
use entities;
use game;

use ggez::event;
use specs::*;

/// Updates entities with both a Position and Velocity
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (player, mut pos, vel): Self::SystemData) {
        // Update entities' positions using their velocities'
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }

        // But make sure the player stays in bounds
        if let Some((pos, _)) = (&mut pos, &player).join().next() {
            let x_bound = game::GAME_WIDTH as f32 - entities::PLAYER_SIZE;
            let y_bound = game::GAME_HEIGHT as f32 - entities::PLAYER_SIZE;
            pos.x = pos.x.min(x_bound).max(0.);
            pos.y = pos.y.min(y_bound).max(0.);
        }
    }
}

/// Update player's velocity in response to input
pub struct PlayerControlSystem {
    /// The key that was pressed/released
    key: event::Keycode,
    /// If the key is pressed or released
    down: bool,
}

impl PlayerControlSystem {
    pub fn new(key: event::Keycode, down: bool) -> PlayerControlSystem {
        PlayerControlSystem { key, down }
    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (player, mut vel): Self::SystemData) {
        for (_, vel) in (&player, &mut vel).join() {
            match (self.down, self.key) {
                (true, event::Keycode::W) => vel.y = -4.,
                (true, event::Keycode::D) => vel.x = 4.,
                (true, event::Keycode::S) => vel.y = 4.,
                (true, event::Keycode::A) => vel.x = -4.,

                (false, event::Keycode::W) => vel.y = 0.,
                (false, event::Keycode::D) => vel.x = 0.,
                (false, event::Keycode::S) => vel.y = 0.,
                (false, event::Keycode::A) => vel.x = 0.,

                _ => {}
            }
        }
    }
}
