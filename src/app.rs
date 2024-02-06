use rand::rngs::ThreadRng;

use crate::{
	chargen::Chargen,
	game::{Class, Doctrine, Mob, Position},
	gameplay::Gameplay,
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

pub struct Modal {
	pub title: String,
	pub text: String,
	pub help: String,
	pub input: fn(app: &mut App, c: crossterm::event::KeyCode),
}

pub struct App {
	pub should_quit: bool,
	pub focus: Vec<GameScreen>,
	pub player: Mob,
	pub player_doctrine: Doctrine,
	pub help_text: String,
	pub location: String,
	pub rng: ThreadRng,
	pub modal: Option<Modal>,
	should_pop: bool,
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
				name: String::new(),
				class: Class::Unknown,
				pos: Position { x: 4, y: 2 },
				hp: 0,
				hp_max: 0,
			},
			player_doctrine: Doctrine::Unknown,
			help_text: String::from(HELP_CONTINUE),
			location: String::from("Beyond the Walls"),
			rng: rand::thread_rng(),
			modal: Option::None,
			should_pop: false,
		}
	}

	pub fn tick(&mut self) {
		//	self.help_text = format!("{}", self.rng.gen_range(1..100));
	}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	/// Queue up the pop for execution at the end of the frame.
	pub fn pop_screen(&mut self) {
		self.should_pop = true;
	}

	pub fn post_update(&mut self) {
		if self.should_pop {
			self.should_pop = false;
			if let None = self.focus.pop() {
				panic!("No screens left to pop.")
			}
		}
	}

	pub fn set_modal(
		&mut self,
		title: String,
		text: String,
		help: String,
		input: fn(app: &mut App, c: crossterm::event::KeyCode),
	) {
		self.modal = Some(Modal {
			title,
			text,
			help,
			input,
		})
	}

	pub fn clear_modal(&mut self) {
		self.modal = Option::None;
	}
}
