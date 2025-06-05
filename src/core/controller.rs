use super::symbol::SymbolString;

#[derive(Default, Debug, Clone)]
pub struct Response {
	pub cw: SymbolString,
	pub short: SymbolString,
	pub long: SymbolString,
}

impl Response {
	pub fn new(str: SymbolString) -> Self {
		Self {
			cw: str.clone(),
			short: str.clone(),
			long: str,
		}
	}
}

pub trait CwController: Default {
	fn tick(&mut self, input: SymbolString) -> Response;
}

#[derive(Default)]
pub struct Echo;

impl CwController for Echo {
	fn tick(&mut self, input: SymbolString) -> Response {
		Response::new(input)
	}
}
