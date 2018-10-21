use specs::*;

/// Registers all our components with Specs.
/// Make sure to modify this if any components
/// are added or removed.
pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
}

/// Marks entities with a position (e.g. player)
#[derive(Clone, Component, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Marks entities with a velocity
#[derive(Clone, Component, Copy)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
