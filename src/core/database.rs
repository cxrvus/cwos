use crate::core::config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Database {
	config: Config,
}
