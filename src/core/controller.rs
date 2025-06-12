use crate::prelude::Config;

pub trait CwController<Input, Output>: Default {
	fn tick(&mut self, ctx: &mut impl CwContext, input: Input) -> Output;
}

pub trait CwContext {
	/// get current configuration
	fn config(&self) -> Config;

	/// get current time in milliseconds
	fn time(&self) -> u32;
}
