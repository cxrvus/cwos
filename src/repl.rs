use cwos::core::{
	context::Context,
	database::Database,
	routine::{Echo, Routine},
	symbol::Symbol,
};
use std::io::{self, Read};
use termion::raw::IntoRawMode;

pub fn main() {
	let stdin = io::stdin();
	let _stdout = io::stdout().into_raw_mode().unwrap();

	let mut ctx = Context::new(Database::default());
	let mut controller = Echo;

	for byte in stdin.bytes() {
		match byte {
			Ok(input_byte) => {
				// Ctrl+C
				if input_byte == 3 {
					break;
				}

				println!("\r<< {}", input_byte as char);

				let input = Symbol::Sign(input_byte.into());
				let output = controller.tick(&mut ctx, input).to_string();

				println!("\r>> {}", output);
			}
			Err(_) => break,
		}
	}
}
