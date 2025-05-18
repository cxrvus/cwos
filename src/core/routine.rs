use super::{context::CwContext, symbol::Symbol};

pub trait Routine {
	fn tick(&mut self, ctx: &mut CwContext, input: Symbol) -> Symbol;
}

pub struct Greeting {
	message: String,
}

impl Routine for Greeting {
	fn tick(&mut self, ctx: &mut CwContext, _: Symbol) -> Symbol {
		if let Some(c) = self.message.chars().next() {
			ctx.symbol.from_char(c).unwrap()
		} else {
			Symbol::Space
		}
	}
}

pub struct Echo;

impl Routine for Echo {
	fn tick(&mut self, _: &mut CwContext, input: Symbol) -> Symbol {
		input
	}
}
