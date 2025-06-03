use super::{context::CwContext, symbol::SymbolString};

pub trait CwController<T>: Default {
	fn tick(&mut self, ctx: &mut CwContext<T>, input: SymbolString) -> SymbolString;
}

#[derive(Default)]
pub struct Echo;

impl<T> CwController<T> for Echo {
	fn tick(&mut self, _: &mut CwContext<T>, input: SymbolString) -> SymbolString {
		input
	}
}

#[derive(Default)]
pub struct TestController {
	is_done: bool,
}

impl<T> CwController<T> for TestController {
	fn tick(&mut self, _: &mut CwContext<T>, _: SymbolString) -> SymbolString {
		if !self.is_done {
			SymbolString::try_from("TEST".to_string()).unwrap()
		} else {
			self.is_done = true;
			Default::default()
		}
	}
}
