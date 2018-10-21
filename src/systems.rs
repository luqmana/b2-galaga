use components::*;

use ggez::event;
use specs::*;

/// Updates entities with both a Position and Velocity
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Velocity>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.x;
            pos.y += vel.y;
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
                (true, event::Keycode::Up) => vel.y = -4.,
                (true, event::Keycode::Right) => vel.x = 4.,
                (true, event::Keycode::Down) => vel.y = 4.,
                (true, event::Keycode::Left) => vel.x = -4.,

                (false, event::Keycode::Up) => vel.y = 0.,
                (false, event::Keycode::Right) => vel.x = 0.,
                (false, event::Keycode::Down) => vel.y = 0.,
                (false, event::Keycode::Left) => vel.x = 0.,

                _ => {}
            }
        }
    }
}
