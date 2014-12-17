use std::io::{
	IoError,
	IoErrorKind,
	IoResult,
};
use std::str::from_utf8;

use super::{
	Color,
	Pos,
};


#[deriving(Clone, Copy, Eq, PartialEq)]
pub struct C {
	pub c   : char,
	pub bold: bool,

	pub foreground_color: Color,
	pub background_color: Option<Color>,
}

impl C {
	pub fn new() -> C {
		C {
			c   : ' ',
			bold: false,

			foreground_color: Color::default(),
			background_color: None,
		}
	}
}


#[deriving(Clone)]
pub struct ScreenBuffer {
	buffer: Vec<Vec<C>>,

	foreground_color: Color,
	background_color: Option<Color>,
}

impl ScreenBuffer {
	pub fn new(width: Pos, height: Pos) -> ScreenBuffer {
		let width  = width  as uint;
		let height = height as uint;

		ScreenBuffer {
			buffer: Vec::from_fn(height, |_| Vec::from_elem(width, C::new())),

			foreground_color: Color::default(),
			background_color: None,
		}
	}

	pub fn width(&self) -> Pos {
		self.buffer[0].len() as Pos
	}

	pub fn height(&self) -> Pos {
		self.buffer.len() as Pos
	}

	pub fn foreground_color(&mut self, color: Color) -> Color {
		let previous_value = self.foreground_color;
		self.foreground_color = color;
		previous_value
	}

	pub fn background_color(&mut self, color: Option<Color>) -> Option<Color> {
		let previous_value = self.background_color;
		self.background_color = color;
		previous_value
	}

	/// Origin is in upper-left corner.
	pub fn writer(&mut self, x: Pos, y: Pos) -> BufferWriter {
		let width = self.width();

		BufferWriter {
			buffer: self,

			x    : x,
			y    : y,
			limit: width,

			bold: true,
		}
	}

	pub fn set(&mut self, x: Pos, y: Pos, c: C) -> IoResult<()> {
		let x = x as uint;
		let y = y as uint;

		if y > self.buffer.len() || x > self.buffer[0].len() {
			return Err(IoError {
				kind  : IoErrorKind::OtherIoError,
				desc  : "Out of bounds",
				detail: None,
			})
		}

		self.buffer[y][x] = c;

		Ok(())
	}

	pub fn iter(&self) -> BufferIterator {
		BufferIterator {
			buffer: &self.buffer,
			x     : 0,
			y     : 0,
		}
	}

	pub fn clear(&mut self) {
		for line in self.buffer.iter_mut() {
			for c in line.iter_mut() {
				*c = C::new();
			}
		}

		self.foreground_color = Color::default();
	}
}


// TODO: Move attributes from ScreenBuffer into BufferWriter. Initialize them
//       with defaults and provide a builder-style API to set them. This scopes
//       attributes to the write operation and removes the need to reset them.
pub struct BufferWriter<'r> {
	pub buffer: &'r mut ScreenBuffer,

	pub x    : Pos,
	pub y    : Pos,
	pub limit: Pos,

	bold: bool,
}

impl<'r> BufferWriter<'r> {
	pub fn limit(mut self, limit: Pos) -> BufferWriter<'r> {
		self.limit = limit;
		self
	}

	// There is no setter for the bold attribute, for the simple reason that
	// it's not currently needed and I don't want to see an unused warning all
	// the time. Once one is needed again, it can be trivially added here.
}

impl<'r> Writer for BufferWriter<'r> {
	fn write(&mut self, buf: &[u8]) -> IoResult<()> {
		if self.y >= self.buffer.height() {
			let detail = format!(
				"x: {}, y: {}, width: {}, height: {}",
				self.x, self.y, self.buffer.width(), self.buffer.height(),
			);

			return Err(IoError {
				kind  : IoErrorKind::OtherIoError,
				desc  : "y coordinate is out of bounds",
				detail: Some(detail),
			})
		}

		let s = match from_utf8(buf) {
			Some(s) =>
				s,
			None =>
				return Err(IoError {
					kind  : IoErrorKind::OtherIoError,
					desc  : "Tried to write invalid UTF-8",
					detail: None,
				})

		};

		for c in s.chars() {
			if self.x >= self.limit || self.x >= self.buffer.width() {
				// Truncate everything beyond the limit
				break;
			}

			let x = self.x as uint;
			let y = self.y as uint;
			self.buffer.buffer[y][x] = C {
				c   : c,
				bold: self.bold,

				foreground_color: self.buffer.foreground_color,
				background_color: self.buffer.background_color,
			};

			self.x += 1;
		}

		Ok(())
	}
}


pub struct BufferIterator<'r> {
	buffer: &'r Vec<Vec<C>>,
	x     : uint,
	y     : uint,
}

impl<'r> Iterator<(Pos, Pos, C)> for BufferIterator<'r> {
	fn next(&mut self) -> Option<(Pos, Pos, C)> {
		if self.x >= self.buffer[0].len() {
			self.x  = 0;
			self.y += 1;
		}

		if self.y >= self.buffer.len() {
			return None;
		}

		let result =
			Some((self.x as Pos, self.y as Pos, self.buffer[self.y][self.x]));

		self.x += 1;

		result
	}
}
