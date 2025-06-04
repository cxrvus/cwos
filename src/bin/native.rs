use cwos::ui::{create_app, load_icon};
use eframe::egui;

fn main() -> eframe::Result<()> {
	let icon = load_icon();
	let mut viewport = egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]);

	if let Some(icon) = icon {
		viewport = viewport.with_icon(icon)
	}

	let options = eframe::NativeOptions {
		viewport,
		centered: true,
		..Default::default()
	};

	eframe::run_native(
		"CWOS",
		options,
		Box::new(|cc| {
			cc.egui_ctx.set_theme(egui::Theme::Dark);
			Ok(Box::new(create_app()))
		}),
	)
}
