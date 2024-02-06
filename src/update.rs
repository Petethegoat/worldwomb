use crate::app::App;
use crossterm::event::{KeyCode, KeyEvent};

fn go_for_quit(app: &mut App, c: crossterm::event::KeyCode) {
	match c {
		KeyCode::Char('y') => app.quit(),
		KeyCode::Char('n') => app.clear_modal(),
		_ => app.clear_modal(),
	}
}

pub fn update(app: &mut App, key_event: KeyEvent) {
	if let Some(modal) = &app.modal {
		(modal.input)(app, key_event.code);
		return;
	}

	match key_event.code {
		KeyCode::Esc => app.set_modal(
			String::from("Quit Game"),
			"Do you really want to quit?".into(),
			"Y / N to pick.".into(),
			go_for_quit,
		),
		x => {
			let mut stack = app.focus.clone();
			if let Some(handler) = stack.iter_mut().last() {
				handler.handle_input(app, x);
			}
			app.focus = stack;
		}
	};
}
