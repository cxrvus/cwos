use anyhow::{anyhow, Result};

use super::{
	context::CwContext,
	database::{Config, SignalConfig},
	routine::Routine,
	symbol::{self, Signals, Symbol},
};

#[derive(Default)]
pub enum Role {
	#[default]
	User,
	Comp,
}

pub enum SignalState {
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
	elapsed_ms: u32,
	active_role: Role,
	signal_buffer: Signals,
	symbol_buffer: Vec<Symbol>,
}

impl<T: Default, R: Routine<T>> SignalController<T, R> {
	const PAUSE_MS: u32 = 1000;

	pub fn new(config: &Config, routine: R, ctx: CwContext<T>) -> Self {
		Self {
			user_config: SignalMsConfig::from(config.user),
			comp_config: SignalMsConfig::from(config.comp),
			ctx,
			routine,
			..Default::default()
		}
	}

	pub fn tick(&mut self, delta: u32, user_signal: bool) -> SignalState {
		self.elapsed_ms += delta;

		let last_user_signal = self.signal_buffer.0.last().unwrap_or(&false);

		let signal_state = match (last_user_signal, user_signal) {
			(false, true) => {
				if let Role::Comp = self.active_role {
					self.signal_buffer.0.clear();
					self.symbol_buffer.clear();

					self.active_role = Role::User;
				}

				self.elapsed_ms = 0;

				SignalState::On
			}
			(true, false) => {
				if self.elapsed_ms >= self.user_config.dah_ms {
					self.signal_buffer.0.push(true);
				} else if self.elapsed_ms >= self.user_config.dit_ms {
					self.signal_buffer.0.push(false);
				}

				self.elapsed_ms = 0;

				SignalState::Off
			}
			(true, true) => SignalState::Remain,
			(false, false) => {
				if let Role::User = self.active_role {
					if self.elapsed_ms >= self.user_config.break_ms
						&& !self.signal_buffer.0.is_empty()
					{
						let symbol = self.ctx.symbol.from_signals(self.signal_buffer.clone());
						self.symbol_buffer.push(symbol);
						self.signal_buffer.0.clear();
					}

					if self.elapsed_ms >= self.user_config.space_ms {
						if let Some(symbol) = self.symbol_buffer.last() {
							if *symbol != Symbol::Space {
								self.symbol_buffer.push(Symbol::Space);
							}
						}
					}

					if self.elapsed_ms >= Self::PAUSE_MS {
						let response = self.routine.tick(&mut self.ctx, self.symbol_buffer.clone());
						self.symbol_buffer = response;
						self.active_role = Role::Comp;
						self.elapsed_ms = 0;
					}
				}

				SignalState::Remain
			}
		};

		signal_state
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
