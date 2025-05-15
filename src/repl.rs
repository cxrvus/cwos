use std::io::{self, Read};
use termion::raw::IntoRawMode;

pub fn main() {
	let stdin = io::stdin();
	let _stdout = io::stdout().into_raw_mode().unwrap();

	for byte in stdin.bytes() {
		match byte {
			Ok(b) => {
				// Ctrl+C
				if b == 3 {
					break;
				}

				println!("\r({})", b as char);
			}
			Err(_) => break,
		}
	}
}
