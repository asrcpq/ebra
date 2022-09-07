use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

use super::Ard;

pub struct ArdReader {
	ard: Ard,
	lines: Lines<BufReader<File>>,
	// if n == ard.next_suffix, next file is dst
	current_file: usize,
}

impl ArdReader {
	pub fn new(ard: Ard) -> Self {
		let (file, current_file) = if ard.next_suffix == 0 {
			(
				File::open(&ard.file_string).unwrap(),
				ard.next_suffix,
			)
		} else {
			let art0_string = format!("{}.ard/0.art", ard.file_string);
			(
				File::open(&art0_string).unwrap(),
				0,
			)
		};
		let lines = BufReader::new(file).lines();
		Self {
			ard,
			lines,
			current_file,
		}
	}

	pub fn read_line(&mut self) -> Option<String> {
		loop {
			let line = self.lines.next();
			if line.is_none() {
				self.current_file += 1;
				if self.current_file > self.ard.next_suffix {
					return None
				} else if self.current_file == self.ard.next_suffix {
					// reading dst
					let file = File::open(&self.ard.file_string).unwrap();
					self.lines = BufReader::new(file).lines();
				} else {
					let art_string = format!(
						"{}.ard/{}.art",
						self.ard.file_string,
						self.current_file,
					);
					let file = File::open(&art_string).unwrap();
					self.lines = BufReader::new(file).lines();
				}
			} else {
				return line.map(|x| x.unwrap())
			}
		}
	}

	pub fn unwrap(self) -> Ard {
		self.ard
	}
}
