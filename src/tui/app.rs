use std::error;
use std::io;

use crate::cli::Args;
use crate::conifg::Bookmark;
use crate::conifg::Config;

use clap::ValueEnum;

use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
	EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

/// Avaiable screens for `bmm` TUI
#[derive(Clone, Copy, ValueEnum)]
pub(crate) enum CurrentScreen {
	/// Default mode viewing all the bookmarks in a file
	Default,
	/// Edit mode for editing a bookmark
	Editing,
	/// Screen used for exiting
	Exiting,
	/// Help screen
	Help,
}

/// `bmm` TUI App
pub(crate) struct App {
	/// Bookmarks being managed
	pub(crate) bookmarks: Vec<Bookmark>,
	/// Current screen of app
	pub(crate) current_screen: CurrentScreen,
	///
	pub(crate) input: Input,
}

impl App {
	/// Init a `bmm` TUI App
	pub(crate) fn init(bookmarks: &Vec<Bookmark>, screen: &CurrentScreen) -> Self {
		Self {
			bookmarks: bookmarks.to_vec(),
			current_screen: *screen,
			input: Input::default(),
		}
	}
}

/// Run a `bmm` app
fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<String> {
	loop {
		todo!("start run_app fn")
	}
}

///
pub(crate) fn interactive(args: &Args) -> Result<(), Box<dyn error::Error>> {
	enable_raw_mode()?;

	execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

	let backend = CrosstermBackend::new(io::stderr());
	let mut terminal = Terminal::new(backend)?;

	let bookmark_file = &args.bookmarks_file.as_ref().unwrap();
	let config = Config::get(*bookmark_file)?;

	let screen: CurrentScreen;
	let screen_arg = args.screen;
	match screen_arg {
		Some(given_screen) => screen = given_screen,
		None => screen = CurrentScreen::Default,
	};

	let mut app = App::init(&config.bookmarks, &screen);

	run_app(&mut terminal, &mut app)?;

	disable_raw_mode()?;
	execute!(
		terminal.backend_mut(),
		LeaveAlternateScreen,
		DisableMouseCapture,
	)?;
	terminal.show_cursor()?;

	Ok(())
}
