use super::{
	context::CwContext,
	database::{Config, SignalConfig},
	routine::Routine,
	symbol::{self, Signals, Symbol, SymbolConverter},
};

#[derive(Default)]
pub enum Role {
	#[default]
	Comp,
	User,
}

pub enum OutputState {
	Remain, // remaining in the current state
	On,     // e.g. a speaker starting to beep
	Off,    // e.g. a speaker seizing to beep
}

#[derive(Default)]
pub struct SignalController<T: Default, R: Routine<T>> {
	user_config: SignalMsConfig,
	comp_config: SignalMsConfig,
	ctx: CwContext<T>,
	routine: R,
	active_role: Role,
	last_input_state: bool,
	buffer: Vec<u32>,
	elapsed_ms: u32,
}

impl<T: Default, R: Routine<T>> SignalController<T, R> {
	pub const MAX_MS: u32 = 1000;

	pub fn new(config: &Config, routine: R, ctx: CwContext<T>) -> Self {
		Self {
			user_config: SignalMsConfig::from(config.user),
			comp_config: SignalMsConfig::from(config.comp),
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
					self.buffer.push(self.elapsed_ms);
					self.elapsed_ms = 0;
					self.last_input_state = input_state;
				}

				match input_state {
					true => OutputState::On,
					false => OutputState::Off,
				}
			}
			(true, true) => OutputState::Remain, // todo: send Error Correction symbol if ms >= max
			(false, false) => {
				if self.elapsed_ms >= Self::MAX_MS {
					let conv = &self.ctx.symbol.clone();
					let user_config = &self.user_config;

					let input_signals = self.buffer.clone();
					let input_symbols =
						Self::durations_to_symbols(conv, user_config, input_signals);

					let output_symbols = self.routine.tick(&mut self.ctx, input_symbols);
					let output_signals =
						Self::symbols_to_durations(conv, user_config, output_symbols);

					self.buffer = output_signals;

					self.active_role = Role::Comp;
					self.elapsed_ms = 0;
				}

				OutputState::Remain
			}
		}
	}

	fn comp_tick(&mut self) -> OutputState {
		todo!()
	}

	fn durations_to_symbols(
		conv: &SymbolConverter,
		config: &SignalMsConfig,
		durations: Vec<u32>,
	) -> Vec<Symbol> {
		let mut signal_on = true;
		let mut signals: Signals = Signals(vec![]);
		let mut symbols: Vec<Symbol> = vec![];

		// todo: handle odd duration counts

		for duration in durations {
			if signal_on {
				signals.0.push(duration >= config.dah_ms); // we get a dah (true) or a dit (false)
				signal_on = false;
			} else {
				if duration > config.break_ms {
					symbols.push(conv.from_signals(&signals));
					signals.0.clear();

					if duration > config.space_ms {
						symbols.push(Symbol::Space);
					}
				}
				signal_on = true;
			}
		}

		symbols
	}

	fn symbols_to_durations(
		conv: &SymbolConverter,
		config: &SignalMsConfig,
		symbols: Vec<Symbol>,
	) -> Vec<u32> {
		let mut signal_on = true;
		let mut durations: Vec<u32> = vec![];
		let mut signals: Signals = Signals(vec![]);

		for symbol in symbols {
			if let Symbol::Space = symbol {
				durations.push(config.space_ms);
			} else {
				let signals = conv.to_signals(&symbol);
				for signal in signals.0 {
					durations.push(match signal {
						true => config.dah_ms,
						false => config.dit_ms,
					});

					durations.push(config.dit_ms); // the duration of silence after each signal is one dit
				}
				durations.push(config.break_ms);
			}
		}

		durations
	}
}

#[derive(Default)]
struct SignalMsConfig {
	dit_ms: u32,
	dah_ms: u32,
	break_ms: u32,
	space_ms: u32,
}

impl From<SignalConfig> for SignalMsConfig {
	fn from(config: SignalConfig) -> Self {
		let unit_ms = wpm_to_ms(config.wpm);
		let fw_unit_ms = wpm_to_ms(config.fw_wpm); // Fansworth

		Self {
			dit_ms: unit_ms,
			dah_ms: unit_ms * 3,
			break_ms: fw_unit_ms * 3,
			space_ms: fw_unit_ms * 7,
		}
	}
}

fn wpm_to_ms(wpm: u32) -> u32 {
	// the 1200 represents the milliseconds per word according to the PARIS standard:
	// 1200 = 60000 [ms in a minute] / 50 [units in the word "PARIS"]
	1200 / wpm
}
