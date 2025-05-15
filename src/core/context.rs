use super::database::Database;

// todo: implement Peripherals as <T, U>
// with fn tick(msg: T) -> U

#[derive(Default)]
pub struct CwContext {
	database: Database,
}

impl CwContext {
	pub fn new(db: Database) -> Self {
		Self { database: db }
	}
}
