use super::{context::CwContext, symbol::Symbol};

pub trait Routine<T>: Default {
	fn tick(&mut self, ctx: &mut CwContext<T>, buffer: Vec<Symbol>) -> Vec<Symbol>;
}

#[derive(Default)]
pub struct Greeting {
	message: String,
}

impl<T> Routine<T> for Greeting {
	fn tick(&mut self, ctx: &mut CwContext<T>, _: Vec<Symbol>) -> Vec<Symbol> {
		ctx.symbol.from_str(&self.message).unwrap()
	}
}

#[derive(Default)]
pub struct Echo;

impl<T> Routine<T> for Echo {
	fn tick(&mut self, _: &mut CwContext<T>, buffer: Vec<Symbol>) -> Vec<Symbol> {
		buffer
	}
}
