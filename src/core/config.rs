use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! wpm {
	($wpm:expr) => {
		1200 / $wpm
	};
}

/// calculates the ms per unit for a given WPM
pub fn wpm_to_ms(wpm: u32) -> u32 {
	// the 1200 represents the milliseconds per word according to the PARIS standard:
	// 1200 = 60000 [ms in a minute] / 50 [units in the word "PARIS"]
	wpm!(wpm)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub user_signal: SignalConfig,
	pub comp_signal: SignalConfig,
	pub output: OutputConfig,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			user_signal: SignalConfig {
				unit_ms: wpm!(15),
				fw_ms: wpm!(10),
			},
			comp_signal: SignalConfig {
				unit_ms: wpm!(20),
				fw_ms: wpm!(10),
			},
			output: OutputConfig {
				user_freq: 500,
				comp_freq: 600,
			},
		}
	}
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct SignalConfig {
	/// ms per unit
	pub unit_ms: u32,
	/// Fansworth ms per unit
	pub fw_ms: u32,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct OutputConfig {
	pub user_freq: u32,
	pub comp_freq: u32,
}
