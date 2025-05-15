use std::{
	fs::{read_to_string, write},
	path::PathBuf,
};

use crate::core::database::Database;

const DB_FILE: &str = "nothingdb.json";

struct FsDatabase(Database);

impl FsDatabase {
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
		let db = serde_json::to_string_pretty(&self.0).expect("failed to serialize db file");
		write(path, db).expect("failed to write to db file");
	}
}
