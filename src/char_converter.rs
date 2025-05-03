use anyhow::{anyhow, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct CharConverter {
	encoding_map: HashMap<char, String>,
	decoding_map: HashMap<String, char>,
}

impl CharConverter {
	pub fn new() -> Self {
		let mut char_to_morse = HashMap::new();
		let mut morse_to_char = HashMap::new();

		let lines = MORSE_KEY.trim().lines();

		for line in lines {
			let mut kvp = line.split_whitespace();
			let key = kvp.next().unwrap().chars().next().unwrap();
			let val = kvp.next().unwrap().to_string();

			char_to_morse.insert(key, val.clone());
			morse_to_char.insert(val, key);
		}

		Self {
			encoding_map: char_to_morse,
			decoding_map: morse_to_char,
		}
	}

	pub fn encode(&self, chars: &str) -> Result<String> {
		let mut encoded = String::new();

		for char in chars.chars() {
			let char = char.to_ascii_uppercase();

			let morse_char = if char == ' ' {
				Ok("/")
			} else {
				self.encoding_map
					.get(&char)
					.map(|s| s.as_str())
					.ok_or(anyhow!("invalid character: {}", char))
			};

			encoded += morse_char?;
			encoded += " ";
		}

		Ok(encoded.trim().to_string())
	}

	pub fn decode(&self, morse: &str) -> String {
		let mut decoded = String::new();

		for word in morse.split('/') {
			for letter in word.trim().split(' ') {
				decoded.push(*self.decoding_map.get(letter).unwrap_or(&'~'));
			}
			decoded.push(' ');
		}

		decoded.to_ascii_uppercase()
	}
}

const MORSE_KEY: &str = r#"
A .- 
B -... 
C -.-. 
D -.. 
E . 
F ..-. 
G --. 
H .... 
I .. 
J .--- 
K -.- 
L .-.. 
M -- 
N -. 
O --- 
P .--. 
Q --.- 
R .-. 
S ... 
T - 
U ..- 
V ...- 
W .-- 
X -..- 
Y -.-- 
Z --.. 
1 .---- 
2 ..--- 
3 ...-- 
4 ....- 
5 ..... 
6 -.... 
7 --... 
8 ---.. 
9 ----. 
0 -----
. .-.-.- 
, --..-- 
? ..--.. 
! -.-.-- 
/ -..-. 
( -.--. 
) -.--.- 
& .-... 
: ---... 
; -.-.-. 
= -...- 
+ .-.-. 
- -....- 
_ ..--.- 
\" .-..-. 
$ ...-..- 
@ .--.-. 
~ .-.-.-.
"#;
