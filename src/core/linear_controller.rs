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
pub struct LinearController<C: CwController<CwString, CwString>> {
	controller: C,
	mode: Mode,
	buffer: Vec<Signal<bool>>,
	last_input_state: bool,
	elapsed_ms: u32,
	last_time: u32,
}

impl<C: CwController<CwString, CwString>> LinearController<C> {
	pub const MAX_MS: u32 = 3000; // todo: make this configurable

	// TODO: move to trait impl
	pub fn tick(&mut self, ctx: &mut impl CwContext, input: bool) -> Option<u32> {
		let time = ctx.time();
		if self.last_time == 0 {
			self.last_time = time;
		}
		let delta_ms = time - self.last_time;
		self.elapsed_ms += delta_ms;

		let signal_on = match self.mode {
			Mode::Input => self.input_tick(ctx, input),
			Mode::Output => self.output_tick(ctx, input),
		};

		let signal = match signal_on {
			true => Some(match self.mode {
				Mode::Input => ctx.config().input.signal.freq,
				Mode::Output => ctx.config().output.signal.freq,
			}),
			false => None,
		};

		self.last_time = time;

		signal
	}

	pub fn new(controller: C) -> Self {
		Self {
			controller,
			..Default::default()
		}
	}

	pub fn get_mode(&self) -> Mode {
		self.mode.clone()
	}

	pub fn reset(&mut self) {
		self.buffer.clear();
		self.last_input_state = false;
		self.elapsed_ms = 0;
	}

	fn input_tick(&mut self, ctx: &mut impl CwContext, input_state: bool) -> bool {
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
					let input = Self::signals_to_symbols(input_signals, &ctx.config());

					let output = self.controller.tick(ctx, input);
					let output_signals = Self::symbols_to_signals(output, &ctx.config());

					self.reset();
					self.mode = Mode::Output;

					self.buffer = output_signals;
				}
			}
			(true, true) => {}
		}

		self.last_input_state = input_state;
		input_state
	}

	fn output_tick(&mut self, ctx: &mut impl CwContext, input_state: bool) -> bool {
		if input_state {
			self.mode = Mode::Input;
			self.reset();
			return self.input_tick(ctx, input_state);
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
