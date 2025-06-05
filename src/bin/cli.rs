// idea: add clap
// idea: provide config as file
// idea: add modes: dot/dash mode, string mode etc

use std::io::stdin;

use cwos::{apps::launcher::AppLauncher, prelude::*};

pub fn main() {
	let mut controller = AppLauncher::default();

	loop {
		print!(">> ");
		let mut input_str = String::new();
		stdin().read_line(&mut input_str).unwrap();
		let input_str = input_str.trim();

		match SymbolString::try_from(input_str.to_string()) {
			Ok(input) => {
				let output = controller.tick(input).as_string();
				println!("{output}");
			}
			Err(error) => {
				println!("<!> Error: {}", error);
			}
		};

		println!();
	}
}
