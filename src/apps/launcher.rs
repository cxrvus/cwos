use crate::prelude::*;

#[derive(Default)]
pub struct AppLauncher {
	selected_app: Option<String>,
}

impl CwController for AppLauncher {
	fn tick(&mut self, input: SymbolString) -> SymbolString {
		let input = input.normalized();
		if input.0.is_empty() {
			return SymbolString::default();
		}

		match self.selected_app {
			Some(ref app_name) => match app_name.as_str() {
				"EC" => Echo.tick(input),
				_ => SymbolString::try_from("?".to_string()).unwrap(),
			},
			None => match input.as_string().as_str() {
				app_name @ "EC" => {
					self.selected_app = Some(app_name.into());
					app_name.to_string().try_into().unwrap()
				}
				_ => SymbolString::try_from("?".to_string()).unwrap(),
			},
		}
	}
}
