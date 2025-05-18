use anyhow::{anyhow, Result};

// idea: remove std stuff for embedded
use std::fmt::{self, Formatter};

struct SymbolSpec {
	symbol: Symbol,
	group: Group,
	char: char,
	pulse: PulseSymbol,
}

pub struct SymbolConverter {
	symbol_map: Vec<SymbolSpec>,
}

impl SymbolConverter {
	pub fn from_pulse(&self, pulse: PulseSymbol) -> Result<Symbol> {
		if let Some(spec) = self.symbol_map.iter().find(|spec| spec.pulse == pulse) {
			Ok(spec.symbol.clone())
		} else {
			Err(anyhow!("invalid CW sequence: {}", pulse.to_string()))
		}
	}

	pub fn from_char(&self, char: char) -> Result<Symbol> {
		if let Some(spec) = self
			.symbol_map
			.iter()
			.find(|spec| spec.char == char.to_ascii_uppercase())
		{
			Ok(spec.symbol.clone())
		} else {
			Err(anyhow!("invalid plaintext char: {:?}", char))
		}
	}

	pub fn to_char(&self, symbol: Symbol) -> char {
		self.symbol_map
			.iter()
			.find(|spec| spec.symbol == symbol)
			.unwrap()
			.char
	}

	pub fn to_pulse(&self, symbol: Symbol) -> PulseSymbol {
		self.symbol_map
			.iter()
			.find(|spec| spec.symbol == symbol)
			.unwrap()
			.pulse
			.clone()
	}
}

impl Default for SymbolConverter {
	fn default() -> Self {
		let symbol_map = SYMBOL_SPEC
			.into_iter()
			.map(|(char, pulse_str, group, symbol)| SymbolSpec {
				symbol,
				group,
				char,
				pulse: PulseSymbol::from_dot_str(pulse_str),
			})
			.collect();

		Self { symbol_map }
	}
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PulseSymbol {
	Pulses(Vec<bool>),
	Break,
}

impl PulseSymbol {
	pub fn from_dot_str(pulse_str: &str) -> Self {
		if pulse_str.is_empty() {
			return Self::Break;
		}

		let mut pulses = vec![];

		for pulse_char in pulse_str.chars() {
			let pulse = match pulse_char {
				'.' => false,
				'-' => true,
				_ => panic!("char may only be '.' or '-'"),
			};

			pulses.push(pulse);
		}

		Self::Pulses(pulses)
	}
}

impl fmt::Display for PulseSymbol {
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

enum Group {
	Letter,
	Number,
	Special,
	Prosign,
	Void,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum Symbol {
	Space,
	A,
	B,
	C,
	D,
	E,
	F,
	G,
	H,
	I,
	J,
	K,
	L,
	M,
	N,
	O,
	P,
	Q,
	R,
	S,
	T,
	U,
	V,
	W,
	X,
	Y,
	Z,
	_0,
	_1,
	_2,
	_3,
	_4,
	_5,
	_6,
	_7,
	_8,
	_9,
	Period,
	Comma,
	Question,
	Exclamation,
	Slash,
	ParenthesisOpen,
	ParenthesisClose,
	Ampersand,
	Colon,
	Semicolon,
	Equals,
	Plus,
	Minus,
	Dollar,
	At,
	Tilde,
	Correction,
	Wait,
	Start,
	End,
	EndOfContact,
	NewLine,
	SOS,
}

#[rustfmt::skip]
const SYMBOL_SPEC: [(char, &str, Group, Symbol); 60] = [
	(' ',	"",			Group::Void,	Symbol::Space),
	('A',	".-",		Group::Letter,	Symbol::A),
	('B',	"-...",		Group::Letter,	Symbol::B),
	('C',	"-.-.",		Group::Letter,	Symbol::C),
	('D',	"-..",		Group::Letter,	Symbol::D),
	('E',	".",		Group::Letter,	Symbol::E),
	('F',	"..-.",		Group::Letter,	Symbol::F),
	('G',	"--.",		Group::Letter,	Symbol::G),
	('H',	"....",		Group::Letter,	Symbol::H),
	('I',	"..",		Group::Letter,	Symbol::I),
	('J',	".---",		Group::Letter,	Symbol::J),
	('K',	"-.-",		Group::Letter,	Symbol::K),
	('L',	".-..",		Group::Letter,	Symbol::L),
	('M',	"--",		Group::Letter,	Symbol::M),
	('N',	"-.",		Group::Letter,	Symbol::N),
	('O',	"---",		Group::Letter,	Symbol::O),
	('P',	".--.",		Group::Letter,	Symbol::P),
	('Q',	"--.-",		Group::Letter,	Symbol::Q),
	('R',	".-.",		Group::Letter,	Symbol::R),
	('S',	"...",		Group::Letter,	Symbol::S),
	('T',	"-",		Group::Letter,	Symbol::T),
	('U',	"..-",		Group::Letter,	Symbol::U),
	('V',	"...-",		Group::Letter,	Symbol::V),
	('W',	".--",		Group::Letter,	Symbol::W),
	('X',	"-..-",		Group::Letter,	Symbol::X),
	('Y',	"-.--",		Group::Letter,	Symbol::Y),
	('Z',	"--..",		Group::Letter,	Symbol::Z),
	('0',	"-----",	Group::Number,	Symbol::_0),
	('1',	".----",	Group::Number,	Symbol::_1),
	('2',	"..---",	Group::Number,	Symbol::_2),
	('3',	"...--",	Group::Number,	Symbol::_3),
	('4',	"....-",	Group::Number,	Symbol::_4),
	('5',	".....",	Group::Number,	Symbol::_5),
	('6',	"-....",	Group::Number,	Symbol::_6),
	('7',	"--...",	Group::Number,	Symbol::_7),
	('8',	"---..",	Group::Number,	Symbol::_8),
	('9',	"----.",	Group::Number,	Symbol::_9),
	('.',	".-.-.-",	Group::Special,	Symbol::Period),
	(',',	"--..--",	Group::Special,	Symbol::Comma),
	('?',	"..--..",	Group::Special,	Symbol::Question),
	('!',	"-.-.--",	Group::Special,	Symbol::Exclamation),
	('/',	"-..-.",	Group::Special,	Symbol::Slash),
	('(',	"-.--.",	Group::Special,	Symbol::ParenthesisOpen),
	(')',	"-.--.-",	Group::Special,	Symbol::ParenthesisClose),
	('&',	".-...",	Group::Special,	Symbol::Ampersand),
	(':',	"---...",	Group::Special,	Symbol::Colon),
	(';',	"-.-.-.",	Group::Special,	Symbol::Semicolon),
	('=',	"-...-",	Group::Special,	Symbol::Equals),
	('+',	".-.-.",	Group::Special,	Symbol::Plus),
	('-',	"-....-",	Group::Special,	Symbol::Minus),
	('$',	"...-..-",	Group::Special,	Symbol::Dollar),
	('@',	".--.-.",	Group::Special,	Symbol::At),
	('~',	".-.-.-.",	Group::Special,	Symbol::Tilde),
	('*',	"........",	Group::Prosign,	Symbol::Correction),	// [HH]
	('^',	".-...",	Group::Prosign,	Symbol::Wait),			// [AS]
	('{',	"-.-.-",	Group::Prosign,	Symbol::Start),			// [CT] commencing transmission
	('}',	".-.-.",	Group::Prosign,	Symbol::End),			// [AR] end of message
	('%',	"...-.-",	Group::Prosign,	Symbol::EndOfContact),	// [VA] end of contact
	('\n',	".-.-",		Group::Prosign,	Symbol::NewLine),		// [RT] 
	('#',	"...---...",Group::Prosign,	Symbol::SOS),			// [SOS]
];

#[cfg(test)]
mod tests {}
