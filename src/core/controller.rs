use super::{context::CwContext, symbol::SymbolString};

pub trait CwController<T>: Default {
	fn tick(&mut self, ctx: &mut CwContext<T>, buffer: SymbolString) -> SymbolString;
}

#[derive(Default)]
pub struct Echo;

impl<T> CwController<T> for Echo {
	fn tick(&mut self, _: &mut CwContext<T>, buffer: SymbolString) -> SymbolString {
		buffer
	}
}
