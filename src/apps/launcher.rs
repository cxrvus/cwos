use super::context::AppContext;
use crate::core::{controller::CwController, symbol::SymbolString};

#[derive(Default)]
pub struct AppLauncher;

impl CwController<AppContext> for AppLauncher {
	fn tick(&mut self, ctx: &mut AppContext, input: SymbolString) -> SymbolString {
		todo!();
	}
}
