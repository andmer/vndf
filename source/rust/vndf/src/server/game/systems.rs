use nalgebra::{
    Norm,
    Rot2,
    Rotate,
    Vec1,
    Vec2,
};

use server::game::state::GameState;


pub fn apply_maneuvers(game_state: &mut GameState, now_s: f64) {
    for (&id, maneuver) in &mut game_state.entities.maneuvers {
        if now_s >= maneuver.data.start_s {
            let thrust = match maneuver.data.thrust {
                thrust if thrust > 1.0 => 1.0,
                thrust if thrust < 0.0 => 0.0,

                thrust => thrust,
            };

            let rotation = Rot2::new(Vec1::new(maneuver.data.angle));
            let force    = rotation.rotate(&Vec2::new(1.0, 0.0));
            let force    = force * thrust;

            match game_state.entities.bodies.get_mut(&maneuver.ship_id) {
                Some(body) =>
                    body.force = body.force + force,

                // The ship might not exist due to timing issues (it could
                // have been destroyed while the message was in flight). If
                // this happens too often, it might also be the symptom of a
                // bug.
                None => debug!("Ship not found: {}", maneuver.ship_id),
            }
        }

        if now_s >= maneuver.data.start_s + maneuver.data.duration_s {
            game_state.to_destroy.push(id);
        }
    }
}

pub fn apply_gravity(game_state: &mut GameState) {
    for (_, planet) in &game_state.entities.planets {
        for (_, body) in &mut game_state.entities.bodies {
            let g = 6.674e-11; // unit: N * m^2 / kg^2

            let body_to_planet = body.position - planet.position;
            let distance       = body_to_planet.norm();
            let direction      = body_to_planet / distance;

            let force =
                direction * -g * (planet.mass * body.mass) / distance;

            body.force = body.force + force;
        }
    }
}

pub fn integrate(game_state: &mut GameState) {
     for (_, body) in &mut game_state.entities.bodies {
        // TODO(E7GyYwQy): Take passed time since last iteration into
        //                 account.
        body.velocity = body.velocity + body.force;
        body.position = body.position + body.velocity;

        body.force = Vec2::new(0.0, 0.0);
    }
}

pub fn check_collisions(game_state: &mut GameState) {
    for (&body_id, body) in &game_state.entities.bodies {
        for (_, planet) in &game_state.entities.planets {
            let squared_size = planet.size * planet.size;

            if (body.position - planet.position).sqnorm() < squared_size {
                game_state.to_destroy.push(body_id);
            }
        }
    }
}