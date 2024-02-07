use crate::{
	app::{App, Renderer},
	game::{Class, Doctrine, Position},
};
use ratatui::{layout::Offset, prelude::*, widgets::*};

#[derive(Default)]
pub struct CellDraw {
	pub glyph_left: Option<char>,
	pub glyph_right: Option<char>,
	pub x: u16,
	pub y: u16,
	pub fg: Option<Color>,
	pub bg: Option<Color>,
}

impl CellDraw {
	pub fn bg(x: u16, y: u16, bg: Color) -> CellDraw {
		CellDraw {
			glyph_left: None,
			glyph_right: None,
			x,
			y,
			fg: None,
			bg: Some(bg),
		}
	}
	pub fn entity(pos: &Position, left: char, right: char, fg: Color) -> CellDraw {
		CellDraw {
			glyph_left: Some(left),
			glyph_right: Some(right),
			x: <i32 as TryInto<u16>>::try_into(pos.x).unwrap(),
			y: <i32 as TryInto<u16>>::try_into(pos.y).unwrap(),
			fg: Some(fg),
			bg: None,
		}
	}
}

impl Widget for CellDraw {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let b = buf.get_mut(self.x * 2 + area.x, self.y + area.y);
		if let Some(c) = self.glyph_left {
			b.set_char(c);
		}
		if let Some(fg) = self.fg {
			b.set_fg(fg);
		}
		if let Some(bg) = self.bg {
			b.set_bg(bg);
		}
		let b = buf.get_mut(self.x * 2 + area.x + 1, self.y + area.y);
		if let Some(c) = self.glyph_right {
			b.set_char(c);
		}
		if let Some(fg) = self.fg {
			b.set_fg(fg);
		}
		if let Some(bg) = self.bg {
			b.set_bg(bg);
		}
	}
}

pub fn render(a: &mut App, f: &mut ratatui::Frame) {
	let margin_1 = Margin::new(1, 1);

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
		.vertical_margin(1)
		.horizontal_margin(2)
		.split(sub_layout[1]);

	let gamescreen_layout = sub_layout[0].inner(&margin_1);

	f.render_widget(
		Block::default()
			.borders(Borders::ALL)
			.border_type(BorderType::Rounded)
			.title(format!(" {} ", &*a.location))
			.title_alignment(Alignment::Center)
			.title_position(block::Position::Bottom)
			.yellow(),
		sub_layout[0],
	);

	let name;
	if a.player.name.len() > 0 {
		name = format!(" {} ", &*a.player.name);
	} else {
		name = String::new();
	}

	f.render_widget(
		Block::bordered()
			.border_type(BorderType::Rounded)
			.title(name)
			.yellow(),
		sub_layout[1],
	);

	let stat_class = Line::raw(format!("{:?}", a.player.class));
	let stat_doctrine = Line::raw(format!("In pursuit of: {:?}", a.player_doctrine));

	if a.player.class != Class::Unknown {
		stat_class.render(statistics_layout[0], f.buffer_mut());
	}
	if a.player_doctrine != Doctrine::Unknown {
		stat_doctrine.render(
			statistics_layout[0].offset(Offset { x: 0, y: 1 }),
			f.buffer_mut(),
		);
	}

	if a.player.hp_max > 0 {
		f.render_widget(
			Gauge::default()
				.gauge_style(Style::default().fg(Color::Red))
				.ratio(a.player.hp as f64 / a.player.hp_max as f64)
				.label(format!("{} hp", a.player.hp)),
			statistics_layout[1],
		);
	}

	f.render_widget(
		Paragraph::new(&*a.help_text)
			.alignment(Alignment::Center)
			.block(Block::bordered().border_type(BorderType::Thick).yellow()),
		layout[1],
	);

	if let Some(renderer) = a.focus.last() {
		renderer.render_ui(a, f, gamescreen_layout);
	}

	if let Some(modal) = &a.modal {
		//Render modal
		let modal_rect = centered_rect(sub_layout[0], 55, 40);
		let modal_inner = modal_rect.inner(&Margin::new(5, 2));
		let modal_chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([Constraint::Min(1), Constraint::Length(1)])
			.split(modal_inner);
		Clear.render(modal_rect, f.buffer_mut());
		f.render_widget(
			Block::bordered()
				.border_type(BorderType::Thick)
				.gray()
				.title(format!(" {} ", modal.title))
				.title_alignment(Alignment::Center),
			modal_rect,
		);
		f.render_widget(
			Paragraph::new(&*modal.text)
				.wrap(Wrap { trim: true })
				.gray(),
			modal_chunks[0],
		);

		f.render_widget(
			Paragraph::new(modal.help.clone().line())
				.right_aligned()
				.wrap(Wrap { trim: true })
				.gray(),
			modal_chunks[1],
		);
	}
}

/// # Usage
///
/// ```rust
/// let rect = centered_rect(f.size(), 50, 50);
/// ```
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
	let popup_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Percentage((100 - percent_y) / 2),
			Constraint::Percentage(percent_y),
			Constraint::Percentage((100 - percent_y) / 2),
		])
		.split(r);

	Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Percentage((100 - percent_x) / 2),
			Constraint::Percentage(percent_x),
			Constraint::Percentage((100 - percent_x) / 2),
		])
		.split(popup_layout[1])[1]
}
