use components::*;
use entities;
use game;

use specs::*;

pub struct BaddySpawner;

impl<'a> System<'a> for BaddySpawner {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>, Read<'a, game::Frames>);

    fn run(&mut self, (ent, lazy, frame): Self::SystemData) {
        // Spawn some baddies every so often

        // Noobs
        if frame.0 % 100 == 50 {
            entities::create_noob_baddy(ent.create(), &lazy);
        }
    }
}

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

/// Respond to game input and update game state as necessary
pub struct PlayerControlSystem {
    // What was the last frame where we shot a projectile
    last_shot_frame: u64,
}

impl PlayerControlSystem {
    pub fn new() -> PlayerControlSystem {
        PlayerControlSystem { last_shot_frame: 0 }
    }
}

impl<'a> System<'a> for PlayerControlSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, game::Frames>,
        Read<'a, game::InputState>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ent, lazy, frame, input, player, pos, mut vel) = data;

        for (_, pos, vel) in (&player, &pos, &mut vel).join() {
            // First zero out the player's velocity
            vel.x = 0.;
            vel.y = 0.;

            // Next read the input state and update velocities
            if input.up {
                vel.y -= 4.;
            }
            if input.down {
                vel.y += 4.;
            }
            if input.left {
                vel.x -= 4.;
            }
            if input.right {
                vel.x += 4.;
            }

            // Are we shooting projectiles?
            if input.shoot && frame.0 - self.last_shot_frame >= 10 {
                let e = ent.create();
                entities::create_player_projectile(e, *pos, &lazy);

                // Update frame reference
                self.last_shot_frame = frame.0;
            }
        }
    }
}
