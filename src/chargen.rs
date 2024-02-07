use crate::{
	app::{App, InputTarget, Renderer, HELP_CONTINUE},
	game::{Class, Doctrine},
};
use crossterm::event::KeyCode;
use rand::Rng;
use ratatui::{
	layout::{Alignment, Rect},
	widgets::{Paragraph, Wrap},
	Frame,
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
	pub fn get_render_text(&self, a: &App) -> String {
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
					a.player.class, a.player.name
				)
			}
			ChargenStage::Farewell => {
				if let Class::Vagrant = a.player.class {
					format!(
						"\"Heed my words, {:?}.\n\nBeware the Worldwomb.\"",
						a.player.class
					)
				} else {
					match a.player_doctrine {
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
							a.player.name
						)
						}
					}
				}
			}
		}
	}
}

impl InputTarget for Chargen {
	fn handle_input(&mut self, a: &mut App, c: crossterm::event::KeyCode) {
		match self.stage {
			ChargenStage::Intro => {
				if let KeyCode::Enter = c {
					self.stage = ChargenStage::Class;
					a.help_text = String::from("Press 1-3 to select class.");
				}
			}
			ChargenStage::Class => {
				match c {
					KeyCode::Char('1') => {
						a.player.class = Class::Vagrant;
					}
					KeyCode::Char('2') => {
						a.player.class = Class::Conscript;
					}
					KeyCode::Char('3') => {
						a.player.class = Class::Pilgrim;
					}
					_ => {}
				}
				if a.player.class != Class::Unknown {
					self.stage = ChargenStage::Doctrine;
					a.help_text = String::from("Press 1-4 to select doctrine.");
				}
			}
			ChargenStage::Doctrine => {
				match c {
					KeyCode::Char('1') => {
						a.player_doctrine = Doctrine::Naught;
					}
					KeyCode::Char('2') => {
						a.player_doctrine = Doctrine::Power;
					}
					KeyCode::Char('3') => {
						a.player_doctrine = Doctrine::Knowledge;
					}
					KeyCode::Char('4') => {
						a.player_doctrine = Doctrine::Camaraderie;
					}
					_ => {}
				}

				if a.player_doctrine != Doctrine::Unknown {
					self.stage = ChargenStage::Name;
					a.player.name = String::new();
					a.help_text = String::from("Enter your name, and press enter to continue.");
				}
			}
			ChargenStage::Name => match c {
				KeyCode::Enter => {
					self.stage = ChargenStage::Farewell;
					a.help_text = String::from(HELP_CONTINUE);

					a.player.hp_max = a.rng.gen_range(100..=120);
					a.player.hp = a.player.hp_max;
				}
				KeyCode::Backspace => {
					a.player.name.pop();
				}
				KeyCode::Char(c) => {
					a.player.name.push(c);
				}
				_ => {}
			},
			ChargenStage::Farewell => {
				if let KeyCode::Enter = c {
					a.location = String::from("Within the Worldwomb");
					a.help_text = String::from("Use arrows or WASD to traverse the Worldwomb.");
					a.pop_screen();
				}
			}
		}
	}
}

impl Renderer for Chargen {
	fn render_ui(&self, a: &crate::app::App, f: &mut Frame, area: Rect) {
		f.render_widget(
			Paragraph::new(Chargen::get_render_text(self, a))
				.wrap(Wrap { trim: true })
				.alignment(Alignment::Center),
			area.inner(&ratatui::layout::Margin::new(1, 1)),
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
