use super::{context::CwContext, symbol::Symbol};

pub trait Routine<T> {
	fn tick(&mut self, ctx: &mut CwContext<T>, input: Symbol) -> Symbol;
}

pub struct Greeting {
	message: String,
}

impl<T> Routine<T> for Greeting {
	fn tick(&mut self, ctx: &mut CwContext<T>, _: Symbol) -> Symbol {
		if let Some(c) = self.message.chars().next() {
			ctx.symbol.from_char(c).unwrap()
		} else {
			Symbol::Space
		}
	}
}

pub struct Echo;

impl<T> Routine<T> for Echo {
	fn tick(&mut self, _: &mut CwContext<T>, input: Symbol) -> Symbol {
		input
	}
}
