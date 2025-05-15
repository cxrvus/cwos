use super::{context::Context, symbol_converter::Symbol};

pub trait Routine {
	fn tick(&mut self, ctx: &mut Context, input: Symbol) -> Symbol;
}
