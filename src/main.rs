mod morse;

fn main() {
	let parser = morse::Parser::new();

	let message = "hello world!";
	let encoded = parser.encode(message).unwrap();
	println!("{}", encoded);
	let decoded = parser.decode(&encoded);
	println!("{}", decoded);
}
