use cwos::core::{
	context::CwContext,
	database::Database,
	routine::{Echo, Routine},
};
use std::io::{self, Read};
use termion::raw::IntoRawMode;

pub fn main() {
	let stdin = io::stdin();
	let _stdout = io::stdout().into_raw_mode().unwrap();

	let mut ctx = CwContext::new(Database::default());
	let mut controller = Echo;

	for byte in stdin.bytes() {
		match byte {
			Ok(mut input_byte) => {
				if input_byte == 3 {
					break; // Ctrl+C
				}

				if input_byte == b'\r' {
					input_byte = b'\n';
				}

				println!("\r<< {}", input_byte as char);

				let input = ctx.symbol.from_char(input_byte.into()).unwrap();
				let response = controller.tick(&mut ctx, input);
				let output = ctx.symbol.to_char(response);

				println!("\r>> {}", output);
			}
			Err(_) => break,
		}
	}
}
