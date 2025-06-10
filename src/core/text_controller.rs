use crate::{apps::Echo, prelude::*};

pub trait TextController: Default {
	fn tick(&mut self, ctx: &mut impl CwContext<CwString, CwString>);
}

#[derive(Default)]
pub struct AppLauncher {
	selected_app: Option<String>,
}

impl TextController for AppLauncher {
	fn tick(&mut self, ctx: &mut impl CwContext<CwString, CwString>) {
		match self.selected_app {
			Some(ref app_name) => match app_name.as_str() {
				"EC" => Echo.tick(ctx),
				_ => idk(ctx),
			},
			None => match ctx.input().as_string().as_str() {
				app_name @ "EC" => {
					self.selected_app = Some(app_name.into());
					ctx.set_output(app_name.to_string().try_into().unwrap())
				}
				"" => Default::default(),
				_ => idk(ctx),
			},
		}
	}
}

fn idk(ctx: &mut impl CwContext<CwString, CwString>) {
	ctx.set_output(CwString(vec![Symbol::Question]));
}
