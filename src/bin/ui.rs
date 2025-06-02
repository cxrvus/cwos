use eframe::egui::{self, Color32};

const OFF_COLOR: Color32 = Color32::from_gray(64);
const ON_COLOR: Color32 = Color32::from_gray(128);

fn main() -> eframe::Result {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
		..Default::default()
	};

	// Our application state:
	eframe::run_simple_native("CWOS", options, move |ctx, _frame| {
		egui::CentralPanel::default().show(ctx, |ui| {
			ctx.request_repaint();

			let time = ctx.input(|i| i.time);

			let signal_is_on = (time as u64) % 2 == 0;

			let color = if signal_is_on { ON_COLOR } else { OFF_COLOR };

			let center = ui.min_rect().center();
			ui.painter().circle_filled(center, 100.0, color);
		});
	})
}
