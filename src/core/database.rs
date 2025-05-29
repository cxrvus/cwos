use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
	config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	pub user: PulseConfig,
	pub comp: PulseConfig,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			user: PulseConfig {
				wpm: 15,
				fw_wpm: 10,
				freq: 600,
			},
			comp: PulseConfig {
				wpm: 20,
				fw_wpm: 10,
				freq: 800,
			},
		}
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PulseConfig {
	pub wpm: u32,
	pub fw_wpm: u32, // Fansworth
	pub freq: u32,
}
