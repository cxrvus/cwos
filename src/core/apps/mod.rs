use crate::prelude::*;

#[derive(Default)]
pub struct AppLauncher {
	selected_app: Option<CwString>,
}

impl CwController<CwString, CwString> for AppLauncher {
	fn tick(&mut self, ctx: &mut impl CwContext, input: CwString) -> CwString {
		use CwSymbol::*;

		match &self.selected_app {
			Some(app_name) => match app_name.0.as_slice() {
				[E, C] => Echo.tick(ctx, input),
				_ => idk(),
			},
			None => match input.normalized().0.as_slice() {
				app_name @ [E, C] => {
					self.selected_app = Some(CwString(app_name.to_vec()));
					CwString(app_name.to_vec())
				}
				[] => Default::default(),
				_ => idk(),
			},
		}
	}
}

fn idk() -> CwString {
	CwString(vec![CwSymbol::Question])
}

#[derive(Default)]
pub struct Echo;

impl CwController<CwString, CwString> for Echo {
	fn tick(&mut self, _ctx: &mut impl CwContext, input: CwString) -> CwString {
		input
	}
}
