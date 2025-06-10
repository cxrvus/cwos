use crate::prelude::Config;

pub trait CwContext<Input, Output> {
	/// get current input state
	fn input(&self) -> Input;

	/// set current output state
	fn set_output(&mut self, value: Output);

	/// get current configuration
	fn config(&self) -> &Config;

	/// get current time in milliseconds
	fn time(&self) -> u32;
}
