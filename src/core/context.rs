use super::symbol::SymbolConverter;

// todo: implement Peripherals as <T, U>
// with fn tick(msg: T) -> U

#[derive(Default)]
pub struct CwContext<T> {
	pub database: T,
	pub symbol: SymbolConverter,
}

impl<T: Default> CwContext<T> {
	pub fn new(db: T) -> Self {
		Self {
			database: db,
			..Default::default()
		}
	}
}
