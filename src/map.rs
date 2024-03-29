use crate::{entity::*, game::*};
use rand::Rng;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
	Wall,
	Floor,
}

pub fn tile(x: i32, y: i32) -> usize {
	(y as usize * WIDTH as usize) + x as usize
}

pub const WIDTH: i32 = 38;
pub const HEIGHT: i32 = 25;
pub fn new_map() -> Vec<TileType> {
	let mut map = vec![TileType::Floor; WIDTH as usize * HEIGHT as usize];

	//Walls at boundaries
	for x in 0..WIDTH {
		map[tile(x, 0)] = TileType::Wall;
		map[tile(x, HEIGHT - 1)] = TileType::Wall;
	}
	for y in 0..HEIGHT {
		map[tile(0, y)] = TileType::Wall;
		map[tile(WIDTH - 1, y)] = TileType::Wall;
	}

	let mut rng = rand::thread_rng();
	for _i in 0..WIDTH + HEIGHT {
		let x = rng.gen_range(0..WIDTH);
		let y = rng.gen_range(0..HEIGHT);
		map[tile(x, y)] = TileType::Wall;
	}

	map
}

pub fn decorate_map(ecs: &mut EntitySystem) {
	let mut rng = rand::thread_rng();
	for _i in 0..(WIDTH + HEIGHT) / 3 {
		let x = rng.gen_range(0..WIDTH);
		let y = rng.gen_range(0..HEIGHT);

		let e = ecs.add_entity();
		ecs.add_position(e, Position { x, y });
		ecs.add_render(
			e,
			Renderable {
				glyph_left: '෴',
				glyph_right: ' ',
				fg: ratatui::style::Color::Green,
			},
		);
	}
	for _i in 0..(WIDTH + HEIGHT) / 3 {
		let x = rng.gen_range(0..WIDTH);
		let y = rng.gen_range(0..HEIGHT);

		let e = ecs.add_entity();
		ecs.add_position(e, Position { x, y });
		ecs.add_render(
			e,
			Renderable {
				glyph_left: ' ',
				glyph_right: '෴',
				fg: ratatui::style::Color::Green,
			},
		);
	}
}
