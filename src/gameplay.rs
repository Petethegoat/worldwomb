use crate::{
	app::{App, InputTarget, Renderer, HELP_CONTINUE},
	game::{Class, Doctrine},
};
use crossterm::event::KeyCode;
use ratatui::{
	layout::{Alignment, Rect}, style::{Color, Stylize}, terminal, widgets::{
		block::Position,
		canvas::{Canvas, Points},
		Block, BorderType, Borders, Paragraph, Wrap,
	}, Frame
};

#[derive(Copy, Clone)]
pub struct Gameplay {}

impl InputTarget for Gameplay {
	fn handle_input(&mut self, app: &mut App, c: crossterm::event::KeyCode) {
		match c {
			KeyCode::Left | KeyCode::Char('a') => app.player.pos.x -= 1,
			KeyCode::Right | KeyCode::Char('d') => app.player.pos.x += 1,
			KeyCode::Up | KeyCode::Char('w') => app.player.pos.y += 1,
			KeyCode::Down | KeyCode::Char('s') => app.player.pos.y -= 1,
			_ => {}
		}
	}
}

impl Renderer for Gameplay {
	fn render_ui(&self, app: &crate::app::App, f: &mut Frame, area: Rect) {
		f.render_widget(
			Canvas::default()
				.x_bounds([-4.0, 4.0])
				.y_bounds([-4.0, 4.0])
				.paint(|ctx| {
					ctx.draw(&Points {
						coords: &[(app.player.pos.x.into(), app.player.pos.y.into())],
						color: Color::Red,
					});
				})
				.block(
					Block::default()
						.borders(Borders::ALL)
						.border_type(BorderType::Rounded)
						.title(&*app.location)
						.title_alignment(Alignment::Center)
						.title_position(Position::Bottom)
						.yellow(),
				),
			area,
		);
	}
}

impl Default for Gameplay {
	fn default() -> Self {
		Self {}
	}
}
