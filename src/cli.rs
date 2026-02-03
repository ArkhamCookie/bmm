use std::path::PathBuf;

use clap::{ArgAction, Parser};

/// Commandline arguments for `bmm`
#[derive(Clone, Parser)]
pub(crate) struct Args {
	/// Give the bookmark file you want used
	pub(crate) bookmarks_file: Option<PathBuf>,

	/// Print version and exit
	#[arg(short = 'V', long, action = ArgAction::SetTrue)]
	pub(crate) version: bool,
}
