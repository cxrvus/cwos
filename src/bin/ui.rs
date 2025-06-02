use cwos::core::{config::Config, signal::Mode};
use eframe::egui::{self, Color32};
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};

const OFF_COLOR: Color32 = Color32::from_gray(64);
const ON_COLOR: Color32 = Color32::from_gray(128);

fn main() -> eframe::Result<()> {
	let options = eframe::NativeOptions {
		viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
		centered: true,
		..Default::default()
	};

	let app = CwosApp::new(Config::default());

	eframe::run_native("CWOS", options, Box::new(|_cc| Ok(Box::new(app))))
}

struct CwosApp {
	controller: UiController,
	_stream: OutputStream, // must be kept alive
	stream_handle: OutputStreamHandle,
	sink: Option<Sink>,
}

impl CwosApp {
	fn new(config: Config) -> Self {
		let (stream, stream_handle) = OutputStream::try_default().expect("Audio init failed");

		Self {
			controller: UiController::new(config),
			_stream: stream,
			stream_handle,
			sink: None,
		}
	}
}

impl eframe::App for CwosApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ctx.request_repaint();

			let time = ctx.input(|i| i.time);
			let time_ms = (time * 1000.0) as u32;

			let signal_is_on = self.controller.tick(false, time_ms).is_some();

			// render circle
			let color = if signal_is_on { ON_COLOR } else { OFF_COLOR };
			let center = ui.min_rect().center();
			ui.painter().circle_filled(center, 100.0, color);

			// play audio
			match (signal_is_on, self.sink.is_some()) {
				(true, false) => {
					let new_sink = Sink::try_new(&self.stream_handle).unwrap();
					let source = SineWave::new(440.).amplify(0.1).repeat_infinite();
					new_sink.append(source);
					new_sink.play();
					self.sink = Some(new_sink);
				}
				(false, true) => {
					self.sink = None; // drop sink to stop audio
				}
				_ => {}
			}
		});
	}
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
		let delta_ms = time_ms - self.time_ms;
		self.time_ms = time_ms;

		let signal_is_on = time_ms % 2000 < 1000;
		if signal_is_on {
			Some(Mode::default())
		} else {
			None
		}
	}
}
