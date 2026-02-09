use std::error;
use std::io;

use crate::cli::Args;
use crate::conifg::{Bookmark, FileConfig};
use crate::tui::ui::ui;

use clap::ValueEnum;

use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::{
	self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind,
};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{
	EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use tui_input::Input;

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

impl CurrentScreen {
	/// Allows help messages for current screen option
	#[allow(dead_code)]
	pub(crate) fn description(&self) -> &'static str {
		match self {
			CurrentScreen::Default => "Default screen for viewing all bookmarks",
			CurrentScreen::Editing => "Screen for editing a bookmark",
			CurrentScreen::Exiting => "Screen for exiting `bmm` TUI mode",
			CurrentScreen::Help => "Help screen for `bmm`",
		}
	}
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
		terminal.draw(|f| ui(f, app)).unwrap(); // TODO: Handle unwrap better

		if let Event::Key(key) = event::read()? {
			if key.kind == KeyEventKind::Release {
				continue;
			}

			match app.current_screen {
				CurrentScreen::Default => match key.code {
					KeyCode::Char('q') => {
						app.current_screen = CurrentScreen::Exiting
					},
					KeyCode::Char('?') => {
						app.current_screen = CurrentScreen::Help;
					}
					_ => {},
				},
				CurrentScreen::Editing => match key.code {
					KeyCode::Esc => {
						app.current_screen = CurrentScreen::Default;
					},
					_ => {},
				}
				CurrentScreen::Exiting => match key.code {
					KeyCode::Char('y') => {
						return Ok(String::new())
					},
					KeyCode::Char('n') => {
						app.current_screen = CurrentScreen::Default;
					},
					_ => {},
				}
				CurrentScreen::Help => match key.code {
					KeyCode::Char('?') => {
						app.current_screen = CurrentScreen::Default
					},
					KeyCode::Char('q') => {
						app.current_screen = CurrentScreen::Exiting;
					},
					_ => {},
				}
			}
		}
	}
}

/// Start `bmm` in interactive TUI mode
pub(crate) fn interactive(args: &Args) -> Result<(), Box<dyn error::Error>> {
	enable_raw_mode()?;

	execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

	let backend = CrosstermBackend::new(io::stderr());
	let mut terminal = Terminal::new(backend)?;

	let bookmark_file = &args.bookmarks_file.as_ref().unwrap();
	let config = FileConfig::get(*bookmark_file)?;

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
