use crate::core::symbol::SymbolString;

use super::context::CwContext;

pub trait Procedure<T>: Default {
	fn tick(&mut self, ctx: &mut CwContext<T>, buffer: SymbolString) -> SymbolString;
}

#[derive(Default)]
pub struct Greeting {
	message: String,
}

impl<T> Procedure<T> for Greeting {
	fn tick(&mut self, _: &mut CwContext<T>, _: SymbolString) -> SymbolString {
		SymbolString::try_from(self.message.clone()).unwrap()
	}
}

#[derive(Default)]
pub struct Echo;

impl<T> Procedure<T> for Echo {
	fn tick(&mut self, _: &mut CwContext<T>, buffer: SymbolString) -> SymbolString {
		buffer
	}
}
