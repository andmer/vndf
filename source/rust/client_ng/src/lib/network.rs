use std::io::net::ip::{
	SocketAddr,
	ToSocketAddr,
};

use acpe::network;
use acpe::protocol::Perception;

use common::protocol::Percept;


pub struct Socket {
	address: SocketAddr,
	inner  : network::Socket,
}

impl Socket {
	pub fn new<T: ToSocketAddr>(address: T) -> Socket {
		let address = address
			.to_socket_addr()
			.unwrap_or_else(|error|
				panic!("Error converting socket address: {}", error)
			);

		let socket = network::Socket::new(0);

		Socket {
			address: address,
			inner  : socket,
		}
	}

	pub fn recv_from(&self) -> Vec<Perception<String, Percept>> {
		self.inner
			.recv_from()
			.into_iter()
			.map(|(message, _)|
				Perception::decode(message.as_slice())
					.unwrap_or_else(|error|
						panic!(
							"Error decoding message from server. \
							Message: {}; Error: {}",
							message, error
						)
					)
			)
			.collect()
	}

	pub fn send_to(&mut self, message: &[u8]) {
		self.inner.send_to(message, self.address);
	}
}