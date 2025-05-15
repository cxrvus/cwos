use super::{context::Context, symbol::Symbol};

pub trait Routine {
	fn tick(&mut self, ctx: &mut Context, input: Symbol) -> Symbol;
}

pub struct Greeting {
	message: String,
}

impl Routine for Greeting {
	fn tick(&mut self, _: &mut Context, _: Symbol) -> Symbol {
		if let Some(c) = self.message.chars().next() {
			Symbol::Sign(c)
		} else {
			Symbol::Break
		}
	}
}

pub struct Echo;

impl Routine for Echo {
	fn tick(&mut self, _: &mut Context, input: Symbol) -> Symbol {
		input
	}
}
