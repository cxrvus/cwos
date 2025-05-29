use anyhow::{anyhow, Result};

use super::{
	database::{Config, PulseConfig},
	symbol::{self, PulseSymbol, Symbol, SymbolConverter},
};

struct TimingPulse(u32, u32);

pub enum Role {
	User,
	Comp,
}

pub enum Signal {
	Remain,   // remain in the current state
	On(Role), // e.g. a speaker starting to beep - the Role can be used to change the beep frequency
	Off,      // e.g. a speaker seizing to beep
}

#[derive(Default)]
pub struct CwInput {
	symbol_conv: SymbolConverter,
	config: TimingConfig,
	elapsed_ms: u32,
	user_buffer: Vec<bool>,
	comp_symbol: Option<Symbol>,
}

impl CwInput {
	pub fn new(config: Config) -> Self {
		Self {
			config: TimingConfig::from(config),
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

	pub fn tick(&mut self, delta: u32, pressed: bool) -> (Signal, Option<Symbol>) {
		self.elapsed_ms += delta;

		let last_user_signal = self.user_buffer.last().unwrap_or(&false);

		match (last_user_signal, pressed) {
			(false, true) => {
				let symbol = if self.elapsed_ms >= self.config.user.space_ms {
					self.elapsed_ms = 0;
					Some(Symbol::Void)
				} else {
					None
				};

				(Signal::On(Role::User), symbol)
			}
			(true, false) => {
				if self.elapsed_ms >= self.config.user.dah_ms {
					self.user_buffer.push(true);
				} else if self.elapsed_ms >= self.config.user.dit_ms {
					self.user_buffer.push(false);
				}

				self.elapsed_ms = 0;

				(Signal::Off, None)
			}
			(true, true) => (Signal::Remain, None),
			(false, false) => {
				if self.elapsed_ms >= self.config.user.space_ms {
					self.elapsed_ms = 0;
					self.comp_symbol = None; // we may receive new comp input, now that the user is done sending

					let pulse_symbol = PulseSymbol(self.user_buffer.clone());
					self.user_buffer.clear();
					(
						Signal::Remain,
						Some(self.symbol_conv.from_pulse(pulse_symbol).unwrap()), // fixme: unwrap
					)
				} else {
					(Signal::Remain, None)
				}
			}
		}
	}
}

struct TimingConfig {
	user: TimingPulseConfig,
	comp: TimingPulseConfig,
}

impl From<Config> for TimingConfig {
	fn from(config: Config) -> Self {
		Self {
			user: TimingPulseConfig::from(config.user),
			comp: TimingPulseConfig::from(config.comp),
		}
	}
}

impl Default for TimingConfig {
	fn default() -> Self {
		Self::from(Config::default())
	}
}

struct TimingPulseConfig {
	dit_ms: u32,
	dah_ms: u32,
	break_ms: u32,
	space_ms: u32,
}

impl From<PulseConfig> for TimingPulseConfig {
	fn from(config: PulseConfig) -> Self {
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
