use crate::{database::Database, symbol_converter::PlainSymbol};

#[derive(Default)]
pub struct Controller {
	database: Database,
	stack: Vec<Routine>,
}

impl Controller {
	pub fn new(db: Database) -> Self {
		Self {
			database: db,
			..Default::default()
		}
	}

	pub fn tick(&self, cw: PlainSymbol) {
		todo!()
	}
}

pub struct Routine {}
