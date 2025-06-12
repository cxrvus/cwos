use crate::prelude::CwContext;

pub trait CwController<Input, Output>: Default {
	fn tick(&mut self, ctx: &mut impl CwContext, input: Input) -> Output;
}
