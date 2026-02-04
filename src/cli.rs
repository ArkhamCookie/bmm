use std::path::PathBuf;

use clap::{ArgAction, Parser, Subcommand};

/// `bmm` commands
#[derive(Clone, Subcommand)]
pub(crate) enum Command {
	/// Add a bookmark
	Add {
		link: String,
		name: Option<String>,
		description: Option<String>,
	},
	/// Remove a bookmark
	Rm { bookmark: String },
	/// List bookmarks
	List {},
	/// View a bookmark
	View { bookmark: String },
}

/// Commandline arguments for `bmm`
#[derive(Clone, Parser)]
pub(crate) struct Args {
	/// Give the bookmark file you want used
	pub(crate) bookmarks_file: Option<PathBuf>,

	/// Print version and exit
	#[arg(short = 'V', long, action = ArgAction::SetTrue)]
	pub(crate) version: bool,

	/// Command to run
	#[command(subcommand)]
	pub(crate) command: Option<Command>,
}
