use std::{
	fs::{read_to_string, write},
	path::PathBuf,
};

use serde::{de::DeserializeOwned, Serialize};

const DB_FILE: &str = "cwos_db.json";

pub struct FsDatabase<T>(T)
where
	T: Default + Serialize + DeserializeOwned;

impl<T> FsDatabase<T>
where
	T: Default + Serialize + DeserializeOwned,
{
	fn path() -> PathBuf {
		let mut path = dirs::home_dir().expect("could not get HOME directory");
		path.push(DB_FILE);
		path
	}

	pub fn load() -> T {
		let path = Self::path();
		if !path.exists() {
			T::default()
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
