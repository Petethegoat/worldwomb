use crate::game::{Position, Renderable};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Entity {
	pub id: usize,
}

pub struct EntitySystem {
	pub entities: Vec<Entity>,
	pub renderables: HashMap<usize, Renderable>,
	pub positions: HashMap<usize, Position>,
}
impl EntitySystem {
	pub fn new() -> Self {
		EntitySystem {
			entities: vec![],
			renderables: HashMap::new(),
			positions: HashMap::new(),
		}
	}

	pub fn add_entity(&mut self) -> usize {
		let id = self.entities.len();
		self.entities.push(Entity { id });
		id
	}

	pub fn add_position(&mut self, id: usize, pos: Position) -> usize {
		self.positions.insert(id, pos);
		id
	}

	pub fn add_render(&mut self, id: usize, rend: Renderable) -> usize {
		self.renderables.insert(id, rend);
		id
	}
}
