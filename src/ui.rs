use crate::prelude::*;
use eframe::egui::{self, Color32, IconData, Key, Ui};
use image::load_from_memory;
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};
use std::{sync::Arc, time::SystemTime};

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
struct UiContext;

impl CwContext for UiContext {
	fn config(&self) -> CwConfig {
		CwConfig::default()
	}

	fn time(&self) -> u32 {
		SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.map(|d| d.as_millis() as u32)
			.unwrap()
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
	audio: AudioState,
	controller: SignalController<AppLauncher>,
}

impl eframe::App for App {
	fn update(&mut self, egui_ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(egui_ctx, |ui| {
			egui_ctx.request_repaint();

			let mouse_input = egui_ctx.input(|i| i.pointer.primary_down());
			let kb_input = egui_ctx.input(|i| i.key_down(Key::Space));
			let input = mouse_input || kb_input;

			let mut cw_ctx = UiContext;

			let signal = self.controller.tick(&mut cw_ctx, input);

			let color = match signal {
				Some(_) => match self.controller.get_mode() {
					Mode::Output => OUTPUT_COLOR,
					Mode::Input => INPUT_COLOR,
				},
				None => OFF_COLOR,
			};

			if self.audio.last_signal != signal {
				self.audio.sink = get_audio_sink(&self.audio.stream_handle, signal);
			}

			self.audio.last_signal = signal;

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
