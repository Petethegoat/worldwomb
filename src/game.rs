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

pub struct Mob<'a> {
	pub name: &'a str,
	pub pos: Position,
	pub class: Class,
	pub hp: u8,
}

#[derive(Debug, Copy, Clone)]
pub enum Class {
	Vagrant,
	Conscript,
	Magician,
}
impl Class {
	pub const ALL: [Self; 3] = [Self::Vagrant, Self::Conscript, Self::Magician];
}
