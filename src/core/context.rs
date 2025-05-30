#[derive(Default)]
pub struct CwContext<T> {
	pub database: T,
}

impl<T: Default> CwContext<T> {
	pub fn new(db: T) -> Self {
		Self { database: db }
	}
}
