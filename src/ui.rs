use crate::app::{App, Renderer};
use ratatui::{
	prelude::*,
	widgets::{block::Position, *},
};

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
		Paragraph::new(format!("{:?}\n{}", app.player.class, app.player.hp)).block(
			Block::new()
				.borders(Borders::ALL)
				.border_type(BorderType::Rounded)
				.title(app.player.name)
				.title_alignment(Alignment::Center)
				.title_position(Position::Bottom)
				.yellow(),
		),
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

	if let Some(renderer) = app.focus.last() {
		renderer.render_ui(app, f, sub_layout[0]);
	}
}
