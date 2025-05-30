use anyhow::{anyhow, Result};
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct ElementString(pub Vec<bool>);

impl ElementString {
	pub fn to_dot_string(&self) -> String {
		if self.0.is_empty() {
			" / ".to_string()
		} else {
			self.0
				.iter()
				.map(|&element| if element { "-" } else { "." })
				.collect::<String>()
		}
	}
}

impl From<String> for ElementString {
	fn from(element_str: String) -> Self {
		if element_str.is_empty() {
			return Self::default();
		}

		let mut elements = vec![];

		for signal_char in element_str.chars() {
			let element = match signal_char {
				'.' => false,
				'-' => true,
				_ => panic!("char may only be '.' or '-'"),
			};

			elements.push(element);
		}

		Self(elements)
	}
}

#[derive(Clone)]
pub enum Group {
	Letter,
	Number,
	Special,
	Prosign,
	Void,
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub enum Symbol {
	#[default]
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
	Invalid,
	Correction,
	Wait,
	Start,
	End,
	EndOfContact,
	NewLine,
	SOS,
}

impl Symbol {
	pub fn character(&self) -> char {
		self.get_spec().character()
	}

	pub fn elements(&self) -> ElementString {
		self.get_spec().elements()
	}

	pub fn group(&self) -> Group {
		self.get_spec().group()
	}

	fn get_spec(&self) -> &SymbolSpec {
		SYMBOL_SPEC
			.iter()
			.find(|spec| spec.symbol() == *self)
			.unwrap()
	}

	pub fn from_elements(elements: &ElementString) -> Self {
		SYMBOL_SPEC
			.iter()
			.find(|spec| spec.elements() == *elements)
			.map(|spec| spec.symbol().clone())
			.unwrap_or(Self::Invalid)
	}

	pub fn from_char(char: char) -> Result<Self> {
		SYMBOL_SPEC
			.iter()
			.find(|spec| spec.character() == char.to_ascii_uppercase())
			.map(|spec| spec.symbol().clone())
			.ok_or_else(|| anyhow!("invalid plaintext char: {:?}", char))
	}
}

pub struct SymbolString(pub Vec<Symbol>);

impl SymbolString {
	pub fn as_string(&self) -> String {
		self.0.iter().map(|symbol| symbol.character()).collect()
	}
}

impl TryFrom<String> for SymbolString {
	fn try_from(string: String) -> Result<Self> {
		let symbols = string
			.chars()
			.map(Symbol::from_char)
			.collect::<Result<Vec<Symbol>>>()?;

		Ok(Self(symbols))
	}

	type Error = anyhow::Error;
}

struct SymbolSpec(char, ElementString, Group, Symbol);

#[rustfmt::skip]
impl SymbolSpec {
	pub fn character(&self) -> char { self.0 }
	pub fn elements(&self) -> ElementString { self.1.clone() }
	pub fn group(&self) -> Group { self.2.clone() }
	pub fn symbol(&self) -> Symbol { self.3.clone() }
}

/// Placeholder
#[macro_export]
macro_rules! cw {
	($($tt:tt)*) => {
		ElementString(vec![])
	};
	() => {
		ElementString(vec![])
	};
}

#[rustfmt::skip]
static SYMBOL_SPEC: [SymbolSpec; 60] = [
	SymbolSpec(' ',		cw!(),				Group::Void,		Symbol::Space),
	SymbolSpec('A',		cw!(.-),			Group::Letter,		Symbol::A),
	SymbolSpec('B',		cw!(-...),			Group::Letter,		Symbol::B),
	SymbolSpec('C',		cw!(-.-.),			Group::Letter,		Symbol::C),
	SymbolSpec('D',		cw!(-..),			Group::Letter,		Symbol::D),
	SymbolSpec('E',		cw!(.),				Group::Letter,		Symbol::E),
	SymbolSpec('F',		cw!(..-.),			Group::Letter,		Symbol::F),
	SymbolSpec('G',		cw!(--.),			Group::Letter,		Symbol::G),
	SymbolSpec('H',		cw!(....),			Group::Letter,		Symbol::H),
	SymbolSpec('I',		cw!(..),			Group::Letter,		Symbol::I),
	SymbolSpec('J',		cw!(.---),			Group::Letter,		Symbol::J),
	SymbolSpec('K',		cw!(-.-),			Group::Letter,		Symbol::K),
	SymbolSpec('L',		cw!(.-..),			Group::Letter,		Symbol::L),
	SymbolSpec('M',		cw!(--),			Group::Letter,		Symbol::M),
	SymbolSpec('N',		cw!(-.),			Group::Letter,		Symbol::N),
	SymbolSpec('O',		cw!(---),			Group::Letter,		Symbol::O),
	SymbolSpec('P',		cw!(.--.),			Group::Letter,		Symbol::P),
	SymbolSpec('Q',		cw!(--.-),			Group::Letter,		Symbol::Q),
	SymbolSpec('R',		cw!(.-.),			Group::Letter,		Symbol::R),
	SymbolSpec('S',		cw!(...),			Group::Letter,		Symbol::S),
	SymbolSpec('T',		cw!(-),				Group::Letter,		Symbol::T),
	SymbolSpec('U',		cw!(..-),			Group::Letter,		Symbol::U),
	SymbolSpec('V',		cw!(...-),			Group::Letter,		Symbol::V),
	SymbolSpec('W',		cw!(.--),			Group::Letter,		Symbol::W),
	SymbolSpec('X',		cw!(-..-),			Group::Letter,		Symbol::X),
	SymbolSpec('Y',		cw!(-.--),			Group::Letter,		Symbol::Y),
	SymbolSpec('Z',		cw!(--..),			Group::Letter,		Symbol::Z),
	SymbolSpec('0',		cw!(-----),			Group::Number,		Symbol::_0),
	SymbolSpec('1',		cw!(.----),			Group::Number,		Symbol::_1),
	SymbolSpec('2',		cw!(..---),			Group::Number,		Symbol::_2),
	SymbolSpec('3',		cw!(...--),			Group::Number,		Symbol::_3),
	SymbolSpec('4',		cw!(....-),			Group::Number,		Symbol::_4),
	SymbolSpec('5',		cw!(.....),			Group::Number,		Symbol::_5),
	SymbolSpec('6',		cw!(-....),			Group::Number,		Symbol::_6),
	SymbolSpec('7',		cw!(--...),			Group::Number,		Symbol::_7),
	SymbolSpec('8',		cw!(---..),			Group::Number,		Symbol::_8),
	SymbolSpec('9',		cw!(----.),			Group::Number,		Symbol::_9),
	SymbolSpec('.',		cw!(.-.-.-),		Group::Special,		Symbol::Period),
	SymbolSpec(',',		cw!(--..--),		Group::Special,		Symbol::Comma),
	SymbolSpec('?',		cw!(..--..),		Group::Special,		Symbol::Question),
	SymbolSpec('!',		cw!(-.-.--),		Group::Special,		Symbol::Exclamation),
	SymbolSpec('/',		cw!(-..-.),			Group::Special,		Symbol::Slash),
	SymbolSpec('(',		cw!(-.--.),			Group::Special,		Symbol::ParenthesisOpen),
	SymbolSpec(')',		cw!(-.--.-),		Group::Special,		Symbol::ParenthesisClose),
	SymbolSpec('&',		cw!(.-...),			Group::Special,		Symbol::Ampersand),
	SymbolSpec(':',		cw!(---...),		Group::Special,		Symbol::Colon),
	SymbolSpec(';',		cw!(-.-.-.),		Group::Special,		Symbol::Semicolon),
	SymbolSpec('=',		cw!(-...-),			Group::Special,		Symbol::Equals),
	SymbolSpec('+',		cw!(.-.-.),			Group::Special,		Symbol::Plus),
	SymbolSpec('-',		cw!(-....-),		Group::Special,		Symbol::Minus),
	SymbolSpec('$',		cw!(...-..-),		Group::Special,		Symbol::Dollar),
	SymbolSpec('@',		cw!(.--.-.),		Group::Special,		Symbol::At),
	SymbolSpec('~',		cw!(.-.-.-.),		Group::Special,		Symbol::Invalid),		// for undefined rhythms
	SymbolSpec('*',		cw!(........),		Group::Prosign,		Symbol::Correction),	// [HH]
	SymbolSpec('^',		cw!(.-...),			Group::Prosign,		Symbol::Wait),			// [AS]
	SymbolSpec('{',		cw!(-.-.-),			Group::Prosign,		Symbol::Start),			// [CT] commencing transmission
	SymbolSpec('}',		cw!(.-.-.),			Group::Prosign,		Symbol::End),			// [AR] end of message
	SymbolSpec('%',		cw!(...-.-),		Group::Prosign,		Symbol::EndOfContact),	// [VA] end of contact
	SymbolSpec('\n',	cw!(.-.-),			Group::Prosign,		Symbol::NewLine),		// [RT] 
	SymbolSpec('#',		cw!(...---...),		Group::Prosign,		Symbol::SOS),			// [SOS]
];
