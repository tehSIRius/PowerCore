use core::fmt;

use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGAColor {
	Black      = 0,
	Blue       = 1,
	Green      = 2,
	Cyan       = 3,
	Red        = 4,
	Magenta    = 5,
	Brown      = 6,
	LightGray  = 7,
	DarkGray   = 8,
	LightBlue  = 9,
	LigthGreen = 10,
	LigthCyan  = 11,
	LightRed   = 12,
	Pink       = 13,
	Yellow     = 14,
	White      = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct VGAColorCode(u8);

impl VGAColorCode {
	fn new(foreground: VGAColor, background: VGAColor) -> VGAColorCode {
		VGAColorCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGAChar {
	ascii_character: u8,
	color_code: VGAColorCode,
}

const BUFFER_HEIGHT: usize = 25;

const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VGABuffer {
	// TODO: Figure out a way to get around outdated Volatile
	chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VGAWriter {
	column_position: usize,
	color_code: VGAColorCode,
	buffer: &'static mut VGABuffer,
}

impl VGAWriter {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;
				let color_code = self.color_code;
				self.buffer.chars[row][col].write(VGAChar { ascii_character: byte,
				                                            color_code, },);

				self.column_position += 1;
			}
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// Printable ASCII or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// Oopsie woopsie not an ASCII
				_ => self.write_byte(0xfe),
			}
		}
	}

	fn new_line(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();

				self.buffer.chars[row - 1][col].write(character);
			}
		}

		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	fn clear_row(&mut self, row: usize) {
		let blank = VGAChar { ascii_character: b' ',
		                      color_code: self.color_code };

		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

/// Support for rust write! macros
impl fmt::Write for VGAWriter {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);

		Ok(())
	}
}

lazy_static! {
	pub static ref WRITER: Mutex<VGAWriter> =
		Mutex::new(VGAWriter { column_position: 0,
		                       color_code: VGAColorCode::new(VGAColor::Yellow, VGAColor::Black),
		                       buffer: unsafe { &mut *(0xb8000 as *mut VGABuffer) } });
}

#[macro_export]
macro_rules! VGAprint {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! VGAprintln {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::VGAprint!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}
