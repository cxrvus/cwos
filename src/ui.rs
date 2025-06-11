use crate::prelude::*;
use eframe::egui::{self, Color32, IconData, Key, Ui};
use image::load_from_memory;
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::Arc;

pub fn load_icon() -> Option<Arc<IconData>> {
	let bytes = include_bytes!("../assets/icon.png");
	let image = load_from_memory(bytes).ok()?.into_rgba8();
	let (width, height) = image.dimensions();
	let rgba = image.into_raw();
	let icon = Some(IconData {
		rgba,
		width,
		height,
	});

	icon.map(Arc::new)
}

pub fn create_app() -> App {
	App::default()
}

#[derive(Default)]
struct UiContext {
	egui_ctx: egui::Context,
	audio: AudioState,
	config: Config,
	output_color: Color32,
}

impl CwContext<bool, Option<u32>> for UiContext {
	fn input(&self) -> bool {
		let mouse_input = self.egui_ctx.input(|i| i.pointer.primary_down());
		let kb_input = self.egui_ctx.input(|i| i.key_down(Key::Space));
		mouse_input || kb_input
	}

	fn set_output(&mut self, signal: Option<u32>) {
		let color = match signal {
			// TODO
			// Some(_) => match self.controller.get_mode() {
			// 	Mode::Output => OUTPUT_COLOR,
			// 	Mode::Input => INPUT_COLOR,
			// },
			Some(_) => INPUT_COLOR,
			None => OFF_COLOR,
		};

		if self.audio.last_signal != signal {
			self.audio.sink = get_audio_sink(&self.audio.stream_handle, signal);
		}

		self.audio.last_signal = signal;
		self.output_color = color;
	}

	fn config(&self) -> &Config {
		&self.config
	}

	fn time(&self) -> u32 {
		let time = self.egui_ctx.input(|i| i.time);
		(time * 1000.0) as u32
	}
}

struct AudioState {
	_stream: OutputStream, // must be kept alive
	stream_handle: OutputStreamHandle,
	sink: Option<Sink>,
	last_signal: Option<u32>,
}

impl Default for AudioState {
	fn default() -> Self {
		let (stream, stream_handle) = OutputStream::try_default().expect("Audio init failed");

		Self {
			_stream: stream,
			stream_handle,
			sink: None,
			last_signal: None,
		}
	}
}

const OFF_COLOR: Color32 = Color32::from_gray(64);
const INPUT_COLOR: Color32 = Color32::from_gray(192);
const OUTPUT_COLOR: Color32 = Color32::from_gray(128);

#[derive(Default)]
pub struct App {
	cw_ctx: UiContext,
	controller: LinearController<AppLauncher>,
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			ctx.request_repaint();

			self.controller.tick(&mut self.cw_ctx);

			draw_circle(ui, self.cw_ctx.output_color);

			// let mut callback = |input: CwString| {
			// 	dbg!(&input.as_string());
			// 	let output = self.cw_controller.tick(input);
			// 	dbg!(&output.as_string());
			// 	output
			// };

			// let signal_on = self.controller.tick(delta_ms, input_state, &mut callback);
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
