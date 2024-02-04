use crate::app::{InputTarget, Renderer};
use ratatui::widgets::Paragraph;

#[derive(Debug)]
pub struct Chargen<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Chargen<'_> {
	fn handle_input(&self, _app: &String, _c: char) {
	//	app = &c.to_string();
	}
}

impl Renderer for Chargen<'_> {
	fn render_ui(&self, _app: &crate::app::App, f: &mut ratatui::Frame) {
		f.render_widget(Paragraph::new("Chargen rendering!!"), f.size());
	}
}

impl Default for Chargen<'_> {
	fn default() -> Self {
		Self {
			title: "Birthing...",
			input: String::new(),
		}
	}
}

#[derive(Debug)]
pub struct Gameplay<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Gameplay<'_> {
	fn handle_input(&self, _app: &String, _c: char) {
	//	app = &c.to_string();
	}
}

impl Renderer for Gameplay<'_> {
	fn render_ui(&self, _app: &crate::app::App, f: &mut ratatui::Frame) {
		f.render_widget(Paragraph::new("Gameplay rendering!!"), f.size());
	}
}

impl Default for Gameplay<'_> {
	fn default() -> Self {
		Self {
			title: "Adventuring...",
			input: String::new(),
		}
	}
}
