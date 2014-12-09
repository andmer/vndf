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


#[deriving(Clone, Eq, PartialEq)]
pub struct C {
	pub c    : char,
	pub bold : bool,
	pub color: Color,
}

impl C {
	fn new() -> C {
		C {
			c    : ' ',
			bold : false,
			color: Color::default(),
		}
	}
}


#[deriving(Clone)]
pub struct ScreenBuffer {
	buffer: Vec<Vec<C>>,
	bold  : bool,
	color : Color,
}

impl ScreenBuffer {
	pub fn new(width: Pos, height: Pos) -> ScreenBuffer {
		let width  = width  as uint;
		let height = height as uint;

		ScreenBuffer {
			buffer: Vec::from_fn(height, |_| Vec::from_elem(width, C::new())),
			bold  : false,
			color : Color::default(),
		}
	}

	pub fn width(&self) -> Pos {
		self.buffer[0].len() as Pos
	}

	pub fn height(&self) -> Pos {
		self.buffer.len() as Pos
	}

	pub fn bold(&mut self, bold: bool) -> bool {
		let previous_value = self.bold;
		self.bold = bold;
		previous_value
	}

	pub fn color(&mut self, color: Color) -> Color {
		let previous_value = self.color;
		self.color = color;
		previous_value
	}

	/// Origin is in upper-left corner.
	pub fn writer(&mut self, x: Pos, y: Pos, limit: Pos) -> BufferWriter {
		BufferWriter {
			buffer: self,
			x     : x,
			y     : y,
			limit : limit,
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
	}
}


pub struct BufferWriter<'r> {
	pub buffer: &'r mut ScreenBuffer,
	pub x     : Pos,
	pub y     : Pos,
	pub limit : Pos,
}

impl<'r> Writer for BufferWriter<'r> {
	fn write(&mut self, buf: &[u8]) -> IoResult<()> {
		if self.y >= self.buffer.height() {
			return Err(IoError {
				kind  : IoErrorKind::OtherIoError,
				desc  : "y coordinate is out of bounds",
				detail: None,
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
				c    : c,
				bold : self.buffer.bold,
				color: self.buffer.color,
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
