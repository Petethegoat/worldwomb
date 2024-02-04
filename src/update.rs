use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut crate::app::App, key_event: KeyEvent) {
	match key_event.code {
		KeyCode::Esc => app.quit(),
		KeyCode::Char('c') | KeyCode::Char('C') => {
			if key_event.modifiers == KeyModifiers::CONTROL {
				app.quit()
			}
		}
		x => {
			let mut stack = app.focus.clone();
			for handler in stack.iter_mut() {
				handler.handle_input(app, x)
			}
			app.focus = stack;
		}
		_ => {}
	};
}
