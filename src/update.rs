use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn update(app: &mut crate::app::App, key_event: KeyEvent) {
	match key_event.code {
		KeyCode::Esc => app.quit(),
		KeyCode::Char('c') | KeyCode::Char('C') => {
			if key_event.modifiers == KeyModifiers::CONTROL {
				app.quit()
			}
		}
		KeyCode::Char(x) => {
			let stack = app.focus;
			if let Some(handler) = stack.last().as_mut() {
				handler.handle_input(&app.input, x)
			}
		}
		_ => {}
	};
}
