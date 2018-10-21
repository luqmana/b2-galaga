use components::*;

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
