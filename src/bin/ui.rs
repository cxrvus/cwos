use cwos::core::{config::Config, signal::Mode};
use eframe::egui::{self, Color32};

const OFF_COLOR: Color32 = Color32::from_gray(64);
const ON_COLOR: Color32 = Color32::from_gray(128);

fn main() -> eframe::Result {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
		centered: true,
		..Default::default()
	};

	let config = Config::default();
	let mut controller = UiController::new(config.clone());

	// Our application state:
	eframe::run_simple_native("CWOS", options, move |ctx, _frame| {
		egui::CentralPanel::default().show(ctx, |ui| {
			ctx.request_repaint();

			let time = ctx.input(|i| i.time);
			let time_ms = (time * 1000.0) as u32;

			let signal_is_on = controller.tick(false, time_ms).is_some();

			let color = if signal_is_on { ON_COLOR } else { OFF_COLOR };
			let center = ui.min_rect().center();
			ui.painter().circle_filled(center, 100.0, color);
		});
	})
}

#[derive(Default)]
struct UiController {
	config: Config,
	mode: Mode,
	time_ms: u32,
}

impl UiController {
	fn new(config: Config) -> Self {
		Self {
			config,
			..Default::default()
		}
	}

	fn tick(&mut self, _input_state: bool, time_ms: u32) -> Option<Mode> {
		let signal_is_on = time_ms % 2000 < 1000;

		if signal_is_on {
			Some(Mode::default())
		} else {
			None
		}
	}
}
