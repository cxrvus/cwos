// idea: add clap
// idea: provide config as file
// idea: add modes: dot/dash mode, string mode etc

use cwos::{prelude::*, std_context::StdContext};
use std::io::stdin;

pub fn main() {
	let mut controller = AppLauncher::default();

	loop {
		print!(">> ");
		let mut input_str = String::new();
		stdin().read_line(&mut input_str).unwrap();
		let input_str = input_str.trim();

		match CwString::try_from(input_str.to_string()) {
			Ok(input) => {
				let mut ctx = StdContext;
				let output = controller.tick(&mut ctx, input).as_string();

				println!("{output}");
			}
			Err(error) => {
				println!("<!> Error: {}", error);
			}
		};

		println!();
	}
}
