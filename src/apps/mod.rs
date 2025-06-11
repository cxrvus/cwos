use crate::prelude::*;

#[derive(Default)]
pub struct Echo;

impl CwController<CwString, CwString> for Echo {
	fn tick(&mut self, _ctx: &mut impl CwContext, input: CwString) -> CwString {
		input
	}
}
