use super::{
	config::{Config, SignalConfig},
	context::CwContext,
	routine::Routine,
	symbol::{SignalElements, Symbol, SymbolConverter},
};

#[derive(Default)]
pub enum Role {
	#[default]
	Comp,
	User,
}

pub struct OutputState(pub bool);

#[derive(Clone)]
struct Signal {
	is_on: bool,
	duration: u32,
}

#[derive(Default)]
pub struct SignalController<T: Default, R: Routine<T>> {
	user_config: SignalElementConfig,
	comp_config: SignalElementConfig,
	ctx: CwContext<T>,
	routine: R,
	active_role: Role,
	last_input_state: bool,
	buffer: Vec<Signal>,
	elapsed_ms: u32,
}

impl<T: Default, R: Routine<T>> SignalController<T, R> {
	pub const MAX_MS: u32 = 3000;

	pub fn new(config: &Config, routine: R, ctx: CwContext<T>) -> Self {
		Self {
			user_config: SignalElementConfig::from(config.user_signal),
			comp_config: SignalElementConfig::from(config.comp_signal),
			ctx,
			routine,
			..Default::default()
		}
	}

	pub fn tick(&mut self, input_state: bool, delta_ms: u32) -> OutputState {
		self.elapsed_ms += delta_ms;

		if let Role::Comp = self.active_role {
			if input_state {
				// if the user inputs the signal, put them back into control
				self.buffer.clear();
				self.active_role = Role::User;
				self.user_tick(input_state)
			} else {
				self.comp_tick()
			}
		} else {
			self.user_tick(input_state)
		}
	}

	fn user_tick(&mut self, input_state: bool) -> OutputState {
		match (self.last_input_state, input_state) {
			(false, true) | (true, false) => {
				// the if-clause prevents adding an Off-Signal duration to an empty duration buffer
				if !self.buffer.is_empty() || self.last_input_state {
					self.buffer.push(Signal {
						duration: self.elapsed_ms,
						is_on: input_state,
					});
					self.elapsed_ms = 0;
					self.last_input_state = input_state;
				}
			}
			(false, false) => {
				// if the user is idle for long enough (but the buffer is not empty, meaning that signals have been transmitted)
				// then send the user's buffer to the computer and return control
				if !self.buffer.is_empty() && self.elapsed_ms >= Self::MAX_MS {
					let conv = &self.ctx.symbol.clone();

					let input_signals = self.buffer.clone();
					let input_symbols =
						Self::signals_to_symbols(conv, &self.user_config, input_signals);

					let output_symbols = self.routine.tick(&mut self.ctx, input_symbols);
					let output_signals =
						Self::symbols_to_signals(conv, &self.comp_config, output_symbols);

					self.buffer = output_signals;

					self.active_role = Role::Comp;
					self.elapsed_ms = 0;
				}
			}
			(true, true) => {}
		}

		OutputState(input_state)
	}

	fn comp_tick(&mut self) -> OutputState {
		if let Some(signal) = self.buffer.first() {
			let output_state = signal.is_on;

			if self.elapsed_ms >= signal.duration {
				self.elapsed_ms = 0;

				if !self.buffer.is_empty() {
					self.buffer.remove(0);
				}
			}

			OutputState(output_state)
		} else {
			self.elapsed_ms = 0;
			self.active_role = Role::User;

			OutputState(false)
		}
	}

	fn signals_to_symbols(
		conv: &SymbolConverter,
		config: &SignalElementConfig,
		signals: Vec<Signal>,
	) -> Vec<Symbol> {
		let mut elements: SignalElements = SignalElements(vec![]);
		let mut symbols: Vec<Symbol> = vec![];

		for signal in signals {
			if signal.is_on {
				// todo: send Error Correction symbol if ms >= max
				// add a dah (true) or a dit (false)
				elements.0.push(signal.duration >= config.dah_ms);
			} else if signal.duration > config.break_ms {
				// convert elements to a symbol if silence qualifies for a character break
				symbols.push(conv.from_elements(&elements));
				elements.0.clear();

				// add a space
				if signal.duration > config.space_ms {
					symbols.push(Symbol::Space);
				}
			}
		}

		symbols
	}

	fn symbols_to_signals(
		conv: &SymbolConverter,
		config: &SignalElementConfig,
		symbols: Vec<Symbol>,
	) -> Vec<Signal> {
		let mut signals: Vec<Signal> = vec![];

		for symbol in symbols {
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
				let elements = conv.to_elements(&symbol);
				for signal in elements.0 {
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
		let SignalConfig { unit_ms, fw_ms } = config;

		Self {
			dit_ms: unit_ms,
			dah_ms: unit_ms * 3,
			break_ms: fw_ms * 3,
			space_ms: fw_ms * 7,
		}
	}
}
