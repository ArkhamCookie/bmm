use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};

use crate::tui::app::{App, CurrentScreen};
use crate::tui::utils::centered_rectange;

/// The UI for `bmm`
pub(crate) fn ui(frame: &mut Frame, app: &App) {
	// TUI Areas
	let chunks = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Length(3),
			Constraint::Min(1),
			Constraint::Length(3),
		])
		.split(frame.area());
	// TUI Title Block
	let title_block = Block::default()
		.borders(Borders::ALL)
		.style(Style::default());

	let title =
		Paragraph::new(Text::styled("bmm", Style::default().fg(Color::Green))).block(title_block);

	frame.render_widget(title, chunks[0]);

	let mut bookmarks = Vec::<ListItem>::new();

	for bookmark in &app.bookmarks {
		let mut content = String::new();

		if bookmark.name.is_some() {
			content.push_str(bookmark.name.clone().unwrap().as_str());
			content.push_str(": ");
		}

		content.push_str(&bookmark.link);

		if bookmark.description.is_some() {
			content.push_str(" — ");
			content.push_str(bookmark.description.clone().unwrap().as_str());
		}

		bookmarks.push(ListItem::new(Span::styled(content, Style::default())));
	}

	let list = List::new(bookmarks);

	frame.render_widget(list, chunks[1]);

	// TUI Footer Block
	let footer_keys = Span::styled(
		"(?) for help menu / Created by ArkhamCookie",
		Style::default().fg(Color::Red),
	);

	let footer =
		Paragraph::new(Line::from(footer_keys)).block(Block::default().borders(Borders::ALL));

	frame.render_widget(footer, chunks[2]);

	// Quiting CurrentScreen
	if let CurrentScreen::Exiting = &app.current_screen {
		let popup_block = Block::default()
			.title("Y/N")
			.style(Style::default().bg(Color::DarkGray));
		let exit_text = Text::styled("Would you like to exit?", Style::default().fg(Color::Red));
		let exit_paragraph = Paragraph::new(exit_text)
			.block(popup_block)
			.wrap(Wrap { trim: false });
		let area = centered_rectange(60, 25, frame.area());

		frame.render_widget(exit_paragraph, area);
	}

	// Help screen
	if let CurrentScreen::Help = &app.current_screen {
		let popup_block = Block::default()
			.title("Help Menu")
			.style(Style::default().bg(Color::DarkGray));
		let help_text = Text::styled(
			"(?) to open/close the help menu
(q) to quit
(n) to create a new bookmark",
			Style::default().fg(Color::Red),
		);
		let help_paragraph = Paragraph::new(help_text)
			.block(popup_block)
			.wrap(Wrap { trim: false });
		let area = centered_rectange(60, 25, frame.area());

		frame.render_widget(help_paragraph, area);
	}
}
