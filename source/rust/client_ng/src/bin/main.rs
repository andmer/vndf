#![allow(unstable)]
#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate acpe;

extern crate client;
extern crate common;


use std::collections::HashMap;
use std::io::timer::sleep;
use std::time::Duration;

use acpe::MAX_PACKET_SIZE;

use args::Args;
use client::platform::{
	Frame,
	Input,
	Status,
};
use common::protocol::{
	ClientEvent,
	ServerEvent,
};
use network::Network;
use platform::{
	HeadlessIo,
	PlatformIo,
	PlayerIo,
};


mod args;
mod network;
mod platform;
mod termios;


fn main() {
	let args = Args::parse(std::os::args().as_slice());

	if args.headless {
		run(args, init_platform::<HeadlessIo>())
	}
	else {
		run(args, init_platform::<PlayerIo>())
	}
}


fn init_platform<P: PlatformIo>() -> P {
	match PlatformIo::new() {
		Ok(platform) =>
			platform,
		Err(error) =>
			panic!("Error initializing platform I/O: {}", error),
	}
}

fn run<P: PlatformIo>(args: Args, mut platform: P) {
	let mut frame = Frame {
		self_id   : String::new(),
		status    : Status::None,
		broadcasts: Vec::new(),
	};

	let mut broadcasts     = HashMap::new();
	let mut network        = Network::new(args.server);
	let mut previous_input = Input::new();

	loop {
		let input = platform.input();

		if input != previous_input {
			match input.broadcast {
				Some(ref message) =>
					if message.len() == 0 {
						frame.status = Status::Error(
							"Broadcasts can not be empty".to_string()
						);
					}
					else if message.len() > MAX_PACKET_SIZE / 2 {
						frame.status = Status::Error(
							"Broadcast message too long".to_string()
						);
					}
					else {
						network.send(
							ClientEvent::StartBroadcast(message.clone())
						);

						frame.status = Status::Notice(
							"Sending broadcast".to_string()
						);
					},
				None => {
					network.send(ClientEvent::StopBroadcast);

					frame.status = Status::Notice(
						"Stopped sending broadcast".to_string()
					);
				},
			}
		}

		previous_input = input.clone();

		for event in network.receive() {
			match event {
				ServerEvent::SelfId(self_id) => {
					frame.self_id = self_id;
				},
				ServerEvent::StartBroadcast(broadcast) => {
					broadcasts.insert(broadcast.sender.clone(), broadcast);
				},
				ServerEvent::StopBroadcast(sender) => {
					broadcasts.remove(&sender);
				},
			}
		}

		frame.broadcasts = broadcasts
			.iter()
			.map(|(_, broadcast)|
				broadcast.clone()
			)
			.collect();

		network.update();

		if let Err(error) = platform.render(&frame) {
			panic!("Error writing output: {}", error);
		}

		sleep(Duration::milliseconds(20));
	}
}
