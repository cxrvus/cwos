use crate::prelude::CwContext;

pub trait CwController: Default {
	fn tick(&mut self, ctx: &mut impl CwContext);
}

#[derive(Default)]
pub struct Echo;

impl CwController for Echo {
	fn tick(&mut self, ctx: &mut impl CwContext) {
		ctx.set_linear_output(true);
	} //fixme
}
