use crate::database::Database;

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

	pub fn tick(&self) {
		todo!()
	}
}

pub struct Routine {}
