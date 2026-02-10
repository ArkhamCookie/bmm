use std::fs::{self, File};
use std::io::Write;
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

	pub fn add(&mut self, new_bookmark: Bookmark) -> Result<&mut Self, String> {
		self.bookmarks.push(new_bookmark);

		let mut contents = String::new();

		for bookmark in &self.bookmarks {
			contents.push_str("[[bookmarks]]\n");
			contents.push_str("link = \"");
			contents.push_str(&bookmark.link);
			contents.push_str("\"\n");

			if bookmark.name.is_some() {
				contents.push_str("name = \"");
				contents.push_str(bookmark.name.clone().unwrap().as_str());
				contents.push_str("\"\n");
			}

			if bookmark.description.is_some() {
				contents.push_str("description = \"");
				contents.push_str(bookmark.description.clone().unwrap().as_str());
				contents.push_str("\"\n");
			}

			contents.push('\n');
		}

		let mut file = File::create(&self.file).expect("couldn't get file");

		let write_result = file.write(contents.as_bytes());

		if write_result.is_err() {
			let error = write_result.unwrap_err();

			eprintln!("ERROR: {}", error);
			return Err(String::from("write error"));
		}

		Ok(self)
	}
}

#[cfg(test)]
mod tests {}
