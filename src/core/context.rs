use crate::prelude::Config;

pub trait CwContext {
	fn config(&self) -> Config;
	fn linear_input(&self) -> bool;
	fn set_linear_output(&mut self, value: bool);
}
