use crate::prelude::{Config, SymbolString};
use core::marker::PhantomData;

#[derive(Default)]
pub struct InputData;
#[derive(Default)]
pub struct OutputData;

#[derive(Default)]
struct CwContext<State = OutputData> {
	input: SymbolString,
	output: SymbolString,
	config: Config,
	state: PhantomData<State>,
}

pub type InputContext = CwContext<InputData>;
pub type OutputContext = CwContext<OutputData>;

impl<State: Default> CwContext<State> {
	pub fn new(config: Config) -> Self {
		Self {
			config,
			..Default::default()
		}
	}

	pub fn output(&self) -> &SymbolString {
		&self.output
	}

	pub fn input(&self) -> &SymbolString {
		&self.input
	}

	pub fn config(&self) -> &Config {
		&self.config
	}
}

impl InputContext {
	pub fn set_output(&mut self, value: SymbolString) {
		self.output = value;
	}

	pub fn set_config(&mut self, value: Config) {
		self.config = value;
	}
}

impl OutputContext {
	pub fn set_input(&mut self, value: SymbolString) {
		self.input = value;
	}
}
