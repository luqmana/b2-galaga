use ggez::graphics::Point2;
use specs::*;


/// Registers all our components with Specs.
/// Make sure to modify this if any components
/// are added or removed.
pub fn register_components(world: &mut World) {
    world.register::<Position>();
}

/// Marks entities with a position (e.g. player)
#[derive(Clone, Component, Copy)]
#[storage(VecStorage)]
pub struct Position(pub Point2);
