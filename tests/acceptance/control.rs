use rand;

use common::protocol::{Message, SelfInfo, Update};

use util::Process;


pub struct GameService {
	pub port   : uint,
	pub process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = rand::random::<uint>() % 10000 + 40000;

		let mut process = Process::start(
			"output/bin/vndf-game-service", [port.to_str()]);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}


pub struct ClientCore {
	process: Process
}

impl ClientCore {
	pub fn start(port: uint) -> ClientCore {
		ClientCore {
			process: Process::start(
				"output/bin/vndf-client-core", [~"localhost", port.to_str()])
		}
	}

	pub fn ignore_message(&mut self) {
		self.process.read_stdout_line();
	}

	pub fn expect_self_id(&mut self) -> uint {
		match self.next_message() {
			SelfInfo(self_info) => self_info.id,
			message @ _         => fail!("unexpected message ({})", message)
		}
	}

	pub fn expect_update(&mut self) -> Update {
		match self.next_message() {
			Update(update) => update,
			message @ _    => fail!("unexpected message ({})", message)
		}
	}

	fn next_message(&mut self) -> Message {
		let message = self.process.read_stdout_line();
		Message::from_str(message)
	}
}
