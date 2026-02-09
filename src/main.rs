use std::process::exit;

use crate::cli::{Args, Command};
use crate::conifg::FileConfig;
use crate::tui::app::interactive;

use clap::{Parser, crate_authors, crate_description, crate_name, crate_version};

use colored::Colorize;

use terminal_link::Link;

mod cli;
mod conifg;
mod tui;

fn main() {
	let args = Args::parse();

	if args.version {
		println!("{}: v{}", crate_name!(), crate_version!());
		println!("{}", crate_authors!());
		println!("\n{}", crate_description!());
		exit(0)
	}

	if args.bookmarks_file.is_none() {
		let colored_error = String::from("error:").red().bold();
		let colored_missing = String::from("<BOOKMARKS_FILE>").green();
		let error_message = format!(
			"{} {} is a required argument",
			colored_error, colored_missing
		);

		eprintln!("{}", error_message);
		exit(1)
	}

	match &args.command {
		Some(Command::Add {
			link,
			name,
			description,
		}) => {
			println!("{:?}", link);
			println!("{:?}", name);
			println!("{:?}", description);
		}
		Some(Command::Rm { bookmark }) => {
			println!("{:?}", bookmark);
		}
		Some(Command::List {}) => {
			let file_config = FileConfig::get(&args.bookmarks_file.expect("couldn't find bookmark file")).expect("couldn't get FileConfig"); // TODO: Better error handling

			for bookmark in file_config.bookmarks {
				if bookmark.name.is_some() {
					let name = bookmark.name.unwrap();
					let link = Link::new(&name, &bookmark.link);

					print!("{}", link);
				} else {
					let link = Link::new(&bookmark.link, &bookmark.link);

					print!("{}", link);
				}

				if bookmark.description.is_some() {
					println!(" — {}", bookmark.description.unwrap());
				} else {
					println!("");
				}
			}

			exit(0);
		}
		Some(Command::View { bookmark }) => {
			println!("{:?}", bookmark);
		}
		None => {
			interactive(&args).expect("calling interactive mode for tui failed");
			exit(0);
		}
	}
}
