use crate::app::{App, Renderer};

pub fn render(app: &mut App, f: &mut ratatui::Frame) {
	if let Some(renderer) = app.focus.last() {
		renderer.render_ui(app, f);
	}
}
