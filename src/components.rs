use ggez::graphics::Point2;
use specs::*;

/// Marks entities with a position (e.g. player)
#[derive(Clone, Component, Copy)]
#[storage(VecStorage)]
pub struct Position(pub Point2);
