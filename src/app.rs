use crate::{
	chargen::{Chargen, Gameplay},
	game::{Class, Mob, Position},
};

pub const TITLE: &str = "Worldwomb";

#[derive(Debug)]
pub enum GameScreen<'a> {
	ScreenChargen { screen: Chargen<'a> },
	ScreenGameplay { screen: Gameplay<'a> },
}

impl GameScreen<'_> {
	pub fn handle_input(&mut self, app: &String, c: char) {
		match self {
			GameScreen::ScreenChargen { ref mut screen } => screen.handle_input(app, c),
			GameScreen::ScreenGameplay { ref mut screen } => screen.handle_input(app, c),
		}
	}
}

impl Renderer for GameScreen<'_> {
	fn render_ui(&self, app: &App, f: &mut ratatui::Frame) {
		match self {
			GameScreen::ScreenChargen { screen } => screen.render_ui(app, f),
			GameScreen::ScreenGameplay { screen } => screen.render_ui(app, f),
		}
	}
}

pub trait InputTarget {
	fn handle_input(&mut self, app: &String, c: char);
}

pub trait Renderer {
	fn render_ui(&self, app: &App, f: &mut ratatui::Frame);
}

#[derive(Copy)]
pub struct App<'a> {
	pub should_quit: bool,
	pub focus: Vec<GameScreen<'a>>,
	pub player: Mob<'a>,
	pub input: String,
}

impl App<'_> {
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
				name: "Player",
				class: Class::Conscript,
				pos: Position { x: 0, y: 0 },
				hp: 5,
			},
			input: String::new(),
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
}
