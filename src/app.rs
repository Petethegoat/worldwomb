use crate::game::{Class, Mob, Position};

pub const TITLE: &str = "Worldwomb";

pub struct App<'a> {
	pub should_quit: bool,
    pub player: Mob<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            player: Mob {
                name: "Player",
                class: Class::Conscript,
                pos: Position {x: 0, y: 0},
                hp: 5
            }
        }
    }

    pub fn tick(&mut self) {
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
