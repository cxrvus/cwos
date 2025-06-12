use crate::prelude::CwElementString;

impl CwSymbol {
	pub fn character(&self) -> char {
		self.spec().character()
	}

	pub fn elements(&self) -> CwElementString {
		self.spec().elements()
	}

	pub fn group(&self) -> Group {
		self.spec().group()
	}

	fn spec(&self) -> &SymbolSpec {
		SYMBOL_SPEC
			.iter()
			.find(|spec| spec.symbol() == *self)
			.unwrap()
	}
}

impl From<char> for CwSymbol {
	fn from(c: char) -> Self {
		SYMBOL_SPEC
			.iter()
			.find(|spec| spec.character() == c.to_ascii_uppercase())
			.map(|spec| spec.symbol().clone())
			.unwrap_or(CwSymbol::Invalid)
	}
}

impl From<&CwElementString> for CwSymbol {
	fn from(elements: &CwElementString) -> Self {
		SYMBOL_SPEC
			.iter()
			.find(|spec| spec.elements() == *elements)
			.map(|spec| spec.symbol().clone())
			.unwrap_or(Self::Invalid)
	}
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CwString(pub Vec<CwSymbol>);

impl CwString {
	pub fn normalized(&self) -> Self {
		let str = String::from(self);
		let str = str.trim();
		// todo: handle [HH]
		Self::from(str)
	}
}

impl From<&str> for CwString {
	fn from(s: &str) -> Self {
		Self(
			s.to_ascii_uppercase()
				.chars()
				.map(CwSymbol::from)
				.collect::<Vec<CwSymbol>>(),
		)
	}
}

impl From<&CwString> for String {
	fn from(cw_string: &CwString) -> Self {
		cw_string
			.0
			.iter()
			.map(|symbol| symbol.character())
			.collect()
	}
}

struct SymbolSpec(char, &'static str, Group, CwSymbol);

#[rustfmt::skip]
impl SymbolSpec {
	pub fn character(&self) -> char { self.0 }
	pub fn elements(&self) -> CwElementString { CwElementString::new(self.1.to_string()) }
	pub fn group(&self) -> Group { self.2.clone() }
	pub fn symbol(&self) -> CwSymbol { self.3.clone() }
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
pub enum CwSymbol {
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

#[rustfmt::skip]
static SYMBOL_SPEC: [SymbolSpec; 58] = [
	SymbolSpec(' ',		"",			Group::Void,		CwSymbol::Space),
	SymbolSpec('A',		".-",		Group::Letter,		CwSymbol::A),
	SymbolSpec('B',		"-...",		Group::Letter,		CwSymbol::B),
	SymbolSpec('C',		"-.-.",		Group::Letter,		CwSymbol::C),
	SymbolSpec('D',		"-..",		Group::Letter,		CwSymbol::D),
	SymbolSpec('E',		".",		Group::Letter,		CwSymbol::E),
	SymbolSpec('F',		"..-.",		Group::Letter,		CwSymbol::F),
	SymbolSpec('G',		"--.",		Group::Letter,		CwSymbol::G),
	SymbolSpec('H',		"....",		Group::Letter,		CwSymbol::H),
	SymbolSpec('I',		"..",		Group::Letter,		CwSymbol::I),
	SymbolSpec('J',		".---",		Group::Letter,		CwSymbol::J),
	SymbolSpec('K',		"-.-",		Group::Letter,		CwSymbol::K),
	SymbolSpec('L',		".-..",		Group::Letter,		CwSymbol::L),
	SymbolSpec('M',		"--",		Group::Letter,		CwSymbol::M),
	SymbolSpec('N',		"-.",		Group::Letter,		CwSymbol::N),
	SymbolSpec('O',		"---",		Group::Letter,		CwSymbol::O),
	SymbolSpec('P',		".--.",		Group::Letter,		CwSymbol::P),
	SymbolSpec('Q',		"--.-",		Group::Letter,		CwSymbol::Q),
	SymbolSpec('R',		".-.",		Group::Letter,		CwSymbol::R),
	SymbolSpec('S',		"...",		Group::Letter,		CwSymbol::S),
	SymbolSpec('T',		"-",		Group::Letter,		CwSymbol::T),
	SymbolSpec('U',		"..-",		Group::Letter,		CwSymbol::U),
	SymbolSpec('V',		"...-",		Group::Letter,		CwSymbol::V),
	SymbolSpec('W',		".--",		Group::Letter,		CwSymbol::W),
	SymbolSpec('X',		"-..-",		Group::Letter,		CwSymbol::X),
	SymbolSpec('Y',		"-.--",		Group::Letter,		CwSymbol::Y),
	SymbolSpec('Z',		"--..",		Group::Letter,		CwSymbol::Z),
	SymbolSpec('0',		"-----",	Group::Number,		CwSymbol::_0),
	SymbolSpec('1',		".----",	Group::Number,		CwSymbol::_1),
	SymbolSpec('2',		"..---",	Group::Number,		CwSymbol::_2),
	SymbolSpec('3',		"...--",	Group::Number,		CwSymbol::_3),
	SymbolSpec('4',		"....-",	Group::Number,		CwSymbol::_4),
	SymbolSpec('5',		".....",	Group::Number,		CwSymbol::_5),
	SymbolSpec('6',		"-....",	Group::Number,		CwSymbol::_6),
	SymbolSpec('7',		"--...",	Group::Number,		CwSymbol::_7),
	SymbolSpec('8',		"---..",	Group::Number,		CwSymbol::_8),
	SymbolSpec('9',		"----.",	Group::Number,		CwSymbol::_9),
	SymbolSpec('.',		".-.-.-",	Group::Special,		CwSymbol::Period),
	SymbolSpec(',',		"--..--",	Group::Special,		CwSymbol::Comma),
	SymbolSpec('?',		"..--..",	Group::Special,		CwSymbol::Question),
	SymbolSpec('!',		"-.-.--",	Group::Special,		CwSymbol::Exclamation),
	SymbolSpec('/',		"-..-.",	Group::Special,		CwSymbol::Slash),
	SymbolSpec('(',		"-.--.",	Group::Special,		CwSymbol::ParenthesisOpen),	// [KN] go ahead
	SymbolSpec(')',		"-.--.-",	Group::Special,		CwSymbol::ParenthesisClose),
	SymbolSpec('&',		".-...",	Group::Special,		CwSymbol::Ampersand),		// [AS] wait
	SymbolSpec(':',		"---...",	Group::Special,		CwSymbol::Colon),
	SymbolSpec(';',		"-.-.-.",	Group::Special,		CwSymbol::Semicolon),
	SymbolSpec('=',		"-...-",	Group::Special,		CwSymbol::Equals),
	SymbolSpec('+',		".-.-.",	Group::Special,		CwSymbol::Plus),
	SymbolSpec('-',		"-....-",	Group::Special,		CwSymbol::Minus),
	SymbolSpec('@',		".--.-.",	Group::Special,		CwSymbol::At),
	SymbolSpec('$',		"...-..-",	Group::Special,		CwSymbol::Dollar),
	SymbolSpec('~',		".-.-.-.",	Group::Prosign,		CwSymbol::Invalid), 		// [~] - for undefined CW sequences
	SymbolSpec('*',		"........",	Group::Prosign,		CwSymbol::Correction), 	// [HH] error / correction
	SymbolSpec('^',		"-.-.-",	Group::Prosign,		CwSymbol::Start), 		// [CT] commencing transmission
	SymbolSpec('#',		"...-.-",	Group::Prosign,		CwSymbol::End), 			// [VA] end of contact
	SymbolSpec('\n',	".-.-",		Group::Prosign,		CwSymbol::NewLine), 		// [RT] carriage return
	SymbolSpec('%',		"...---...",	Group::Prosign,	CwSymbol::SOS), 			// [SOS]
];
