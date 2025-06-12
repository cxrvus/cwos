// idea: add clap
// idea: provide config as file
// idea: add modes: dot/dash mode, string mode etc

use cwos::prelude::*;
use std::io::stdin;

#[derive(Default)]
struct CliContext;

// TODO: use StdContext (merged with UiContext)
impl CwContext for CliContext {
	fn config(&self) -> Config {
		todo!()
	}

	fn time(&self) -> u32 {
		todo!()
	}
}

pub fn main() {
	let mut controller = AppLauncher::default();

	loop {
		print!(">> ");
		let mut input_str = String::new();
		stdin().read_line(&mut input_str).unwrap();
		let input_str = input_str.trim();

		match CwString::try_from(input_str.to_string()) {
			Ok(input) => {
				let mut ctx = CliContext;
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
