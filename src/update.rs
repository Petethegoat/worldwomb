use crate::app::{App, StyledLine};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::style::{Style, Stylize};

fn go_for_quit(a: &mut App, c: crossterm::event::KeyCode) {
	match c {
		KeyCode::Char('y') => a.quit(),
		KeyCode::Char('n') => a.clear_modal(),
		_ => a.clear_modal(),
	}
}

pub fn update(a: &mut App, key_event: KeyEvent) {
	if let Some(modal) = &a.modal {
		(modal.input)(a, key_event.code);
		return;
	}

	match key_event.code {
		KeyCode::Esc => a.set_modal(
			String::from("Quit Game"),
			"Do you really want to quit?".into(),
			StyledLine {
				text: [
					String::from("‎"),
					String::from(" Y / N "),
					String::from(" to pick."),
				],
				styles: [
					Style::default(),
					Style::default().reversed(),
					Style::default(),
				],
			},
			go_for_quit,
		),
		x => {
			let mut stack = a.focus.clone();
			if let Some(handler) = stack.iter_mut().last() {
				handler.handle_input(a, x);
			}
			a.focus = stack;
		}
	};
}
