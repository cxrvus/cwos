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

#[derive(Default)]
pub struct SymbolString(pub Vec<Symbol>);

impl SymbolString {
	pub fn as_string(&self) -> String {
		self.0.iter().map(|symbol| symbol.character()).collect()
	}

	pub fn normalized(&self) -> Self {
		let str = self.as_string();
		let str = str.trim();
		// todo: handle [HH]
		Self::try_from(str.to_string()).unwrap()
	}
}

impl TryFrom<String> for SymbolString {
	fn try_from(string: String) -> Result<Self> {
		let symbols = string
			.to_ascii_uppercase()
			.chars()
			.map(Symbol::from_char)
			.collect::<Result<Vec<Symbol>>>()?;

		Ok(Self(symbols))
	}

	type Error = anyhow::Error;
}

struct SymbolSpec(char, &'static str, Group, Symbol);

#[rustfmt::skip]
impl SymbolSpec {
	pub fn character(&self) -> char { self.0 }
	pub fn elements(&self) -> ElementString { ElementString::from(self.1.to_string()) }
	pub fn group(&self) -> Group { self.2.clone() }
	pub fn symbol(&self) -> Symbol { self.3.clone() }
}

#[rustfmt::skip]
static SYMBOL_SPEC: [SymbolSpec; 58] = [
	SymbolSpec(' ',		"",			Group::Void,		Symbol::Space),
	SymbolSpec('A',		".-",		Group::Letter,		Symbol::A),
	SymbolSpec('B',		"-...",		Group::Letter,		Symbol::B),
	SymbolSpec('C',		"-.-.",		Group::Letter,		Symbol::C),
	SymbolSpec('D',		"-..",		Group::Letter,		Symbol::D),
	SymbolSpec('E',		".",		Group::Letter,		Symbol::E),
	SymbolSpec('F',		"..-.",		Group::Letter,		Symbol::F),
	SymbolSpec('G',		"--.",		Group::Letter,		Symbol::G),
	SymbolSpec('H',		"....",		Group::Letter,		Symbol::H),
	SymbolSpec('I',		"..",		Group::Letter,		Symbol::I),
	SymbolSpec('J',		".---",		Group::Letter,		Symbol::J),
	SymbolSpec('K',		"-.-",		Group::Letter,		Symbol::K),
	SymbolSpec('L',		".-..",		Group::Letter,		Symbol::L),
	SymbolSpec('M',		"--",		Group::Letter,		Symbol::M),
	SymbolSpec('N',		"-.",		Group::Letter,		Symbol::N),
	SymbolSpec('O',		"---",		Group::Letter,		Symbol::O),
	SymbolSpec('P',		".--.",		Group::Letter,		Symbol::P),
	SymbolSpec('Q',		"--.-",		Group::Letter,		Symbol::Q),
	SymbolSpec('R',		".-.",		Group::Letter,		Symbol::R),
	SymbolSpec('S',		"...",		Group::Letter,		Symbol::S),
	SymbolSpec('T',		"-",		Group::Letter,		Symbol::T),
	SymbolSpec('U',		"..-",		Group::Letter,		Symbol::U),
	SymbolSpec('V',		"...-",		Group::Letter,		Symbol::V),
	SymbolSpec('W',		".--",		Group::Letter,		Symbol::W),
	SymbolSpec('X',		"-..-",		Group::Letter,		Symbol::X),
	SymbolSpec('Y',		"-.--",		Group::Letter,		Symbol::Y),
	SymbolSpec('Z',		"--..",		Group::Letter,		Symbol::Z),
	SymbolSpec('0',		"-----",	Group::Number,		Symbol::_0),
	SymbolSpec('1',		".----",	Group::Number,		Symbol::_1),
	SymbolSpec('2',		"..---",	Group::Number,		Symbol::_2),
	SymbolSpec('3',		"...--",	Group::Number,		Symbol::_3),
	SymbolSpec('4',		"....-",	Group::Number,		Symbol::_4),
	SymbolSpec('5',		".....",	Group::Number,		Symbol::_5),
	SymbolSpec('6',		"-....",	Group::Number,		Symbol::_6),
	SymbolSpec('7',		"--...",	Group::Number,		Symbol::_7),
	SymbolSpec('8',		"---..",	Group::Number,		Symbol::_8),
	SymbolSpec('9',		"----.",	Group::Number,		Symbol::_9),
	SymbolSpec('.',		".-.-.-",	Group::Special,		Symbol::Period),
	SymbolSpec(',',		"--..--",	Group::Special,		Symbol::Comma),
	SymbolSpec('?',		"..--..",	Group::Special,		Symbol::Question),
	SymbolSpec('!',		"-.-.--",	Group::Special,		Symbol::Exclamation),
	SymbolSpec('/',		"-..-.",	Group::Special,		Symbol::Slash),
	SymbolSpec('(',		"-.--.",	Group::Special,		Symbol::ParenthesisOpen),	// [KN] go ahead
	SymbolSpec(')',		"-.--.-",	Group::Special,		Symbol::ParenthesisClose),
	SymbolSpec('&',		".-...",	Group::Special,		Symbol::Ampersand),		// [AS] wait
	SymbolSpec(':',		"---...",	Group::Special,		Symbol::Colon),
	SymbolSpec(';',		"-.-.-.",	Group::Special,		Symbol::Semicolon),
	SymbolSpec('=',		"-...-",	Group::Special,		Symbol::Equals),
	SymbolSpec('+',		".-.-.",	Group::Special,		Symbol::Plus),
	SymbolSpec('-',		"-....-",	Group::Special,		Symbol::Minus),
	SymbolSpec('@',		".--.-.",	Group::Special,		Symbol::At),
	SymbolSpec('$',		"...-..-",	Group::Special,		Symbol::Dollar),
	SymbolSpec('~',		".-.-.-.",	Group::Prosign,		Symbol::Invalid), 		// [~] - for undefined CW sequences
	SymbolSpec('*',		"........",	Group::Prosign,		Symbol::Correction), 	// [HH] error / correction
	SymbolSpec('^',		"-.-.-",	Group::Prosign,		Symbol::Start), 		// [CT] commencing transmission
	SymbolSpec('#',		"...-.-",	Group::Prosign,		Symbol::End), 			// [VA] end of contact
	SymbolSpec('\n',	".-.-",		Group::Prosign,		Symbol::NewLine), 		// [RT] carriage return
	SymbolSpec('%',		"...---...",	Group::Prosign,	Symbol::SOS), 			// [SOS]
];
