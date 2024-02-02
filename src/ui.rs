use ratatui::{
	layout::{Constraint, Direction, Layout},
	prelude::{Alignment, Frame},
	style::{Color, Style},
	widgets::{block::*, Block, BorderType, Borders, Paragraph},
};

use crate::app::{App, TITLE};

pub fn render(app: &mut App, f: &mut Frame) {
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
				Entering the Worldwomb...\n\
				\n\
				Choose your class:\n\
				1 - Vagrant\n\
				2 - Conscript\n\
				3 - Magician\n\
			",
		))
		.block(
			Block::default()
				.borders(Borders::ALL)
				.border_type(BorderType::Rounded)
				.title(TITLE)
				.title_alignment(Alignment::Center)
				.title_position(Position::Bottom)
				.style(Style::default().fg(Color::Yellow)),
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
			.style(Style::default().fg(Color::Yellow)),
		sub_layout[1],
	);

	f.render_widget(
		Paragraph::new("by Pete Goodfellow")
			.alignment(Alignment::Right)
			.block(
				Block::default()
					.borders(Borders::ALL)
					.border_type(BorderType::Rounded)
					.style(Style::default().fg(Color::Yellow)),
			),
		layout[1],
	);
}
