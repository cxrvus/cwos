use crate::{database::Database, symbol_converter::Symbol};

// todo: implement Peripherals as <T, U>
// with fn tick(msg: T) -> U

#[derive(Default)]
pub struct Controller {
	database: Database,
	stack: Vec<Box<dyn Routine>>,
}

impl Controller {
	pub fn new(db: Database) -> Self {
		Self {
			database: db,
			..Default::default()
		}
	}

	pub fn tick(&mut self, input: Option<Symbol>) -> Option<Symbol> {
		// todo: add behavior for empty stack
		let mut current_routine = self
			.stack
			.pop()
			.expect("exited because no routine was active");

		let result = current_routine.tick(self, input);

		self.stack.push(current_routine);

		result
	}

	// todo: create functions for pushing and pulling etc
}

pub trait Routine {
	fn tick(&mut self, cnt: &mut Controller, input: Option<Symbol>) -> Option<Symbol>;
}
