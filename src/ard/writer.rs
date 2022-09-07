use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

use super::Ard;

fn count_line_nocheck<P: AsRef<Path>>(path: P) -> usize {
	let reader = BufReader::new(File::open(path).unwrap());
	reader.lines().count()
}

// TODO: lock the target file?
pub struct ArdWriter {
	ard: Ard,
	rotlen: usize,
	line_count: usize,
	writer: BufWriter<File>,
}

impl ArdWriter {
	pub fn new(ard: Ard, rotlen: usize) -> Self {
		let line_count = count_line_nocheck(&ard.file_string);
		assert!(line_count < rotlen);
		let file = OpenOptions::new()
			.append(true)
			.open(&ard.file_string)
			.unwrap();
		let writer = BufWriter::new(file);
		Self {
			ard,
			rotlen,
			line_count,
			writer,
		}
	}

	pub fn write_line(&mut self, line: &str) {
		writeln!(&mut self.writer, "{}", line).unwrap();
		self.line_count += 1;
		if self.line_count == self.rotlen {
			std::fs::rename(
				&self.ard.file_string,
				format!("{}.ard/{}.art", self.ard.file_string, self.ard.next_suffix),
			).unwrap();
			self.ard.next_suffix += 1;
			let file = OpenOptions::new()
				.create_new(true)
				.write(true)
				.open(&self.ard.file_string)
				.unwrap();
			self.writer = BufWriter::new(file);
			self.line_count = 0;
		}
	}

	pub fn unwrap(self) -> Ard {
		self.ard
	}
}
