pub mod base;
pub mod input;
pub mod render;
pub mod state;
pub mod update;


use std::old_io::IoResult;
use std::vec::Drain;

use client::platform::{
	Frame,
	InputEvent,
};
use render::{
	Pos,
	Screen,
};

use self::base::{
	ProcessInput,
	Render,
	Update,
};
use self::base::InputEvent::{
	Backspace,
	Char,
	CursorDown,
	CursorLeft,
	CursorRight,
	CursorUp,
	Enter,
};
use self::state::{
	InfoSection,
	MainSection,
	TabSwitcher,
};
use self::update::CommTabArgs;


pub struct Ui {
	screen      : Screen,
	main        : MainSection,
	info        : InfoSection,
	tab_switcher: TabSwitcher,

	mode  : TextInputMode,
	events: Vec<InputEvent>,

	// TODO: This is not very pretty, and a sign that the strict separation of
	//       at least update and render isn't working out. I should find a way
	//       to work more towards an immediate-mode approach.
	pub broadcast_list_height: Pos,
}

impl Ui {
	pub fn new() -> IoResult<Ui> {
		let width = 80;

		let screen = try!(Screen::new(width, 24));

		Ok(Ui {
			screen      : screen,
			main        : MainSection::new(width, 18),
			info        : InfoSection::new(width,  6),
			tab_switcher: TabSwitcher::new(),
			mode        : TextInputMode::Regular,
			events      : Vec::new(),

			broadcast_list_height: 5,
		})
	}

	pub fn update(&mut self, frame: &Frame, chars: &[char])
		-> IoResult<Drain<InputEvent>>
	{
		self.process_input(chars);
		self.generate_events(frame);
		try!(self.render(frame));

		Ok(self.events.drain())
	}

	fn process_input(&mut self, chars: &[char]) {
		for &c in chars.iter() {
			match self.mode {
				TextInputMode::Regular => {
					if c == '\x1b' { // Escape
						self.mode = TextInputMode::Escape;
					}
					else if c == '\x7f' { // Backspace
						self.tab_switcher.process_event(Backspace);
					}
					else if c == '\n' {
						self.tab_switcher.process_event(Enter);
					}
					else {
						self.tab_switcher.process_event(Char(c));
					}
				},
				TextInputMode::Escape => {
					if c == '[' {
						self.mode = TextInputMode::Cursor;
					}
					else {
						// Unexpected character. Fall back to regular mode.
						self.mode = TextInputMode::Regular;
					}
				},
				TextInputMode::Cursor => {
					let event = match c {
						'A' => Some(CursorUp),
						'B' => Some(CursorDown),
						'C' => Some(CursorRight),
						'D' => Some(CursorLeft),
						_   => None, // Unexpected character
					};

					if let Some(event) = event {
						self.tab_switcher.process_event(event);
					}

					self.mode = TextInputMode::Regular;
				},
			}
		}
	}

	fn generate_events(&mut self, frame: &Frame) {
		let is_sending = frame.broadcasts
			.iter()
			.any(|broadcast|
				broadcast.sender == frame.self_id
			);

		self.tab_switcher.comm_tab.update(&CommTabArgs {
			is_sending : is_sending,
			list_length: frame.broadcasts.len(),
			list_height: self.broadcast_list_height,
		});

		if self.tab_switcher.comm_tab.broadcast_form.button.was_activated {
			self.tab_switcher.comm_tab.broadcast_form.button.was_activated = false;

			if is_sending {
				self.events.push(InputEvent::StopBroadcast);
			}
			else {
				let message =
					self.tab_switcher.comm_tab.broadcast_form.text_field.text.clone();
				self.events.push(InputEvent::StartBroadcast(message));
			}
		}
	}

	fn render(&mut self, frame: &Frame) -> IoResult<()> {
		self.screen.cursor(None);

		try!(self.render_main(frame));
		try!(self.info.render(
			self.screen.buffer(),
			0, self.main.height,
			&frame.status,
		));

		try!(self.screen.submit());

		Ok(())
	}

	fn render_main(&mut self, frame: &Frame) -> IoResult<()> {
		self.main.section.buffer.clear();

		let mut broadcasts: Vec<String> = frame.broadcasts
			.iter()
			.map(|broadcast|
				format!("{}: {}", broadcast.sender, broadcast.message)
			)
			.collect();
		broadcasts.sort();

		try!(self.tab_switcher.render(
			&mut self.main.section.buffer,
			0, 0,
			&render::TabSwitcherArgs {
				self_id    : frame.self_id.as_slice(),
				broadcasts : broadcasts.as_slice(),
				list_height: self.broadcast_list_height,
			},
		));

		try!(self.main.section.write(0, 0, &mut self.screen));

		Ok(())
	}
}


enum TextInputMode {
	Regular,
	Escape,
	Cursor,
}
