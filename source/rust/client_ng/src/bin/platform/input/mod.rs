use std::comm::TryRecvError;
use std::io::stdin;


pub struct InputReader {
	receiver: Receiver<char>,
}

impl InputReader {
	pub fn new() -> InputReader {
		let (sender, receiver) = channel();

		spawn(move || {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_char() {
					Ok(c) =>
						sender.send(c),
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		InputReader {
			receiver: receiver,
		}
	}

	pub fn input(&mut self, chars: &mut Vec<char>) {
		loop {
			match self.receiver.try_recv() {
				Ok(c) =>
					chars.push(c),

				Err(error) => match error {
					TryRecvError::Empty =>
						break,
					TryRecvError::Disconnected =>
						panic!("Channel disconnected"),
				}
			}
		}
	}
}
