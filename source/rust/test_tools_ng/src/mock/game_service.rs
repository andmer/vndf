use std::io::net::ip::{
	Port,
	SocketAddr,
};
use time::precise_time_s;

use acceptance::random_port;
use acpe::MAX_PACKET_SIZE;
use acpe::protocol::{
	Action,
	Encoder,
};

use common::network::SocketSender;
use common::protocol::{
	Percept,
	Step,
};
use game_service::{
	ReceiveResult,
	Socket,
};


pub struct GameService {
	port    : Port,
	socket  : Socket,
	received: Vec<ReceiveResult>,
}

impl GameService {
	pub fn start() -> GameService {
		let port   = random_port(40000, 50000);
		let socket = Socket::new(port);

		GameService {
			port    : port,
			socket  : socket,
			received: Vec::new(),
		}
	}

	pub fn port(&self) -> Port {
		self.port
	}

	pub fn expect_action(&mut self) -> Option<ActionHandle> {
		let start_s = precise_time_s();

		while self.received.len() == 0 && precise_time_s() - start_s < 0.5 {
			self.received.push_all(self.socket.recv_from().as_slice());
		}

		match self.received.remove(0) {
			Some(result) => match result {
				Ok((action, address)) =>
					Some(ActionHandle {
						inner  : action,
						address: address,
						sender : self.socket.inner.sender.clone(),
					}),
				Err((error, address)) =>
					panic!(
						"Error receiving message from {}: {}",
						address, error
					),
			},
			None =>
				None,
		}
	}

	pub fn wait_until(
		&mut self,
		condition: |&Option<ActionHandle>| -> bool
	) -> Option<ActionHandle> {
		let mut action = self.expect_action();

		while !condition(&action) {
			action = self.expect_action();
		}

		action
	}
}


pub struct ActionHandle {
	pub inner: Action<Step>,

	address: SocketAddr,
	sender : SocketSender,
}

impl ActionHandle {
	pub fn ignore(&self) {}

	pub fn confirm(&mut self) {
		let mut encoder       = Encoder::new();
		let mut encode_buffer = [0, ..MAX_PACKET_SIZE];
		let     perception    = encoder.message::<Percept>(self.inner.seq);

		let message = perception.encode(&mut encode_buffer).unwrap();
		self.sender.send(message, self.address);
	}
}
