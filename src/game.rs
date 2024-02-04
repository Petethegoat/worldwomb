use std::fmt;

pub struct Position {
	pub x: i32,
	pub y: i32,
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
}

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
