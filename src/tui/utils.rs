use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Helper function to create a centered rectange using up a certain percentage of the availaable rect `r`
pub(crate) fn centered_rectange(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
	let popout_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Percentage((100 - percent_y) /2),
			Constraint::Percentage(percent_y),
			Constraint::Percentage((100 - percent_y) /2),
		])
		.split(r);

	Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Percentage((100 - percent_x) / 2),
			Constraint::Percentage(percent_x),
			Constraint::Percentage((100 - percent_x) / 2),
		])
		.split(popout_layout[1])[1]
}
