use super::symbol::SymbolString;

pub fn echo<T>(_: &mut T, input: SymbolString) -> Vec<Action<T>> {
	vec![
		Action::Respond(Response::new(input.normalized())),
		Action::Call(Service(echo)),
	]
}

// --------

#[derive(Default, Debug, Clone)]
pub struct Response {
	pub cw: SymbolString,
	pub short: SymbolString,
	pub long: SymbolString,
}

impl Response {
	pub fn new(str: SymbolString) -> Self {
		Self {
			cw: str.clone(),
			short: str.clone(),
			long: str,
		}
	}
}

#[derive(Default)]
pub struct CwController<T: Default> {
	stack: Vec<Action<T>>,
	state: T,
}

pub enum Action<T> {
	Call(Service<T>),
	Respond(Response),
	Repeat,
}

pub struct Service<T>(pub fn(&mut T, SymbolString) -> Vec<Action<T>>);

impl<T> Service<T> {
	#[inline]
	fn tick(&self, state: &mut T, input: SymbolString) -> Vec<Action<T>> {
		(self.0)(state, input)
	}
}

impl<T: Default> CwController<T> {
	pub fn new(root: Service<T>) -> Self {
		Self {
			stack: vec![Action::Call(root)],
			..Default::default()
		}
	}

	pub fn tick(&mut self, input: SymbolString) -> Response {
		while let Some(action) = self.stack.pop() {
			match action {
				Action::Call(service) => {
					let mut actions = service.tick(&mut self.state, input.clone());
					actions.reverse();
					self.stack.extend(actions);
				}
				Action::Respond(response) => {
					return response.clone();
				}
				Action::Repeat => {
					self.stack.push(action);
				}
			};
		}

		Response::new(SymbolString::try_from("end".to_string()).unwrap())
	}
}
