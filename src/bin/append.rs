use ebra::epoch::Epoch;
use ebra::ard::{Ard, writer::ArdWriter};

use std::io::BufRead;

fn main() {
	let aarg = aarg::parse().unwrap();
	let dst = &aarg.get("").unwrap()[1];
	let ard = Ard::new(dst);
	let (last_epoch, last_line) = if let Some(x) = ard.last_line() {
		x
	} else {
		(Epoch::default(), String::new())
	};
	let mut writer = ArdWriter::new(ard);

	let stdin = std::io::stdin();
	let mut point_found = false;
	let mut buffer: Vec<String> = Vec::new();
	let mut count = 0;
	for line in stdin.lock().lines() {
		let line = line.unwrap();
		let epoch = Epoch::from_line(&line);
		if epoch < last_epoch { continue }
		if point_found {
			writer.write_line(&line);
			count += 1;
			continue
		}
		if epoch == last_epoch {
			if line == last_line {
				point_found = true;
				eprintln!("Point found: {}", &line);
			} else {
				buffer.push(line.to_string());
			}
			continue
		}
		if epoch > last_epoch {
			eprintln!("Point not found: {}", &line);
			for line in buffer.iter() {
				writer.write_line(&line);
			}
			point_found = true;
		}
	}
	eprintln!("Wrote {} lines", count);
}