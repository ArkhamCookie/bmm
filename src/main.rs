use crate::cli::Args;
use crate::conifg::Config;

use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};

mod cli;
mod conifg;

fn main() {
	let args = Args::parse();

	if args.version {
		println!("{}: v{}", crate_name!(), crate_version!());
		println!("{}", crate_authors!());
		println!("\n{}", crate_description!());
	}

	let config = Config::get(&args.bookmarks_file).unwrap(); // TODO: Handle unwrap better

	println!("{:?}", config);
}
