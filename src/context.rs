use crate::database::Database;

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
