use ebra::ard::{Ard, reader::ArdReader};

fn main() {
	let aarg = aarg::parse().unwrap();
	let dst = &aarg.get("").unwrap()[1];
	let ard = Ard::new(dst);
	let mut reader = ArdReader::new(ard);
	while let Some(line) = reader.read_line() {
		println!("{}", line);
	}
}