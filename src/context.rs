use crate::{database::Database, symbol_converter::Symbol};

// todo: implement Peripherals as <T, U>
// with fn tick(msg: T) -> U

#[derive(Default)]
pub struct Context {
	database: Database,
}

impl Context {
	pub fn new(db: Database) -> Self {
		Self { database: db }
	}
}

pub trait Routine {
	fn tick(&mut self, ctx: &mut Context, input: Option<Symbol>) -> Option<Symbol>;
}
