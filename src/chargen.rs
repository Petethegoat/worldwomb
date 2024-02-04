use crate::app::{InputTarget, Renderer, TITLE};
use ratatui::{
	layout::{Alignment, Constraint, Direction, Layout},
	style::Stylize,
	widgets::{block::Position, Block, BorderType, Borders, Paragraph},
};

#[derive(Debug)]
pub struct Chargen<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Chargen<'_> {
	fn handle_input(&mut self, _app: &String, c: char) {
		self.input = c.to_string();
	}
}

impl Renderer for Chargen<'_> {
	fn render_ui(&self, _app: &crate::app::App, f: &mut ratatui::Frame) {
		f.render_widget(
			Paragraph::new(format!(
				"\
					Entering the Worldwomb...\n\
					\n\
					Choose your class:\n\
					1 - Vagrant\n\
					2 - Conscript\n\
					3 - Magician\n\
					{}
				", self.input
			))
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title(TITLE)
					.title_alignment(Alignment::Center)
					.title_position(Position::Bottom)
					.yellow(),
			)
			.alignment(Alignment::Center),
			f.size(),
		)
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

#[derive(Debug)]
pub struct Gameplay<'a> {
	pub title: &'a str,
	pub input: String,
}

impl InputTarget for Gameplay<'_> {
	fn handle_input(&mut self, _app: &String, _c: char) {
		//	app = &c.to_string();
	}
}

impl Renderer for Gameplay<'_> {
	fn render_ui(&self, app: &crate::app::App, f: &mut ratatui::Frame) {
		let layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Min(1), Constraint::Length(3)])
			.split(f.size());

		let sub_layout = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
			.split(layout[0]);

		f.render_widget(
			Paragraph::new(format!(
				"\
					Adventure!
				",
			))
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.title(TITLE)
					.title_alignment(Alignment::Center)
					.title_position(Position::Bottom)
					.yellow(),
			)
			.alignment(Alignment::Center),
			sub_layout[0],
		);

		f.render_widget(
			Block::new()
				.borders(Borders::ALL)
				.border_type(BorderType::Rounded)
				.title(app.player.name)
				.title_alignment(Alignment::Center)
				.title_position(Position::Bottom)
				.yellow(),
			sub_layout[1],
		);

		f.render_widget(
			Paragraph::new("  by Pete Goodfellow  ")
				.alignment(Alignment::Right)
				.block(
					Block::default()
						.borders(Borders::ALL)
						.border_type(BorderType::Thick)
						.yellow(),
				),
			layout[1],
		);
	}
}

impl Default for Gameplay<'_> {
	fn default() -> Self {
		Self {
			title: "Adventuring...",
			input: String::new(),
		}
	}
}
