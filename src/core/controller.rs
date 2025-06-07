use crate::prelude::InputContext;

pub trait CwController: Default {
	fn tick(&mut self, ctx: &mut InputContext);
}

#[derive(Default)]
pub struct Echo;

impl CwController for Echo {
	fn tick(&mut self, ctx: &mut InputContext) {
		ctx.set_output(ctx.input().normalized());
	}
}
