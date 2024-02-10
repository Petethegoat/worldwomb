use std::fmt;

use ratatui::style::Color;
use ratatui::style::Color::*;

#[derive(Copy, Clone, PartialEq)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

pub struct Renderable {
	pub glyph_left: char,
	pub glyph_right: char,
	pub fg: Color,
}

impl fmt::Display for Position {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {})", self.x, self.y)
	}
}

pub struct Mob {
	pub name: String,
	pub pos: Position,
	pub class: Class,
	pub hp: u8,
	pub hp_max: u8,
	pub item: PlayerItem,
}

pub struct Pushable {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Class {
	Unknown,
	Vagrant,
	Conscript,
	Pilgrim,
}
impl Class {
	pub const ALL: [Self; 3] = [Self::Vagrant, Self::Conscript, Self::Pilgrim];
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Doctrine {
	Unknown,
	Naught,
	Power,
	Knowledge,
	Camaraderie,
}
impl Doctrine {
	pub const ALL: [Self; 4] = [
		Self::Naught,
		Self::Power,
		Self::Knowledge,
		Self::Camaraderie,
	];
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerItem {
	None,
	Torch,
	Sword,
}
impl PlayerItem {
	pub fn get_char(self) -> (char, Color) {
		match self {
			PlayerItem::None => ('⏑', Gray),
			PlayerItem::Torch => ('ٱ', Yellow),
			PlayerItem::Sword => ('˨', Gray),
		}
	}
}

pub struct Title {
	title: String,
	priority: i32,
}
