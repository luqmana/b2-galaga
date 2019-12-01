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

        // Wavers
        if frame.0 % 300 == 200 {
            entities::create_waver_baddy(ent.create(), None, &lazy);
        }
    }
}

pub struct BaddyActions;

impl<'a> System<'a> for BaddyActions {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Baddy>,
        ReadStorage<'a, NoobBaddy>,
        WriteStorage<'a, Oscillates>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, WaverBaddy>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ent, mut baddy, noob, mut oscs, pos, mut vel, waver, lazy) = data;

        // Update baddies' ages
        for baddy in (&mut baddy).join() {
            baddy.age += 1;
        }

        // Noob baddy logic
        for (_, e, baddy, oscs, pos, vel) in (&noob, &ent, &baddy, &mut oscs, &pos, &mut vel).join()
        {
            // Noob baddies oscillate some number of times in the center 60% of game area
            let xpct = pos.x / game::GAME_WIDTH;
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

            // Noob's fire some projectiles every so often
            if baddy.age % 15 == 0 {
                entities::create_noob_projectile(ent.create(), *pos, &lazy);
            }
        }

        // Waver baddy logic
        for (waver, baddy, vel) in (&waver, &baddy, &mut vel).join() {
            // If we're not the last waver, summon the rest of our wave
            if baddy.age == 15 && waver.rank > 0 {
                entities::create_waver_baddy(ent.create(), Some(*waver), &lazy);
            }

            // Decrease vertical velocity
            if baddy.age % 8 == 0 {
                vel.y -= 1.;
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
            let x_bound = game::GAME_WIDTH - entities::PLAYER_SIZE;
            let y_bound = game::GAME_HEIGHT - entities::PLAYER_SIZE;
            pos.x = pos.x.min(x_bound).max(0.);
            pos.y = pos.y.min(y_bound).max(0.);
        }

        // Update the rendered area's offset to the new position
        for (pos, rendered) in (&pos, &mut rendered).join() {
            rendered.area.move_to([pos.x, pos.y]);
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

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, game::Frames>,
        Write<'a, game::PlayerHealth>,
        Write<'a, game::PlayerScore>,
        WriteStorage<'a, Baddy>,
        ReadStorage<'a, DamageBaddy>,
        ReadStorage<'a, DamagePlayer>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Rendered>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            ent,
            lazy,
            frame,
            mut health,
            mut score,
            mut baddy,
            damage_b,
            damage_p,
            player,
            pos,
            rendered,
        ) = data;

        // Grab the player's render area
        let player_area = (&player, &rendered)
            .join()
            .map(|p| p.1.area)
            .next()
            .expect("no player rendered component?");

        // Go over all baddies and see if we hit em!
        for (b, b_pos, b_e, b_rendered) in (&mut baddy, &pos, &*ent, &rendered).join() {
            // Go over entities that can hurt baddies
            for (_, d_e, d_rendered) in (&damage_b, &*ent, &rendered).join() {
                if b_rendered.area.overlaps(&d_rendered.area) {
                    if b.health > 0 {
                        // Decrement baddy's health
                        b.health -= 1;

                        // Baddy was vanquished! Update player
                        // score and remove baddy
                        if b.health == 0 {
                            score.0 += b.score;

                            // Show little score popup
                            let e = ent.create();
                            entities::create_score_popup(e, *b_pos, b.score, frame.0, &lazy);

                            ent.delete(b_e).expect("unexpected generation error");
                        }
                    }

                    // Remove the damaging entity
                    ent.delete(d_e).expect("unexpected generation error");
                }
            }
        }

        // Go over all entities that can damage the player
        for (_, e, rendered) in (&damage_p, &*ent, &rendered).join() {
            // Ouch, we hit a baddy or projectile :(
            if rendered.area.overlaps(&player_area) {
                if health.0 > 0. {
                    // Decrement player's health
                    health.0 -= 1.;
                }

                // This baddy or projectile did its job, let it go now
                ent.delete(e).expect("unexpected generation error");
            }
        }
    }
}
