use std::path::PathBuf;

use clap::{ArgAction, Parser};

#[derive(Clone, Parser)]
pub(crate) struct Args {
	pub(crate) bookmarks_file: PathBuf,

	#[arg(short = 'V', long, action = ArgAction::SetTrue)]
	pub(crate) version: bool,
}
