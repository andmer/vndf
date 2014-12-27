use std::io::net::ip::{
	Port,
	SocketAddr,
};

use acpe::network::{
	mod,
	Message,
};
use acpe::protocol::ActionHeader;
use acpe::protocol::Message as AcpeMessage;

use common::protocol::Step;


pub type ReceiveResult =
	Result<(AcpeMessage<ActionHeader, Step>, SocketAddr), (String, SocketAddr)>;


pub struct Socket {
	pub inner: network::Socket,

	messages: Vec<Message>,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		Socket {
			inner   : network::Socket::new(port),
			messages: Vec::new(),
		}
	}

	pub fn send(&mut self, message: &[u8], address: SocketAddr) {
		self.inner.send(message, address)
	}

	pub fn receive(&mut self, results: &mut Vec<ReceiveResult>) {
		self.inner.receive(&mut self.messages);

		for (message, address) in self.messages.drain() {
			let result = match decode_message(message.as_slice()) {
				Ok(message) => Ok((message, address)),
				Err(error)  => Err((error, address)),
			};

			results.push(result);
		}
	}
}


fn decode_message(message: &[u8]) -> Result<AcpeMessage<ActionHeader, Step>, String> {
	let message = match AcpeMessage::decode(message) {
		Ok(message) =>
			message,
		Err(error) =>
			return Err((
				format!(
					"Error decoding message. Error: {}; Message: {}",
					error, message
				)
			)),
	};

	Ok(message)
}
