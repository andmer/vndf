extern crate common;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;


use std::hashmap::HashMap;
use std::io;
use std::os;
use std::path;
use std::str;

use camera::Camera;
use entities::Entities;


mod camera;
mod display;
mod entities;
mod images;
mod input;
mod net;
mod protocol;
mod text;
mod texture;
mod visual;


#[link(name = "stb-image", kind = "static")]
extern {}


fn main() {
	let screen_width  = 800;
	let screen_height = 600;

	let args = os::args();

	if args.len() > 2 {
		print!("Usage: {:s} serverAddress\n", args[0]);
		return
	}

	let serverAddress = if args.len() == 2 {
		args[1]
	}
	else {
		let mut file = match io::File::open(&path::posix::Path::new("server")) {
			Ok(file) => file,
			Err(e)   => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		let contents = match file.read_to_end() {
			Ok(contents) => contents,
			Err(e)       => {
				print!("ERROR {}\n", e);
				fail!();
			}
		};

		str::from_utf8(contents).unwrap_or_else(|| { fail!() }).to_owned()
	};

	let window = display::init(screen_width, screen_height);
	let images = images::load();
	let font   = text::load_font();

	let mut textures = HashMap::new();
	for (id, &texture) in images.iter() {
		textures.insert(id.clone(), texture);
	}
	for (id, &texture) in font.iter() {
		textures.insert(id.clone(), texture);
	}

	let     socket_fd  = net::connect(serverAddress, ~"34481");
	let mut connection = protocol::init(socket_fd);

	let mut entities = Entities::new();

	let mut cam = Camera::new();

	while !window.should_close() &&
		window.get_key(glfw::KeyEscape) == glfw::Release {

		protocol::receive_positions(
			&mut connection,
			entities);
		input::apply(
			&window,
			cam);
		display::render(
			&window,
			cam,
			&entities.positions,
			&entities.visuals,
			&textures);

		glfw::poll_events();
	}
}
