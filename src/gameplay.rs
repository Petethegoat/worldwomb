use crate::{
	app::{App, InputTarget, Renderer},
	map::{tile, TileType},
	ui::CellDraw,
};
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, widgets::Widget, Frame};

#[derive(Copy, Clone)]
pub struct Gameplay;

fn try_move(x: i32, y: i32, a: &mut App) {
	if a.map[tile(a.player.pos.x + x, a.player.pos.y + y)] == TileType::Floor {
		a.player.pos.x += x;
		a.player.pos.y += y;
	}
}

impl InputTarget for Gameplay {
	fn handle_input(&mut self, a: &mut App, c: crossterm::event::KeyCode) {
		if a.player.hp <= 0 {
			return;
		}

		let prev_pos = a.player.pos.clone();

		match c {
			KeyCode::Left | KeyCode::Char('a') => try_move(-1, 0, a),
			KeyCode::Right | KeyCode::Char('d') => try_move(1, 0, a),
			KeyCode::Up | KeyCode::Char('w') => try_move(0, -1, a),
			KeyCode::Down | KeyCode::Char('s') => try_move(0, 1, a),
			_ => return,
		}

		if a.player.pos != prev_pos {
			if a.player.hp > 0 {
				a.player.hp -= 1;
				if a.player.hp == 0 {
					a.help_text = String::from("You have perished. Press escape to quit.");
				}
			}
		}
	}
}

impl Renderer for Gameplay {
	fn render_ui(&self, a: &crate::app::App, f: &mut Frame, area: Rect) {
		let x_off: i32 = area.x.into();
		let y_off: i32 = area.y.into();
		let pos = crate::game::Position {
			x: a.player.pos.x + x_off,
			y: a.player.pos.y + y_off,
		};

		CellDraw::entity(pos, '@', ratatui::style::Color::Gray).render(area, f.buffer_mut());

		let mut x: i32 = area.x.into();
		let mut y: i32 = area.y.into();
		for cell in a.map.iter() {
			if let TileType::Wall = cell {
				CellDraw::bg(
					x.try_into().unwrap(),
					y.try_into().unwrap(),
					ratatui::style::Color::Gray,
				)
				.render(area, f.buffer_mut());
			}

			x += 1;
			if x >= crate::map::WIDTH + x_off as i32 {
				x = area.x.into();
				y += 1;
			}
		}
	}
}

impl Default for Gameplay {
	fn default() -> Self {
		Self {}
	}
}
