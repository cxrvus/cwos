use crate::prelude::*;
use std::{process::exit, time::SystemTime};

#[derive(Default)]
pub struct StdContext;

impl CwContext for StdContext {
	fn config(&self) -> CwConfig {
		CwConfig::default()
	}

	fn time(&self) -> u32 {
		SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.map(|d| d.as_millis() as u32)
			.unwrap()
	}

	fn quit(&self) {
		exit(0);
	}
}
