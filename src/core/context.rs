use crate::prelude::CwConfig;

pub trait CwContext {
	/// get current configuration
	fn config(&self) -> CwConfig;

	/// get current time in milliseconds
	fn time(&self) -> u32;
}
