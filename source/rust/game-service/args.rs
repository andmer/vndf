use getopts::{
	getopts,
	optopt,
	usage
};
use std::os;


pub struct Args {
	pub port: ~str
}


pub fn parse() -> Option<Args> {
	let mut parsed_args = Args {
		port: ~"34481"
	};

	let args = os::args();

	let options = [
		optopt("p", "port", "port to listen on", parsed_args.port)
	];

	let usage = usage(format!("{} [OPTIONS]", args[0]), options);

	let matches = match getopts(args.tail(), options) {
		Ok(matches) => matches,
		Err(fail)   => {
			print!("{}\n", fail.to_err_msg());
			print!("{}", usage);

			return None
		}
	};

	match matches.opt_str("p") {
		Some(port) => parsed_args.port = port,
		None       => ()
	}

	Some(parsed_args)
}
