use crate::{
	chargen::{Chargen, Gameplay},
	game::{Class, Mob, Position},
};

pub const TITLE: &str = "Worldwomb";

#[derive(Copy, Clone)]
pub enum GameScreen<'a> {
	ScreenChargen { screen: Chargen<'a> },
	ScreenGameplay { screen: Gameplay<'a> },
}

impl GameScreen<'_> {
	pub fn handle_input(&mut self, app: &mut App, c: crossterm::event::KeyCode) {
		match self {
			GameScreen::ScreenChargen { ref mut screen } => screen.handle_input(app, c),
			GameScreen::ScreenGameplay { ref mut screen } => screen.handle_input(app, c),
		}
	}
}

impl Renderer for GameScreen<'_> {
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

pub struct App<'a> {
	pub should_quit: bool,
	pub focus: Vec<GameScreen<'a>>,
	pub player: Mob<'a>,
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
				class: Class::Unknown,
				pos: Position { x: 0, y: 0 },
				hp: 5,
			},
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
		println!("popp");
		if let None = self.focus.pop() {
			panic!("No screens left to pop.")
		}
	}
}
