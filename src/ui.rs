use crate::{
	app::{App, Renderer},
	game::{Class, Doctrine},
};
use ratatui::{layout::Offset, prelude::*, widgets::*};

#[derive(Default)]
pub struct CellDraw {
	pub char: char,
	pub x: u16,
	pub y: u16,
	pub color: Color,
}
impl Widget for CellDraw {
	fn render(self, _area: Rect, buf: &mut Buffer) {
		buf.get_mut(self.x, self.y)
			.set_char(self.char)
			.set_fg(self.color);
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

	let statistics_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Length(2),
			Constraint::Length(1),
			Constraint::Length(1),
			Constraint::Min(4),
		])
		.margin(1)
		.split(sub_layout[1]);

	f.render_widget(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.title(&*app.player.name)
			.yellow(),
		sub_layout[1],
	);

	let stat_class = Line::raw(format!("{:?}", app.player.class));
	let stat_doctrine = Line::raw(format!("In pursuit of: {:?}", app.player_doctrine));

	if app.player.class != Class::Unknown {
		stat_class.render(statistics_layout[0], f.buffer_mut());
	}
	if app.player_doctrine != Doctrine::Unknown {
		stat_doctrine.render(
			statistics_layout[0].offset(Offset { x: 0, y: 1 }),
			f.buffer_mut(),
		);
	}

	if app.player.hp_max > 0 {
		f.render_widget(
			Gauge::default()
				.gauge_style(Style::default().fg(Color::Red))
				.ratio(app.player.hp as f64 / app.player.hp_max as f64)
				.label(format!("{} hp", app.player.hp)),
			statistics_layout[1],
		);
	}

	f.render_widget(
		Paragraph::new(&*app.help_text)
			.alignment(Alignment::Center)
			.block(Block::bordered().border_type(BorderType::Thick).yellow()),
		layout[1],
	);

	if let Some(renderer) = app.focus.last() {
		renderer.render_ui(app, f, sub_layout[0]);
	}
}
