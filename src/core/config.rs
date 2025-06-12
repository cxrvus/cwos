use serde::{Deserialize, Serialize};

/// calculates the ms per unit for a given WPM
///
/// ## Formula
/// `ms/u = 1200/WPM`
///
/// => the 1200 represents the milliseconds per word according to the PARIS standard:
///
/// => 1200 = 60000 (ms in a minute) / 50 (units in the word "PARIS")
#[macro_export]
macro_rules! wpm {
	($wpm:expr) => {
		1200 / $wpm
	};
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CwConfig {
	pub input: InputConfig,
	pub output: OutputConfig,
}

impl Default for CwConfig {
	fn default() -> Self {
		Self {
			output: OutputConfig {
				signal: SignalConfig {
					unit_ms: wpm!(20),
					fw_ms: wpm!(10),
					freq: 650,
				},
			},
			input: InputConfig {
				signal: SignalConfig {
					unit_ms: wpm!(15),
					fw_ms: wpm!(10),
					freq: 550,
				},
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
	/// beep frequency
	pub freq: u32,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct OutputConfig {
	pub signal: SignalConfig,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct InputConfig {
	pub signal: SignalConfig,
	// impl this to replace const MAX_MS
	// pub idle_ms: u32,
	// impl this to send Error Correction
	// pub correction_ms: u32,
}
