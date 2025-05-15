use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fmt::{self, Formatter};

pub struct SymbolConverter {
	encoding_map: HashMap<Symbol, Vec<bool>>,
	decoding_map: HashMap<Vec<bool>, Symbol>,
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
				Symbol::Sign(key.chars().next().unwrap())
			} else {
				Symbol::Prosign(key.to_string())
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

	pub fn encode(&self, plain: Symbol) -> Result<CwSymbol> {
		if let Symbol::Break = plain {
			Ok(CwSymbol::Break)
		} else {
			self.encoding_map
				.get(&plain)
				.cloned()
				.map(CwSymbol::Pulses)
				.ok_or(anyhow!("invalid plaintext symbol: {}", plain.to_string()))
		}
	}

	pub fn decode(&self, cw: CwSymbol) -> Result<Symbol> {
		if let CwSymbol::Pulses(ref pulses) = cw {
			self.decoding_map
				.get(pulses)
				.cloned()
				.ok_or(anyhow!("invalid CW sequence: {}", cw.to_string()))
		} else {
			Ok(Symbol::Break)
		}
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Symbol {
	Sign(char),
	Prosign(String),
	Break,
}

impl fmt::Display for Symbol {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Sign(c) => c.to_string(),
			Self::Prosign(s) => s.clone(),
			Self::Break => " ".into(),
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

[AS] .-...
[CT] -.-.-
[EC] .-.-.
[RT] .-.-
[VA] ...-.-
[SOS] ...---...
"#;

// ## Prosigns:
// [AS] = wait
// [CT] = commencing transmission
// [EC] = end copy / new page
// [RT] = new line
// [VA] = end of contact

#[cfg(test)]
mod tests {}
