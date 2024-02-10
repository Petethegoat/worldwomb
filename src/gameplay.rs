use crate::{
	app::{App, InputTarget, Renderer},
	game::PlayerItem,
	map::{tile, TileType},
	ui::CellDraw,
};
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, style::Color::*, widgets::Widget, Frame};

#[derive(Copy, Clone)]
pub struct Gameplay;

fn try_move(x: i32, y: i32, a: &mut App) {
	if a.map[tile(a.player.pos.x + x, a.player.pos.y + y)] == TileType::Floor {
		let positions = a.ecs.positions.clone();
		for pos in a.ecs.positions.iter_mut() {
			if pos.1.x == a.player.pos.x + x && pos.1.y == a.player.pos.y + y {
				if a.map[tile(pos.1.x + x, pos.1.y + y)] == TileType::Floor {
					for pos2 in positions.iter() {
						if pos2.1.x == pos.1.x + x && pos2.1.y == pos.1.y + y {
							return;
						}
					}
					pos.1.x += x;
					pos.1.y += y;
					break;
				} else {
					return;
				}
			}
		}
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

			KeyCode::Char('1') => a.player.item = PlayerItem::None,
			KeyCode::Char('2') => a.player.item = PlayerItem::Torch,
			KeyCode::Char('3') => a.player.item = PlayerItem::Sword,
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
		let pos = crate::game::Position {
			x: a.player.pos.x,
			y: a.player.pos.y,
		};

		let mut x: i32 = 0;
		let mut y: i32 = 0;
		for cell in a.map.iter() {
			if let TileType::Wall = cell {
				CellDraw::bg(x.try_into().unwrap(), y.try_into().unwrap(), DarkGray)
					.render(area, f.buffer_mut());
			}

			x += 1;
			if x >= (crate::map::WIDTH) + 0 as i32 {
				x = 0;
				y += 1;
			}
		}

		for entity in &a.ecs.renderables {
			if let Some(pos) = a.ecs.positions.get(&entity.0) {
				let r = entity.1;
				CellDraw::entity(pos, r.glyph_left, r.glyph_right, r.fg, r.fg)
					.render(area, f.buffer_mut());
			}
		}

		// Player goes on top.
		let (char, col) = a.player.item.get_char();
		CellDraw::entity(&pos, '@', char, Gray, col).render(area, f.buffer_mut());
		//CellDraw::entity(&pos, '♘', '@', Gray).render(area, f.buffer_mut());
	}
}

impl Default for Gameplay {
	fn default() -> Self {
		Self {}
	}
}
