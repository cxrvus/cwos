use super::symbol::SymbolString;

pub trait CwController: Default {
	fn tick(&mut self, input: SymbolString) -> SymbolString;
}

#[derive(Default)]
pub struct Echo;

impl CwController for Echo {
	fn tick(&mut self, input: SymbolString) -> SymbolString {
		input
	}
}
