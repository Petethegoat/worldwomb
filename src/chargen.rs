use crate::app::InputTarget;

pub struct Chargen<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Chargen<'_> {
	fn handle_input(mut self, c: char) -> Self {
		self.input = c.to_string();
		self
	}
}

impl Default for Chargen<'_> {
	fn default() -> Self {
		Self { 
			title: "Birthing...",
			input: String::new(),
		}
	}
}
pub struct Gameplay<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Gameplay<'_> {
	fn handle_input(mut self, c: char) -> Self {
		self.input = c.to_string();
		self
	}
}

impl Default for Gameplay<'_> {
	fn default() -> Self {
		Self { 
			title: "Playing...",
			input: String::new(),
		}
	}
}