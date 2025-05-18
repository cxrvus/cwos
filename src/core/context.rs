use super::{database::Database, symbol::SymbolConverter};

// todo: implement Peripherals as <T, U>
// with fn tick(msg: T) -> U

#[derive(Default)]
pub struct CwContext {
	pub database: Database,
	pub symbol: SymbolConverter,
}

impl CwContext {
	pub fn new(db: Database) -> Self {
		Self {
			database: db,
			..Default::default()
		}
	}
}
