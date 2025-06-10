// idea: add clap
// idea: provide config as file
// idea: add modes: dot/dash mode, string mode etc

use cwos::prelude::*;
use std::io::stdin;

#[derive(Default)]
struct CliContext {
	input: CwString,
	output: CwString,
}

impl CwContext<CwString, CwString> for CliContext {
	fn input(&self) -> CwString {
		self.input.clone()
	}

	fn set_output(&mut self, value: CwString) {
		self.output = value;
	}

	fn config(&self) -> &Config {
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
				let mut ctx = CliContext {
					input,
					..Default::default()
				};
				controller.tick(&mut ctx);
				let output = ctx.output.as_string();

				println!("{output}");
			}
			Err(error) => {
				println!("<!> Error: {}", error);
			}
		};

		println!();
	}
}
