#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Epoch {
	pub s: u64,
	pub ns: u32, // 0 - 999,999,999
}

impl Epoch {
	pub fn from_line(line: &str) -> Epoch {
		let mut s: u64 = 0;
		let mut ns: u32 = 0;
		let mut iter = line.bytes();
		let has_frac = loop {
			let byte = if let Some(byte) = iter.next() {
				byte
			} else {
				break false
			};
			if (b'0'..=b'9').contains(&byte) {
				s *= 10;
				s += (byte - b'0') as u64;
			} else {
				break byte == b'.'
			}
		};
		assert!(s > 0);

		if has_frac {
			loop {
				let byte = if let Some(byte) = iter.next() {
					byte
				} else {
					break
				};
				if (b'0'..=b'9').contains(&byte) {
					ns *= 10;
					ns += (byte - b'0') as u32;
				} else {
					break
				}
			}
		}

		Epoch { s, ns }
	}
}
