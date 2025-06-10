use crate::prelude::*;

#[derive(Default)]
pub struct Echo;

impl TextController for Echo {
	fn tick(&mut self, ctx: &mut impl CwContext<CwString, CwString>) {
		ctx.set_output(ctx.input());
	}
}
