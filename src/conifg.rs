use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml;

/// A bookmark
#[derive(Debug, Deserialize, PartialEq, Serialize)]
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

impl Config {
	/// Get config from a file
	pub fn get(path: &PathBuf) -> Result<Self, String> {
		let file = match fs::read_to_string(path) {
			Ok(string) => string,
			Err(error) => {
				eprintln!("ERROR: {}", error);
				todo!()
			}
		};

		match toml::from_str(&file) {
			Ok(config) => Ok(config),
			Err(error) => {
				eprintln!("ERROR: {}", error);
				todo!()
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use crate::conifg::{Bookmark, Config};

	/// Test getting a single bookmark from a bookmark file
	#[test]
	fn basic_bookmark_from_file() {
		let wanted_bookmark = Bookmark {
			link: String::from("https://arkhamcookie.com"),
			name: Some(String::from("ArkhamCookie")),
			description: Some(String::from("ArkhamCookie's Website")),
		};
		let wanted_bookmarks = vec![wanted_bookmark];
		let wanted_config = Config {
			bookmarks: wanted_bookmarks,
		};

		let config = Config::get(&PathBuf::from("./tests/data/basic_bookmark.toml")).unwrap();

		assert_eq!(wanted_config, config);
	}

	/// Test getting bookmarks from a bookmark file with different Options
	#[test]
	fn basic_bookmarks_from_file() {
		let wanted_bookmark_1 = Bookmark {
			link: String::from("https://arkhamcookie.com"),
			name: Some(String::from("ArkhamCookie")),
			description: Some(String::from("ArkhamCookie's Website")),
		};
		let wanted_bookmark_2 = Bookmark {
			link: String::from("https://github.com"),
			name: Some(String::from("GitHub")),
			description: None,
		};
		let wanted_bookmark_3 = Bookmark {
			link: String::from("https://youtube.com"),
			name: None,
			description: None,
		};
		let wanted_bookmarks = vec![wanted_bookmark_1, wanted_bookmark_2, wanted_bookmark_3];
		let wanted_config = Config {
			bookmarks: wanted_bookmarks,
		};

		let config = Config::get(&PathBuf::from("./tests/data/basic_bookmarks.toml")).unwrap();

		assert_eq!(wanted_config, config)
	}
}
