use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Bookmark {
	pub link: String,
	pub name: Option<String>,
	pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub bookmarks: Vec<Bookmark>,
}

impl Config {
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
