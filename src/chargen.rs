use crate::{
	app::{App, InputTarget, Renderer, TITLE},
	game::Class,
};
use crossterm::event::KeyCode;
use ratatui::{
	layout::{Alignment, Rect},
	style::Stylize,
	widgets::{block::Position, Block, BorderType, Borders, Paragraph},
	Frame,
};

#[derive(Copy, Clone)]
pub struct Chargen<'a> {
	pub title: &'a str,
	stage: ChargenStage,
	name: &'a str,
}

#[derive(Debug, Copy, Clone)]
enum ChargenStage {
	Class,
	Name,
	Farewell,
}

impl Chargen<'_> {
	pub fn get_render_text(&self, app: &App) -> String {
		match self.stage {
			ChargenStage::Class => {
				let mut s = String::from("Choose your class:\n");

				let mut j = 1;
				for class in Class::ALL {
					s.push_str(&format!("{j} - {class:?}\n").to_owned());
					j += 1;
				}
				s
			}
			ChargenStage::Name => {
				format!("And thy name, {:?}?", app.player.class)
			}
			ChargenStage::Farewell => {
				format!(
					"I fare thee well.\nAnd {}? Beware the Worldwomb.\n\nPress enter to continue.",
					app.player.name
				)
			}
		}
	}
}

impl InputTarget for Chargen<'_> {
	fn handle_input(&mut self, app: &mut App, c: crossterm::event::KeyCode) {
		match self.stage {
			ChargenStage::Class => {
				match c {
					KeyCode::Char('1') => {
						app.player.class = Class::Vagrant;
					}
					KeyCode::Char('2') => {
						app.player.class = Class::Conscript;
					}
					KeyCode::Char('3') => {
						app.player.class = Class::Pilgrim;
					}
					_ => {}
				}
				if app.player.class != Class::Unknown {
					self.stage = ChargenStage::Name;
				}
			}
			ChargenStage::Name => {
				match c {
					KeyCode::Enter => {
						app.player.name = "Cochran";
						self.stage = ChargenStage::Farewell;
					}
					KeyCode::Char(_c) => {
						//	let mut new = self.name.to_string();
						//	new.push(c);
						//	self.name = &new;
					}
					_ => {}
				}
			}
			ChargenStage::Farewell => {
				if let KeyCode::Enter = c {
					app.pop_screen();
				}
			}
		}
	}
}

impl Renderer for Chargen<'_> {
	fn render_ui(&self, app: &crate::app::App, f: &mut Frame, area: Rect) {
		f.render_widget(
			Paragraph::new(format!(
				"{}\n{:?}",
				Chargen::get_render_text(self, app),
				self.stage
			))
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title(TITLE)
					.title_alignment(Alignment::Center)
					.title_position(Position::Bottom)
					.yellow(),
			)
			.alignment(Alignment::Center),
			area,
		)
	}
}

impl Default for Chargen<'_> {
	fn default() -> Self {
		Self {
			title: "Birthing...",
			stage: ChargenStage::Class,
			name: "",
		}
	}
}

#[derive(Copy, Clone)]
pub struct Gameplay<'a> {
	pub title: &'a str,
}

impl InputTarget for Gameplay<'_> {
	fn handle_input(&mut self, _app: &mut App, _c: crossterm::event::KeyCode) {
		//	app = &c.to_string();
	}
}

impl Renderer for Gameplay<'_> {
	fn render_ui(&self, _app: &crate::app::App, f: &mut Frame, area: Rect) {
		f.render_widget(
			Paragraph::new(format!(
				"\
					Adventure!
				",
			))
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title(TITLE)
					.title_alignment(Alignment::Center)
					.title_position(Position::Bottom)
					.yellow(),
			)
			.alignment(Alignment::Center),
			area,
		);
	}
}

impl Default for Gameplay<'_> {
	fn default() -> Self {
		Self {
			title: "Adventuring...",
		}
	}
}
