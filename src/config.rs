use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml;

/// A bookmark
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Bookmark {
	/// Bookmark link
	pub link: String,
	/// Bookmark name
	pub name: Option<String>,
	/// Bookmark description
	pub description: Option<String>,
}

/// `bmm` bookmark file struct
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Config {
	/// Bookmarks in bookmarks file
	pub bookmarks: Vec<Bookmark>,
}

/// A combination of the Config and the file to read & write to
#[derive(Debug)]
pub struct FileConfig {
	pub bookmarks: Vec<Bookmark>,
	pub file: PathBuf,
}

impl FileConfig {
	pub fn get(path: &PathBuf) -> Result<Self, String> {
		let file = match fs::read_to_string(path) {
			Ok(string) => string,
			Err(error) => {
				eprintln!("ERROR: {}", error);
				todo!()
			},
		};

		let toml: Config = toml::from_str(&file).unwrap(); // TODO: Better error handling

		Ok(Self {
			bookmarks: toml.bookmarks,
			file: path.clone(),
		})
	}
}

#[cfg(test)]
mod tests {}
