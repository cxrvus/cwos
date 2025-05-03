mod char_converter;
mod controller;
mod database;

fn main() {
	let conv = char_converter::CharConverter::new();

	let message = "hello world!";
	let encoded = conv.encode(message).unwrap();
	println!("{}", encoded);
	let decoded = conv.decode(&encoded);
	println!("{}", decoded);
}
