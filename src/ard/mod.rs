pub mod writer;
pub mod reader;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::fs::MetadataExt;
use std::path::Path;

use crate::epoch::Epoch;

fn last_line_nocheck<P: AsRef<Path>>(path: P) -> Option<String> {
	let reader = BufReader::new(File::open(path).unwrap());
	reader.lines().last().map(|line| line.unwrap())
}

pub struct Ard {
	rotlen: usize,
	file_string: String,
	next_suffix: usize,
}

impl Ard {
	pub fn new(file_str: &str) -> Self {
		let file_string = file_str.to_string();

		// create empty file
		let dst_path = Path::new(file_str);
		if !dst_path.exists() {
			File::create(dst_path).unwrap();
		}
	
		// check and create ard
		let ard_string = format!("{}.ard", file_str);
		let ard_path = Path::new(&ard_string);
		if !ard_path.exists() {
			std::fs::create_dir(ard_path).unwrap();
		}
	
		// get art max suffix
		let mut next_suffix = 0;
		for art_suffix in 0.. {
			let art_string = format!("{}.ard/{}.art", file_str, art_suffix);
			let art_path = Path::new(&art_string);
			if !art_path.exists() {
				next_suffix = art_suffix;
				break
			}
		}
		Self {
			rotlen: 10000,
			file_string,
			next_suffix,
		}
	}

	pub fn last_line(&self) -> Option<(Epoch, String)> {
		// read last line
		let last_line: String = if std::fs::metadata(&self.file_string)
			.unwrap()
			.size() != 0
		{
			last_line_nocheck(&self.file_string).unwrap()
		} else if self.next_suffix > 0 {
			let art_string = format!("{}.ard/{}.art", self.file_string, self.next_suffix - 1);
			last_line_nocheck(&art_string).unwrap()
		} else {
			return None
		};
		let last_epoch = Epoch::from_line(&last_line);
		Some((last_epoch, last_line))
	}
}
