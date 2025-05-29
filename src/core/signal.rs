use anyhow::{anyhow, Result};

use super::{
	database::{Config, SignalConfig},
	symbol::{Signals, Symbol, SymbolConverter},
};

pub enum Role {
	User,
	Comp,
}

pub enum SignalChange {
	Remain,   // remaining in the current state
	On(Role), // e.g. a speaker starting to beep - the Role can be used to set the beep frequency
	Off,      // e.g. a speaker seizing to beep
}

#[derive(Default)]
pub struct SignalController {
	symbol_conv: SymbolConverter,
	user_config: SignalMsConfig,
	comp_config: SignalMsConfig,
	elapsed_ms: u32,
	user_signal_buffer: Vec<bool>,
	comp_symbol: Option<Symbol>,
}

impl SignalController {
	pub fn new(config: Config) -> Self {
		Self {
			user_config: SignalMsConfig::from(config.user),
			comp_config: SignalMsConfig::from(config.comp),
			..Default::default()
		}
	}

	pub fn set_comp_symbol(&mut self, symbol: Symbol) -> Result<()> {
		if let Some(current_symbol) = &self.comp_symbol {
			Err(anyhow!(
				"computer is already sending a symbol: '{}'",
				self.symbol_conv.to_char(current_symbol)
			))
		} else {
			self.comp_symbol = Some(symbol);
			Ok(())
		}
	}

	pub fn tick(&mut self, delta: u32, user_signal: bool) -> (SignalChange, Option<Symbol>) {
		self.elapsed_ms += delta;

		let last_user_signal = self.user_signal_buffer.last().unwrap_or(&false);

		match (last_user_signal, user_signal) {
			(false, true) => {
				let symbol = if self.elapsed_ms >= self.user_config.space_ms {
					self.elapsed_ms = 0;
					Some(Symbol::Void)
				} else {
					None
				};

				(SignalChange::On(Role::User), symbol)
			}
			(true, false) => {
				if self.elapsed_ms >= self.user_config.dah_ms {
					self.user_signal_buffer.push(true);
				} else if self.elapsed_ms >= self.user_config.dit_ms {
					self.user_signal_buffer.push(false);
				}

				self.elapsed_ms = 0;

				(SignalChange::Off, None)
			}
			(true, true) => (SignalChange::Remain, None),
			(false, false) => {
				if self.elapsed_ms >= self.user_config.space_ms {
					self.elapsed_ms = 0;
					self.comp_symbol = None; // we may receive new comp input, now that the user is done sending

					let signals = Signals(self.user_signal_buffer.clone());
					self.user_signal_buffer.clear();
					(
						SignalChange::Remain,
						Some(self.symbol_conv.from_signals(signals).unwrap()), // fixme: unwrap
					)
				} else {
					(SignalChange::Remain, None)
				}
			}
		}
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
