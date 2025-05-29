use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
	config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub user: SignalConfig,
	pub comp: SignalConfig,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			user: SignalConfig {
				wpm: 15,
				fw_wpm: 10,
				freq: 600,
			},
			comp: SignalConfig {
				wpm: 20,
				fw_wpm: 10,
				freq: 700,
			},
		}
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct SignalConfig {
	pub wpm: u32,
	pub fw_wpm: u32, // Fansworth
	pub freq: u32,
}
