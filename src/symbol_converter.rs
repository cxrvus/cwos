use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::{self, Formatter};

pub struct SymbolConverter {
	encoding_map: HashMap<PlainSymbol, Vec<bool>>,
	decoding_map: HashMap<Vec<bool>, PlainSymbol>,
}

impl SymbolConverter {
	pub fn new() -> Self {
		let mut encoding_map = HashMap::new();
		let mut decoding_map = HashMap::new();

		let lines = CW_SPEC.trim().lines();

		for line in lines {
			if line.is_empty() {
				continue;
			}

			let mut kvp = line.split_whitespace();

			let key = kvp.next().unwrap(); //.chars().next().unwrap();

			let key = if key.len() == 1 {
				PlainSymbol::Sign(key.chars().next().unwrap())
			} else {
				PlainSymbol::Prosign(key.to_string())
			};

			let val = kvp.next().map(CwSymbol::pulses_from_str).unwrap();

			encoding_map.insert(key.clone(), val.clone());
			decoding_map.insert(val, key);
		}

		Self {
			encoding_map,
			decoding_map,
		}
	}

	pub fn encode(&self, plain: PlainSymbol) -> Result<CwSymbol> {
		if let PlainSymbol::Space = plain {
			Ok(CwSymbol::Break)
		} else {
			self.encoding_map
				.get(&plain)
				.cloned()
				.map(CwSymbol::Pulses)
				.ok_or(anyhow!("invalid plaintext symbol: {}", plain.to_string()))
		}
	}

	pub fn decode(&self, cw: CwSymbol) -> Result<PlainSymbol> {
		if let CwSymbol::Pulses(ref pulses) = cw {
			self.decoding_map
				.get(pulses)
				.cloned()
				.ok_or(anyhow!("invalid CW sequence: {}", cw.to_string()))
		} else {
			Ok(PlainSymbol::Space)
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PlainSymbol {
	Sign(char),
	Prosign(String),
	Space,
}

impl fmt::Display for PlainSymbol {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Sign(c) => c.to_string(),
			Self::Prosign(s) => s.clone(),
			Self::Space => " ".into(),
		};
		write!(f, "{}", s)
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum CwSymbol {
	Pulses(Vec<bool>),
	Break,
}

impl CwSymbol {
	pub fn pulses_from_str(pulse_str: &str) -> Vec<bool> {
		let mut pulses = vec![];

		for pulse_char in pulse_str.chars() {
			let pulse = match pulse_char {
				'.' => false,
				'-' => true,
				_ => panic!("char may only be '.' or '-'"),
			};

			pulses.push(pulse);
		}

		pulses
	}
}

impl fmt::Display for CwSymbol {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Break => " / ".to_string(),
			Self::Pulses(pulses) => pulses
				.iter()
				.map(|&pulse| if pulse { "-" } else { "." })
				.collect::<String>(),
		};
		write!(f, "{}", s)
	}
}

const CW_SPEC: &str = r#"
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
$ ...-..-
@ .--.-.
~ .-.-.-.

[SOS] ...---...
"#;

#[cfg(test)]
mod tests {

	#[test]
	fn test_char_converter_encode_decode() {
		use super::SymbolConverter;

		let conv = SymbolConverter::new();

		let message = "HELLO WORLD!";
		let encoded = conv.encode(message).unwrap();

		assert!(!encoded.is_empty(), "encoded message should not be empty");

		let decoded = conv.decode(&encoded);

		assert_eq!(
			decoded, message,
			"decoded message should match the original"
		);

		println!("{encoded}\n{decoded}");
	}
}
