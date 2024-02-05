use crate::{
	app::{App, InputTarget, Renderer},
	ui::CellDraw,
};
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, widgets::Widget, Frame};

#[derive(Copy, Clone)]
pub struct Gameplay;

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
		CellDraw {
			char: '@',
			x: app.player.pos.x.try_into().unwrap(),
			y: app.player.pos.y.try_into().unwrap(),
		}
		.render(area, f.buffer_mut())
	}
}

impl Default for Gameplay {
	fn default() -> Self {
		Self {}
	}
}
