use cwos::core::{
	config::Config,
	context::CwContext,
	controller::{CwController, TestController},
	database::Database,
	signal::{Mode, SignalController},
};
use eframe::egui::{self, Color32, Ui};
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};

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
	config: Config,
	cw: CwInterface,
	last_beep: Beep,
	_stream: OutputStream, // must be kept alive
	stream_handle: OutputStreamHandle,
	sink: Option<Sink>,
}

impl CwosApp {
	fn new(config: Config) -> Self {
		let (stream, stream_handle) = OutputStream::try_default().expect("Audio init failed");

		Self {
			cw: CwInterface::new(config.clone()),
			config,
			_stream: stream,
			stream_handle,
			sink: None,
			last_beep: Beep::Off,
		}
	}
}

const OFF_COLOR: Color32 = Color32::from_gray(64);
const INPUT_COLOR: Color32 = Color32::from_gray(192);
const OUTPUT_COLOR: Color32 = Color32::from_gray(128);

impl eframe::App for CwosApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ctx.request_repaint();

			let time = ctx.input(|i| i.time);
			let time_ms = (time * 1000.0) as u32;

			let beep = self.cw.tick(false, time_ms);

			let (color, freq) = match beep.clone() {
				Beep::On(mode) => match mode {
					Mode::Output => (OUTPUT_COLOR, Some(self.config.output.signal.freq)),
					Mode::Input => (INPUT_COLOR, Some(self.config.input.signal.freq)),
				},
				Beep::Off => (OFF_COLOR, None),
			};

			if self.last_beep != beep {
				self.sink = get_audio_sink(&self.stream_handle, freq);
			}

			draw_circle(ui, color);
		});
	}
}

fn draw_circle(ui: &mut Ui, color: Color32) {
	let center = ui.min_rect().center();
	ui.painter().circle_filled(center, 100.0, color);
}

fn get_audio_sink(stream_handle: &OutputStreamHandle, freq: Option<u32>) -> Option<Sink> {
	if let Some(freq) = freq {
		let new_sink = Sink::try_new(stream_handle).unwrap();
		let source = SineWave::new(freq as f32).amplify(0.1).repeat_infinite();
		new_sink.append(source);
		new_sink.play();
		Some(new_sink)
	} else {
		None
	}
}

#[derive(Clone, PartialEq)]
enum Beep {
	On(Mode),
	Off,
}

#[derive(Default)]
struct CwInterface {
	signal_controller: SignalController,
	cw_controller: TestController,
	cw_ctx: CwContext<Database>,
	time_ms: u32,
}

impl CwInterface {
	fn new(config: Config) -> Self {
		Self {
			signal_controller: SignalController::new(&config),
			..Default::default()
		}
	}

	fn tick(&mut self, input_state: bool, time_ms: u32) -> Beep {
		let delta_ms = time_ms - self.time_ms;
		self.time_ms = time_ms;

		let mode = match (input_state, self.signal_controller.get_mode()) {
			(true, _) => Mode::Input,
			(false, mode) => mode,
		};

		match mode {
			Mode::Output => {
				let is_on = self.signal_controller.output_tick(delta_ms, None);
				if let Some(is_on) = is_on {
					if is_on {
						return Beep::On(Mode::Output);
					}
				}
			}
			Mode::Input => {
				let input = self.signal_controller.input_tick(delta_ms, input_state);
				if let Some(buffer) = input {
					let output = self.cw_controller.tick(&mut self.cw_ctx, buffer);
					self.signal_controller.output_tick(delta_ms, Some(output));
					return Beep::On(Mode::Output);
				} else if input_state {
					return Beep::On(Mode::Input);
				}
			}
		};

		Beep::Off
	}
}
