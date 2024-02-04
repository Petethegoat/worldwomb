use crossterm::event::{KeyCode, KeyEvent};

pub fn update(app: &mut crate::app::App, key_event: KeyEvent) {
	match key_event.code {
		KeyCode::Esc => app.quit(),
		x => {
			let mut stack = app.focus.clone();
			for handler in stack.iter_mut() {
				handler.handle_input(app, x)
			}
			app.focus = stack;
		}
	};
}
