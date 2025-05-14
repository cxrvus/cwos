use controller::Controller;
use database::Database;

mod controller;
mod database;
mod symbol_converter;

fn main() {
	let db = Database::load();
	let ctl = Controller::new(db);
}
