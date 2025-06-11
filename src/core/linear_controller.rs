use crate::prelude::*;

use super::{
	config::SignalConfig,
	symbol::{CwString, ElementString, Symbol},
};

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Mode {
	#[default]
	Input,
	Output,
}

#[derive(Clone, Debug)]
struct Signal<T> {
	value: T,
	duration: u32,
}

#[derive(Default)]
pub struct LinearController<C: TextController> {
	controller: C,
	mode: Mode,
	buffer: Vec<Signal<bool>>,
	last_input_state: bool,
	elapsed_ms: u32,
	last_time: u32,
}

#[derive(Default)]
struct TextContext {
	input: CwString,
	output: CwString,
	config: Config,
	time: u32,
}

impl CwContext<CwString, CwString> for TextContext {
	fn input(&self) -> CwString {
		self.input.clone()
	}

	fn set_output(&mut self, value: CwString) {
		self.output = value;
	}

	fn config(&self) -> &Config {
		&self.config
	}

	fn time(&self) -> u32 {
		self.time
	}
}

type TickCallback<'a> = &'a mut dyn FnMut(CwString) -> CwString;

impl<C: TextController> LinearController<C> {
	pub const MAX_MS: u32 = 3000; // todo: make this configurable

	pub fn tick(&mut self, outer_ctx: &mut impl CwContext<bool, Option<u32>>) {
		let time = outer_ctx.time();
		if self.last_time == 0 {
			self.last_time = time;
		}
		self.elapsed_ms = time - self.last_time;

		let input = outer_ctx.input();

		let signal_on = match self.mode {
			Mode::Input => self.input_tick(outer_ctx),
			Mode::Output => self.output_tick(outer_ctx),
		};

		let signal = match signal_on {
			true => Some(match self.mode {
				Mode::Input => outer_ctx.config().input.signal.freq,
				Mode::Output => outer_ctx.config().output.signal.freq,
			}),
			false => None,
		};

		outer_ctx.set_output(signal);
	}

	pub fn get_mode(&self) -> Mode {
		self.mode.clone()
	}

	pub fn new(controller: C) -> Self {
		Self {
			controller,
			..Default::default()
		}
	}

	pub fn reset(&mut self) {
		self.buffer.clear();
		self.last_input_state = false;
		self.elapsed_ms = 0;
	}

	fn input_tick(&mut self, outer_ctx: &mut impl CwContext<bool, Option<u32>>) -> bool {
		let input_state = outer_ctx.input();
		let last_input_state = self.last_input_state;

		match (last_input_state, input_state) {
			(false, true) | (true, false) => {
				self.buffer.push(Signal {
					duration: self.elapsed_ms,
					value: last_input_state,
				});

				self.elapsed_ms = 0;
				dbg!(&self.buffer);
			}
			(false, false) => {
				// if the user is idle for long enough
				// then pass the input buffer to the procedure and return control
				if self.elapsed_ms >= Self::MAX_MS {
					self.buffer.push(Signal {
						duration: self.elapsed_ms,
						value: false,
					});

					let input_signals = self.buffer.clone();
					let input = Self::signals_to_symbols(input_signals, outer_ctx.config());

					let mut text_ctx = TextContext {
						input,
						time: outer_ctx.time(),
						config: outer_ctx.config().to_owned(),
						..Default::default()
					};

					self.controller.tick(&mut text_ctx);
					let output = Self::symbols_to_signals(text_ctx.output, outer_ctx.config());

					self.reset();
					self.mode = Mode::Output;

					self.buffer = output;
				}
			}
			(true, true) => {}
		}

		self.last_input_state = input_state;
		input_state
	}

	fn output_tick(&mut self, outer_ctx: &mut impl CwContext<bool, Option<u32>>) -> bool {
		let input_state = outer_ctx.input();

		if input_state {
			self.mode = Mode::Input;
			self.reset();
			return self.input_tick(outer_ctx);
		}

		if let Some(signal) = self.buffer.first() {
			let output_state = signal.value;

			if self.elapsed_ms >= signal.duration {
				self.elapsed_ms = 0;

				if !self.buffer.is_empty() {
					self.buffer.remove(0);
				}
			}

			output_state
		} else {
			self.reset();
			self.mode = Mode::Input;
			false
		}
	}

	fn signals_to_symbols(signals: Vec<Signal<bool>>, config: &Config) -> CwString {
		let config = SignalElementConfig::from(config.input.signal);

		let mut elements: ElementString = ElementString(vec![]);
		let mut symbols: Vec<Symbol> = vec![];

		for signal in signals {
			if signal.value {
				// todo: send Error Correction symbol if ms >= max
				// add a dah (true) or a dit (false)
				elements.0.push(signal.duration >= config.dah_ms);
			} else if signal.duration >= config.break_ms {
				dbg!(elements.clone());
				// convert elements to a symbol if silence qualifies for a character break
				symbols.push(Symbol::from_elements(&elements));
				elements.0.clear();

				// add a space
				if signal.duration > config.space_ms {
					symbols.push(Symbol::Space);
				}
			}
		}

		CwString(symbols)
	}

	fn symbols_to_signals(symbols: CwString, config: &Config) -> Vec<Signal<bool>> {
		let config = SignalElementConfig::from(config.output.signal);

		let mut signals: Vec<Signal<bool>> = vec![];

		for symbol in symbols.0 {
			if let Symbol::Space = symbol {
				// remove the last silent signal element
				if let Some(last) = signals.last() {
					if !last.value {
						signals.pop();
					}
				}

				// add a space
				signals.push(Signal {
					value: false,
					duration: config.space_ms,
				});
			} else {
				for signal in symbol.elements().0 {
					// push either a dit or a dah
					signals.push(Signal {
						value: true,
						duration: match signal {
							true => config.dah_ms,
							false => config.dit_ms,
						},
					});

					// element break - the duration of silence after each signal element is one dit
					signals.push(Signal {
						value: false,
						duration: config.dit_ms,
					});
				}

				// remove the last element break to add a symbol break
				signals.pop();

				// add a break
				signals.push(Signal {
					value: false,
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
