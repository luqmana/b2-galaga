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

pub struct BaddyActions;

impl<'a> System<'a> for BaddyActions {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Oscillates>,
        ReadStorage<'a, NoobBaddy>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ent, mut oscs, noob, pos, mut vel, lazy) = data;

        // Noob baddies oscillate some number of times in the center 60% of game area
        for (_, e, oscs, pos, vel) in (&noob, &ent, &mut oscs, &pos, &mut vel).join() {
            let xpct = pos.x / game::GAME_WIDTH as f32;
            let dir = vel.x.is_sign_positive();

            // Swap directions
            if xpct < 0.2 {
                vel.x = vel.x.abs();
            }
            if xpct > 0.8 {
                vel.x = -1.0 * vel.x.abs();
            }

            // Update oscillation count if we swapped directions
            if dir != vel.x.is_sign_positive() {
                // Decrement oscillation count
                oscs.0 -= 1;

                // If that was the last oscillation, remove that
                // component from this Noob baddy
                if oscs.0 == 0 {
                    lazy.remove::<Oscillates>(e);
                }
            }
        }
    }
}

/// Updates entities with both a Position and Velocity
pub struct MovementSystem;

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Rendered>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, (ent, player, mut pos, mut rendered, vel): Self::SystemData) {
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

        // Update the rendered area's offset to the new position
        for (pos, rendered) in (&pos, &mut rendered).join() {
            rendered.area.move_to([pos.x, pos.y].into());
        }

        // Delete any out of bound entity
        for (e, rendered) in (&*ent, &rendered).join() {
            if !rendered.area.overlaps(&game::GAME_AREA.into()) {
                ent.delete(e).expect("unexpected generation error");
            }
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
