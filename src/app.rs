use std::default;

use crate::{
	chargen::{Chargen, Gameplay}, game::{Class, Mob, Position}, ui::GameLayout
};

pub const TITLE: &str = "Worldwomb";

#[default]
#[derive(Default)]
enum GameScreen<'a> {
	ScreenChargen {screen: Chargen<'a>},
	ScreenGameplay {screen: Gameplay<'a>},
}

impl InputTarget for GameScreen<'_> {
	fn handle_input(mut self, c: char) {
		self.screen.input = c.to_string();
	}
}


pub trait InputTarget {
	fn handle_input(self, c: char);
}

pub struct App<'a> {
	pub should_quit: bool,
	pub focus: Vec<GameScreen>,
	pub player: Mob<'a>,
}

impl App<'_> {
	pub fn new() -> Self {
		Self {
			should_quit: false,
			focus: vec![GameScreen::ScreenChargen::default()],
			player: Mob {
				name: "Player",
				class: Class::Conscript,
				pos: Position { x: 0, y: 0 },
				hp: 5,
			},
		}
	}

	pub fn tick(&mut self) {}

	pub fn quit(&mut self) {
		self.should_quit = true;
	}
}
