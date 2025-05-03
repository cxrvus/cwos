mod char_converter;

fn main() {
	let conv = char_converter::CharConverter::new();

	let message = "hello world!";
	let encoded = conv.encode(message).unwrap();
	println!("{}", encoded);
	let decoded = conv.decode(&encoded);
	println!("{}", decoded);
}
