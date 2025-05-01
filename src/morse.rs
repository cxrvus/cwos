use std::collections::HashMap;

pub struct Parser {
	char_to_morse: HashMap<char, String>,
	morse_to_char: HashMap<String, char>,
}

impl Parser {
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
			char_to_morse,
			morse_to_char,
		}
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
