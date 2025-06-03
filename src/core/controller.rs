use super::symbol::SymbolString;

pub trait CwController: Default {
	fn tick(&mut self, input: SymbolString) -> SymbolString;
}

#[derive(Default)]
pub struct Echo;

impl CwController for Echo {
	fn tick(&mut self, input: SymbolString) -> SymbolString {
		input.normalized()
	}
}

#[derive(Default)]
pub struct TestController {
	is_done: bool,
}

impl CwController for TestController {
	fn tick(&mut self, _: SymbolString) -> SymbolString {
		if !self.is_done {
			SymbolString::try_from("TEST".to_string()).unwrap()
		} else {
			self.is_done = true;
			Default::default()
		}
	}
}
