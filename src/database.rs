use std::{
	fs::{read_to_string, write},
	path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
	unit_ms: u32,
	break_units: u32, // for Fansworth timing
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
pub struct Database {
	config: Config,
}

const DB_FILE: &str = "nothingdb.json";

impl Database {
	fn path() -> PathBuf {
		let mut path = dirs::home_dir().unwrap();
		path.push(DB_FILE);
		path
	}

	pub fn load() -> Database {
		let path = Self::path();
		if !path.exists() {
			Database::default()
		} else {
			let string = read_to_string(path).expect("failed to read db file");
			serde_json::from_str(&string).expect("invalid db file")
		}
	}

	pub fn save(&self) {
		let path = Self::path();
		let db = serde_json::to_string_pretty(self).expect("failed to serialize db file");
		write(path, db).expect("failed to write to db file");
	}
}
