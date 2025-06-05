use crate::prelude::*;

#[derive(Default)]
pub struct AppLauncher {
	selected_app: Option<String>,
}

impl CwController for AppLauncher {
	fn tick(&mut self, input: SymbolString) -> Response {
		let input = input.normalized(); // TODO: move normalization into Services

		match self.selected_app {
			Some(ref app_name) => match app_name.as_str() {
				"EC" => Echo.tick(input),
				_ => idk(),
			},
			None => match input.as_string().as_str() {
				app_name @ "EC" => {
					self.selected_app = Some(app_name.into());
					Response::new(app_name.to_string().try_into().unwrap())
				}
				_ => idk(),
			},
		}
	}
}

fn idk() -> Response {
	Response::new(SymbolString::try_from("?".to_string()).unwrap())
}
