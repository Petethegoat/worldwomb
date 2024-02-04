use crate::{
	app::{App, InputTarget, Renderer, HELP_CONTINUE},
	game::{Class, Doctrine},
};
use crossterm::event::KeyCode;
use ratatui::{
	layout::{Alignment, Rect}, style::Stylize, widgets::{block::Position, Block, BorderType, Borders, Paragraph, Wrap}, Frame
};

#[derive(Copy, Clone)]
pub struct Chargen {
	stage: ChargenStage,
}

#[derive(Debug, Copy, Clone)]
enum ChargenStage {
	Intro,
	Class,
	Doctrine,
	Name,
	Farewell,
}

impl Chargen {
	pub fn get_render_text(&self, app: &App) -> String {
		match self.stage {
			ChargenStage::Intro => {
				format!("\
				The Worldwomb.\n\n\
				An unfathomable citadel, stretching for miles down the Icon Coast. Its walls rise far above the proudest minaret. No quarry on the continent runs deep enough, and one hundred thousand masons could not dress so much stone.\n\n\
				Most who arrive here now come as part of prison gangs, forced to explore within at the point of a blade. At the eastmost extent, in the shadow of a single towering buttress, a thriving market serves those foolish enough to enter by choice.\n\n\
				A young officer is preparing to scribe your details for the census...\
				")
			}
			ChargenStage::Class => {
				let mut s = format!("\"State thy occupation.\"\n\n");

				let mut j = 1;
				for class in Class::ALL {
					s.push_str(&format!("{j} - {class:?}\n").to_owned());
					j += 1;
				}
				s
			}
			ChargenStage::Doctrine => {
				let mut s = format!("\"Thy doctrine?\"\n\n");

				let mut j = 1;
				for doctrine in Doctrine::ALL {
					s.push_str(&format!("{j} - {doctrine:?}\n").to_owned());
					j += 1;
				}
				s
			}
			ChargenStage::Name => {
				format!(
					"\"And thy name, {:?}?\"\n\n{}",
					app.player.class, app.player.name
				)
			}
			ChargenStage::Farewell => {
				if let Class::Vagrant = app.player.class {
					format!(
						"\"Heed my words, {:?}.\n\nBeware the Worldwomb.\"",
						app.player.class
					)
				} else {
					match app.player_doctrine {
						Doctrine::Camaraderie => {
							format!(
							"\"Thy will find no companionship within those walls.\nHeed my words.\n\nBeware the Worldwomb.\"",
						)
						}
						Doctrine::Naught => {
							format!(
							"\"I pray thou has reason to enter, even if thou may not state it.\nBut heed my words.\n\nBeware the Worldwomb.\"",
						)
						}
						_ => {
							format!(
							"\"{}, I fare thee well.\nBut heed my words.\n\nBeware the Worldwomb.\"",
							app.player.name
						)
						}
					}
				}
			}
		}
	}
}

impl InputTarget for Chargen {
	fn handle_input(&mut self, app: &mut App, c: crossterm::event::KeyCode) {
		match self.stage {
			ChargenStage::Intro => {
				if let KeyCode::Enter = c {
					self.stage = ChargenStage::Class;
					app.help_text = String::from("Press 1-3 to select class.");
				}
			}
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
					self.stage = ChargenStage::Doctrine;
					app.help_text = String::from("Press 1-4 to select doctrine.");
				}
			}
			ChargenStage::Doctrine => {
				match c {
					KeyCode::Char('1') => {
						app.player_doctrine = Doctrine::Naught;
					}
					KeyCode::Char('2') => {
						app.player_doctrine = Doctrine::Power;
					}
					KeyCode::Char('3') => {
						app.player_doctrine = Doctrine::Knowledge;
					}
					KeyCode::Char('4') => {
						app.player_doctrine = Doctrine::Camaraderie;
					}
					_ => {}
				}
				if app.player_doctrine != Doctrine::Unknown {
					self.stage = ChargenStage::Name;
					app.player.name = String::new();
					app.help_text = String::from("Enter your name, and press enter to continue.");
				}
			}
			ChargenStage::Name => match c {
				KeyCode::Enter => {
					self.stage = ChargenStage::Farewell;
					app.help_text = String::from(HELP_CONTINUE);
				}
				KeyCode::Backspace => {
					app.player.name.pop();
				}
				KeyCode::Char(c) => {
					app.player.name.push(c);
				}
				_ => {}
			},
			ChargenStage::Farewell => {
				if let KeyCode::Enter = c {
					app.location = String::from("Within the Worldwomb");
					app.help_text = String::from("Use arrows or WASD to traverse the Worldwomb.");
					app.pop_screen();
				}
			}
		}
	}
}

impl Renderer for Chargen {
	fn render_ui(&self, app: &crate::app::App, f: &mut Frame, area: Rect) {
		f.render_widget(
			Paragraph::new(Chargen::get_render_text(self, app))
				.wrap(Wrap::default())
				.block(
					Block::default()
						.borders(Borders::ALL)
						.border_type(BorderType::Rounded)
						.title(&*app.location)
						.title_alignment(Alignment::Center)
						.title_position(Position::Bottom)
						.yellow(),
				)
				.alignment(Alignment::Center),
			area,
		)
	}
}

impl Default for Chargen {
	fn default() -> Self {
		Self {
			stage: ChargenStage::Intro,
		}
	}
}
