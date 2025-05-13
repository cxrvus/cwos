use controller::Controller;
use database::Database;

mod char_converter;
mod controller;
mod database;

fn main() {
	let db = Database::load();
	let ctl = Controller::new(db);
}
