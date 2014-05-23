use collections::HashMap;
use time;

use common::physics::{
	Body,
	Vec2
};

use network::Network;


pub struct GameState {
	self_id : Option<uint>,
	ships   : InterpolatedBodies,
	missiles: InterpolatedBodies
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			self_id : None,
			ships   : InterpolatedBodies::new(),
			missiles: InterpolatedBodies::new()
		}
	}

	pub fn receive_updates(&mut self, network: &mut Network) {
		network.receive(|perception| {
			self.self_id = Some(perception.self_id);

			self.ships.receive(&perception.ships);
			self.missiles.receive(&perception.missiles);
		});
	}

	pub fn interpolate(&mut self) -> (Vec<Body>, Vec<Body>) {
		(
			self.ships.interpolate(),
			self.missiles.interpolate())
	}

	pub fn update_camera(&self, camera: &mut Vec2) {
		let self_id = match self.self_id {
			Some(id) => id,
			None     => return
		};

		for (&id, ship) in self.ships.current.iter() {
			if id == self_id {
				*camera = ship.position;
			}
		}
	}
}


struct InterpolatedBodies {
	previous_time: u64,
	current_time : u64,

	previous: HashMap<uint, Body>,
	current : HashMap<uint, Body>
}

impl InterpolatedBodies {
	fn new() -> InterpolatedBodies {
		InterpolatedBodies {
			previous_time: time::precise_time_ns(),
			current_time : time::precise_time_ns(),

			previous: HashMap::new(),
			current : HashMap::new()
		}
	}

	fn receive(&mut self, bodies: &HashMap<uint, Body>) {
		self.previous_time = self.current_time;
		self.current_time  = time::precise_time_ns();

		self.previous.clear();
		for (&id, &body) in self.current.iter() {
			self.previous.insert(id, body);
		}

		self.current.clear();
		for (&id, &body) in bodies.iter() {
			self.current.insert(id, body);
		}
	}

	fn interpolate(&self) -> Vec<Body> {
		let i = {
			let diff = (self.current_time - self.previous_time) as f64;
			if diff <= 0.0 {
				0.0
			}
			else {
				(time::precise_time_ns() - self.current_time) as f64 / diff
			}
		};

		let mut bodies = Vec::new();
		for (&ship_id, &current) in self.current.iter() {
			match self.previous.find(&ship_id) {
				Some(&previous) => {
					let mut body = current.clone();
					body.position = previous.position + (current.position - previous.position) * i;
					bodies.push(body);
				},

				None => ()
			}
		}

		bodies
	}
}
