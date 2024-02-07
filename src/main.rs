pub mod app;
pub mod chargen;
pub mod event;
pub mod game;
pub mod gameplay;
pub mod map;
pub mod tui;
pub mod ui;
pub mod update;

use app::App;
use event::{Event, EventHandler};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use update::update;
type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

fn main() -> Result<()> {
	let mut a = App::new();

	let backend = CrosstermBackend::new(std::io::stderr());
	let terminal = Terminal::new(backend)?;
	let events = EventHandler::new(250);
	let mut tui = Tui::new(terminal, events);
	tui.enter()?;

	while !a.should_quit {
		tui.draw(&mut a)?;

		match tui.events.next()? {
			Event::Tick => {
				a.tick();
			}
			Event::Key(key_event) => update(&mut a, key_event),
			Event::Mouse(_) => {}
			Event::Resize(_, _) => {}
			Event::FocusChange => {
				a.tick();
			}
		};

		a.post_update();
	}

	tui.exit()?;
	Ok(())
}
