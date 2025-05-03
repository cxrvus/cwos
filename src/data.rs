use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	unit_ms: u32,
	break_units: u32, // for Fansworth timing - defaults to BREAK_UNITS
	user_freq: u32,
	comp_freq: u32, // defaults to user_freq
	output_modes: OutputModes,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct OutputModes {
	light: bool,
	debug: bool,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			unit_ms: 60,
			break_units: 5,
			user_freq: 600,
			comp_freq: 800,
			output_modes: Default::default(),
		}
	}
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Data {
	config: Config,
}

impl Data {}
