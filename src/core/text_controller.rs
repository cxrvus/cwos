use crate::{apps::Echo, prelude::*};

#[derive(Default)]
pub struct AppLauncher {
	selected_app: Option<String>,
}

impl CwController<CwString, CwString> for AppLauncher {
	fn tick(&mut self, ctx: &mut impl CwContext, input: CwString) -> CwString {
		match self.selected_app {
			Some(ref app_name) => match app_name.as_str() {
				"EC" => Echo.tick(ctx, input),
				_ => idk(),
			},
			None => match input.normalized().as_string().as_str() {
				app_name @ "EC" => {
					self.selected_app = Some(app_name.into());
					app_name.to_string().try_into().unwrap()
				}
				"" => Default::default(),
				_ => idk(),
			},
		}
	}
}

fn idk() -> CwString {
	CwString(vec![Symbol::Question])
}
