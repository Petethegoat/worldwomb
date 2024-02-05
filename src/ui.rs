use crate::app::{App, Renderer};
use ratatui::{prelude::*, widgets::*};

#[derive(Default)]
pub struct CellDraw {
	pub char: char,
	pub x: u16,
	pub y: u16,
}
impl Widget for CellDraw {
	fn render(self, _area: Rect, buf: &mut Buffer) {
		buf.get_mut(self.x, self.y).set_char(self.char);
	}
}

pub fn render(app: &mut App, f: &mut ratatui::Frame) {
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
			"{:?}\n{} hp\nIn pursuit of: {:?}",
			app.player.class, app.player.hp, app.player_doctrine
		))
		.block(
			Block::new()
				.borders(Borders::ALL)
				.border_type(BorderType::Rounded)
				.title(&*app.player.name)
				.yellow(),
		),
		sub_layout[1],
	);

	f.render_widget(
		Paragraph::new(&*app.help_text)
			.alignment(Alignment::Center)
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Thick)
					.yellow(),
			),
		layout[1],
	);

	if let Some(renderer) = app.focus.last() {
		renderer.render_ui(app, f, sub_layout[0]);
	}
}
