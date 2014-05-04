use libc::c_int;
use std::comm::{
	Disconnected,
	Empty
};

use common::physics::{Body, Radians, Vec2};
use common::net::Connection;
use common::protocol::{
	Action,
	Perception,
	Ship
};

use clients::{Client, Clients};
use events::{
	Action,
	Close,
	DataReceived,
	Enter,
	GameEvent,
	Init,
	Leave,
	NetworkEvent,
	Update
};


pub struct EventHandler {
	pub events: Sender<GameEvent>,

	incoming: Receiver<GameEvent>,
	network : Sender<NetworkEvent>
}


impl EventHandler {
	pub fn new(network: Sender<NetworkEvent>) -> EventHandler {
		let (sender, receiver) = channel();

		EventHandler {
			events  : sender,

			incoming: receiver,
			network : network,
		}
	}

	pub fn handle(&mut self, clients: &mut Clients) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => {
					print!("Incoming event: {}\n", event);

					match event {
						Init =>
							(), // nothing do do, it just exists for the logging
						Enter(connection) =>
							self.on_enter(connection, clients),
						Leave(clientId) =>
							self.on_leave(clientId, clients),
						DataReceived(fd) =>
							self.on_data_received(fd, clients),
						Update(frame_time_in_s) =>
							self.on_update(clients, frame_time_in_s),
						Action(client_id, attitude) =>
							self.on_action(client_id, attitude, clients)
					}
				},

				Err(error) => match error {
					Empty        => break,
					Disconnected => fail!("Unexpected error: {}", error)
				}
			}
		}
	}

	fn on_enter(&mut self, connection: Connection, clients: &mut Clients) {
		let velocity = Vec2(30.0, 10.0);

		let ship = Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		};

		let new_client = Client::new(connection, ship);
		clients.add(new_client);
	}

	fn on_leave(&mut self, removed_id: uint, clients: &mut Clients) {
		clients.remove(removed_id);
	}

	fn on_data_received(&mut self, fd: c_int, clients: &mut Clients) {
		let (client_id, client) = match clients.client_by_fd(fd) {
			Some(result) => result,
			None         => return
		};

		let result = client.conn.receive_messages(|raw_message| {
			let action = match Action::from_str(raw_message) {
				Ok(message) => message,
				Err(error)  => fail!("Error decoding message: {}", error)
			};

			self.events.send(Action(fd, action.attitude));
		});

		match result {
			Ok(()) => (),
			Err(_) => self.network.send(Close(client_id))
		}
	}

	fn on_update(&mut self, clients: &mut Clients, dTimeInS: f64) {
		clients.mut_each(|_, client| {
			client.ship.velocity = client.ship.attitude.to_vec() * 30.0;
			client.ship.position =
				client.ship.position + client.ship.velocity * dTimeInS;
		});

		let mut ships = Vec::new();
		clients.each(|client_id, client| {
			ships.push(Ship {
				id  : client_id,
				body: client.ship
			});
		});

		clients.each(|client_id, client| {
			let update = Perception {
				self_id: client_id,
				ships  : ships.as_slice().to_owned()
			};
			let message = update.to_str();

			match client.conn.send_message(message) {
				Err(_) => self.network.send(Close(client_id)),
				_      => ()
			};
		});
	}

	fn on_action(&self, fd: c_int, attitude: Radians, clients: &mut Clients) {
		match clients.client_by_fd(fd) {
			Some((_, client)) => client.ship.attitude = attitude,
			None              => ()
		}
	}
}
