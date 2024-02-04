use crate::app::{InputTarget, Renderer};
use ratatui::{prelude::*, widgets::Paragraph};

pub struct Chargen<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Chargen<'_> {
	fn handle_input(&self, mut app: &String, c: char) {
	//	app = &c.to_string();
	}
}

impl Renderer for Chargen<'_> {
	fn render_ui(&self, app: &crate::app::App, f: &mut ratatui::Frame) {
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
// pub struct Gameplay<'a> {
// 	pub title: &'a str,
// 	pub input: String,
// }

// impl InputTarget for Gameplay<'_> {
// 	fn handle_input(&self, mut app: &String, c: char) {
// 		app = &c.to_string();
// 	}
// }

// impl Default for Gameplay<'_> {
// 	fn default() -> Self {
// 		Self {
// 			title: "Playing...",
// 			input: String::new(),
// 		}
// 	}
// }
