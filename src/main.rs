use context::Context;
use database::Database;

mod context;
mod database;
mod routine;
mod symbol_converter;

fn main() {
	let db = Database::load();
	let ctl = Context::new(db);
}
