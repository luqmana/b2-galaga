use ggez::graphics;
use specs::*;

/// Registers all our components with Specs.
/// Make sure to modify this if any components
/// are added or removed.
pub fn register_components(world: &mut World) {
    world.register::<Player>();
    world.register::<Position>();
    world.register::<Rendered>();
    world.register::<Velocity>();
}

/// Marks the player entity so we can control it.
#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

/// Marks entities with a position (e.g. player)
#[derive(Clone, Component, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

/// Marks entities that are to be rendered onscreen
#[derive(Component)]
#[storage(VecStorage)]
pub struct Rendered {
    pub area: graphics::Rect,
    pub colour: (u8, u8, u8),
}

/// Marks entities with a velocity
#[derive(Clone, Component, Copy)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
