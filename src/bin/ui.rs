use cwos::{apps::context::AppContext, prelude::*};
use eframe::{
	egui::{self, Color32, IconData, Key, Ui, ViewportBuilder},
	NativeOptions,
};
use image::load_from_memory;
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::Arc;

fn load_icon() -> Option<IconData> {
	let bytes = include_bytes!("assets/icon.png");
	let image = load_from_memory(bytes).ok()?.into_rgba8();
	let (width, height) = image.dimensions();
	let rgba = image.into_raw();
	Some(IconData {
		rgba,
		width,
		height,
	})
}

fn main() -> eframe::Result<()> {
	let icon = load_icon().map(Arc::new);
	let mut viewport = ViewportBuilder::default().with_inner_size([320.0, 240.0]);

	if let Some(icon) = icon {
		viewport = viewport.with_icon(icon)
	}

	let options = NativeOptions {
		viewport,
		centered: true,
		..Default::default()
	};

	let app = UiContext::new(Config::default());

	eframe::run_native("CWOS", options, Box::new(|_cc| Ok(Box::new(app))))
}

struct AudioContext {
	last_beep: Beep,
	_stream: OutputStream, // must be kept alive
	stream_handle: OutputStreamHandle,
	sink: Option<Sink>,
}

struct UiContext {
	audio: AudioContext,
	config: Config,
	signal_controller: SignalController,
	cw_controller: Echo,
	cw_ctx: AppContext,
	time_ms: u32,
}

impl UiContext {
	fn new(config: Config) -> Self {
		let (stream, stream_handle) = OutputStream::try_default().expect("Audio init failed");

		let audio = AudioContext {
			_stream: stream,
			stream_handle,
			sink: None,
			last_beep: Beep::Off,
		};

		Self {
			signal_controller: SignalController::new(&config.clone()),
			config,
			audio,
			cw_controller: Echo,
			cw_ctx: AppContext::default(),
			time_ms: 0,
		}
	}
}

const OFF_COLOR: Color32 = Color32::from_gray(64);
const INPUT_COLOR: Color32 = Color32::from_gray(192);
const OUTPUT_COLOR: Color32 = Color32::from_gray(128);

impl eframe::App for UiContext {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ctx.request_repaint();

			let time = ctx.input(|i| i.time);
			let time_ms = (time * 1000.0) as u32;

			let delta_ms = time_ms - self.time_ms;
			self.time_ms = time_ms;

			let mouse_input = ctx.input(|i| i.pointer.primary_down());
			let kb_input = ctx.input(|i| i.key_down(Key::Space));
			let input_state = mouse_input || kb_input;

			let mut callback = |input: SymbolString| {
				dbg!(&input.as_string());
				let output = self.cw_controller.tick(&mut self.cw_ctx, input);
				dbg!(&output.as_string());
				output
			};

			let signal_on = self
				.signal_controller
				.tick(delta_ms, input_state, &mut callback);

			let beep = match signal_on {
				true => Beep::On(self.signal_controller.get_mode()),
				false => Beep::Off,
			};

			let (color, freq) = match beep.clone() {
				Beep::On(mode) => match mode {
					Mode::Output => (OUTPUT_COLOR, Some(self.config.output.signal.freq)),
					Mode::Input => (INPUT_COLOR, Some(self.config.input.signal.freq)),
				},
				Beep::Off => (OFF_COLOR, None),
			};

			if self.audio.last_beep != beep {
				self.audio.sink = get_audio_sink(&self.audio.stream_handle, freq);
			}

			self.audio.last_beep = beep;

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
