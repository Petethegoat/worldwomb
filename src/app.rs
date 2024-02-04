use crate::{
	chargen::{Chargen, Gameplay},
	game::{Class, Doctrine, Mob, Position},
};

pub const TITLE: &str = "Worldwomb";
pub const HELP_CONTINUE: &str = "Press enter to continue.";

#[derive(Copy, Clone)]
pub enum GameScreen {
	ScreenChargen { screen: Chargen },
	ScreenGameplay { screen: Gameplay },
}

impl GameScreen {
	pub fn handle_input(&mut self, app: &mut App, c: crossterm::event::KeyCode) {
		match self {
			GameScreen::ScreenChargen { ref mut screen } => screen.handle_input(app, c),
			GameScreen::ScreenGameplay { ref mut screen } => screen.handle_input(app, c),
		}
	}
}

impl Renderer for GameScreen {
	fn render_ui(&self, app: &App, f: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
		match self {
			GameScreen::ScreenChargen { screen } => screen.render_ui(app, f, area),
			GameScreen::ScreenGameplay { screen } => screen.render_ui(app, f, area),
		}
	}
}

pub trait InputTarget {
	fn handle_input(&mut self, app: &mut App, c: crossterm::event::KeyCode);
}

pub trait Renderer {
	fn render_ui(&self, app: &App, f: &mut ratatui::Frame, area: ratatui::prelude::Rect);
}

pub struct App {
	pub should_quit: bool,
	pub focus: Vec<GameScreen>,
	pub player: Mob,
	pub player_doctrine: Doctrine,
	pub help_text: String,
    pub location: String,
}

impl App {
	pub fn new() -> Self {
		Self {
			should_quit: false,
			focus: vec![
				GameScreen::ScreenGameplay {
					screen: Gameplay::default(),
				},
				GameScreen::ScreenChargen {
					screen: Chargen::default(),
				},
			],
			player: Mob {
				name: String::from("Player"),
				class: Class::Unknown,
				pos: Position { x: 0, y: 0 },
				hp: 5,
			},
			player_doctrine: Doctrine::Unknown,
			help_text: String::from(HELP_CONTINUE),
            location: String::from("Beyond the Walls"),
		}
	}

	pub fn tick(&mut self) {}

	pub fn quit(&mut self) {
		if self.focus.len() <= 1 {
			self.should_quit = true;
		} else {
			self.focus.pop();
		}
	}

	pub fn pop_screen(&mut self) {
		if let None = self.focus.pop() {
			panic!("No screens left to pop.")
		}
	}
}
