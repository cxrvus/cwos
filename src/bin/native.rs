use cwos::ui::{create_app, load_icon};
use eframe::{egui::ViewportBuilder, NativeOptions};

fn main() -> eframe::Result<()> {
	let icon = load_icon();
	let mut viewport = ViewportBuilder::default().with_inner_size([320.0, 240.0]);

	if let Some(icon) = icon {
		viewport = viewport.with_icon(icon)
	}

	let options = NativeOptions {
		viewport,
		centered: true,
		..Default::default()
	};

	eframe::run_native("CWOS", options, Box::new(|_cc| Ok(Box::new(create_app()))))
}
