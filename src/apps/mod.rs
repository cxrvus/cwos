use crate::prelude::*;

#[derive(Default)]
pub struct AppState {
	selected_app: Option<String>,
}

pub fn app_launcher(state: &mut AppState, input: SymbolString) -> Vec<Action<AppState>> {
	let input = input.normalized(); // TODO: move normalization into Services

	let action = match state.selected_app {
		Some(ref app_name) => match app_name.as_str() {
			"EC" => Action::Call(Service(echo)),
			_ => idk(),
		},
		None => match input.as_string().as_str() {
			app_name @ "EC" => {
				state.selected_app = Some(app_name.into());
				Action::Respond(Response::new(app_name.to_string().try_into().unwrap()))
			}
			_ => idk(),
		},
	};

	let repeat = Action::Call(Service(app_launcher));
	vec![action, repeat]
}

fn idk<T>() -> Action<T> {
	Action::Respond(Response::new(
		SymbolString::try_from("?".to_string()).unwrap(),
	))
}
