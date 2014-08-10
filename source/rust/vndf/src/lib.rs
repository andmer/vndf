#![feature(macro_rules)]
#![feature(phase)]


extern crate getopts;
extern crate libc;
extern crate serialize;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate epoll;
extern crate net;
extern crate physics;
extern crate platform;
extern crate rustecs;
#[phase(plugin)] extern crate rustecs_macros;


pub mod client;
pub mod game;
pub mod game_service;
pub mod protocol;
pub mod test_infra;
pub mod test_tools;
