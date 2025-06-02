use super::{
	config::{Config, SignalConfig},
	symbol::{ElementString, Symbol, SymbolString},
};

#[derive(Default, PartialEq)]
pub enum Mode {
	#[default]
	Output,
	Input,
}

pub struct OutputState(pub bool);

#[derive(Clone)]
struct Signal {
	is_on: bool,
	duration: u32,
}

#[derive(Default)]
pub struct SignalController {
	input_config: SignalElementConfig,
	output_config: SignalElementConfig,
	mode: Mode,
	buffer: Vec<Signal>,
	elapsed_ms: u32,
}

impl SignalController {
	pub const MAX_MS: u32 = 3000; // todo: make this configurable

	pub fn new(config: &Config) -> Self {
		Self {
			input_config: SignalElementConfig::from(config.input.signal),
			output_config: SignalElementConfig::from(config.output.signal),
			..Default::default()
		}
	}

	fn tick(&mut self, delta_ms: u32, new_mode: Mode) {
		self.elapsed_ms += delta_ms;

		if new_mode != self.mode {
			self.reset();
			self.mode = new_mode;
		}
	}

	pub fn reset(&mut self) {
		self.buffer.clear();
		self.elapsed_ms = 0;
	}

	pub fn input_tick(&mut self, delta_ms: u32, input_state: bool) -> Option<SymbolString> {
		self.tick(delta_ms, Mode::Input);

		let last_input_state = if let Some(signal) = self.buffer.last() {
			signal.is_on
		} else {
			false
		};

		match (last_input_state, input_state) {
			(false, true) | (true, false) => {
				// the if-clause prevents adding an Off-Signal duration to an empty duration buffer
				if !self.buffer.is_empty() || last_input_state {
					self.buffer.push(Signal {
						duration: self.elapsed_ms,
						is_on: input_state,
					});
					self.elapsed_ms = 0;
				}
			}
			(false, false) => {
				// if the user is idle for long enough (but the buffer is not empty, meaning that signals have been transmitted)
				// then pass the input buffer to the procedure and return control
				if !self.buffer.is_empty() && self.elapsed_ms >= Self::MAX_MS {
					let input_signals = self.buffer.clone();
					let input_symbols = self.signals_to_symbols(input_signals);

					self.reset();

					return Some(input_symbols);
				}
			}
			(true, true) => {} // todo: [HH] if ms >= MAX
		}

		None
	}

	pub fn output_tick(&mut self, delta_ms: u32, buffer: Option<SymbolString>) -> Option<bool> {
		self.tick(delta_ms, Mode::Output);

		if let Some(symbols) = buffer {
			let signals = self.symbols_to_signals(symbols);
			self.buffer.extend(signals);
		}

		if let Some(signal) = self.buffer.first() {
			let output_state = signal.is_on;

			if self.elapsed_ms >= signal.duration {
				self.elapsed_ms = 0;

				if !self.buffer.is_empty() {
					self.buffer.remove(0);
				}
			}

			Some(output_state)
		} else {
			self.elapsed_ms = 0;
			self.mode = Mode::Input;

			None
		}
	}

	fn signals_to_symbols(&self, signals: Vec<Signal>) -> SymbolString {
		let config = &self.input_config;

		let mut elements: ElementString = ElementString(vec![]);
		let mut symbols: Vec<Symbol> = vec![];

		for signal in signals {
			if signal.is_on {
				// todo: send Error Correction symbol if ms >= max
				// add a dah (true) or a dit (false)
				elements.0.push(signal.duration >= config.dah_ms);
			} else if signal.duration > config.break_ms {
				// convert elements to a symbol if silence qualifies for a character break
				symbols.push(Symbol::from_elements(&elements));
				elements.0.clear();

				// add a space
				if signal.duration > config.space_ms {
					symbols.push(Symbol::Space);
				}
			}
		}

		SymbolString(symbols)
	}

	fn symbols_to_signals(&self, symbols: SymbolString) -> Vec<Signal> {
		let config = &self.output_config;

		let mut signals: Vec<Signal> = vec![];

		for symbol in symbols.0 {
			if let Symbol::Space = symbol {
				// remove the last silent signal element
				if let Some(last) = signals.last() {
					if !last.is_on {
						signals.pop();
					}
				}

				// add a space
				signals.push(Signal {
					is_on: false,
					duration: config.space_ms,
				});
			} else {
				for signal in symbol.elements().0 {
					// push either a dit or a dah
					signals.push(Signal {
						is_on: true,
						duration: match signal {
							true => config.dah_ms,
							false => config.dit_ms,
						},
					});

					// element break - the duration of silence after each signal element is one dit
					signals.push(Signal {
						is_on: false,
						duration: config.dit_ms,
					});
				}

				// remove the last element break to add a symbol break
				signals.pop();

				// add a break
				signals.push(Signal {
					is_on: false,
					duration: config.break_ms,
				});
			}
		}

		signals
	}
}

#[derive(Default)]
struct SignalElementConfig {
	dit_ms: u32,
	dah_ms: u32,
	break_ms: u32,
	space_ms: u32,
}

impl From<SignalConfig> for SignalElementConfig {
	fn from(config: SignalConfig) -> Self {
		let SignalConfig { unit_ms, fw_ms, .. } = config;

		Self {
			dit_ms: unit_ms,
			dah_ms: unit_ms * 3,
			break_ms: fw_ms * 3,
			space_ms: fw_ms * 7,
		}
	}
}
