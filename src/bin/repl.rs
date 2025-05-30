// idea: add clap
// idea: provide config as file
// idea: add modes: dot/dash mode, string mode etc

use std::io::stdin;

use cwos::core::{
	context::CwContext,
	database::Database,
	procedure::{Echo, Procedure},
	symbol::SymbolString,
};

pub fn main() {
	let mut ctx = CwContext::new(Database::default());
	let mut controller = Echo;

	loop {
		let mut input_str = String::new();
		stdin().read_line(&mut input_str).unwrap();
		let input_str = input_str.trim();

		let input = SymbolString::try_from(input_str.to_string()).unwrap();
		let response = controller.tick(&mut ctx, input);
		let output = response.as_string();

		println!("<< {}", output);
	}
}
