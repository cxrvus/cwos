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
