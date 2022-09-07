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
		let mut has_sec = false;
		let has_frac = loop {
			let byte = if let Some(byte) = iter.next() {
				byte
			} else {
				break false
			};
			if (b'0'..=b'9').contains(&byte) {
				s *= 10;
				has_sec = true;
				s += (byte - b'0') as u64;
			} else {
				break byte == b'.'
			}
		};
		assert!(has_sec || has_frac);

		if has_frac {
			let mut count = 0;
			loop {
				let byte = if let Some(byte) = iter.next() {
					byte
				} else {
					break
				};
				eprintln!("{} {}", count, ns);
				if (b'0'..=b'9').contains(&byte) {
					if count >= 9 {
						continue
					}
					count += 1;
					ns *= 10;
					ns += (byte - b'0') as u32;
				} else {
					break
				}
			}
			for _ in count..9 {
				ns *= 10;
			}
		}

		Epoch { s, ns }
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_parse_epoch() {
		let e = Epoch::from_line("999foo");
		assert_eq!(e.s, 999);
		assert_eq!(e.ns, 0);

		let e = Epoch::from_line("123123.123123 foo");
		assert_eq!(e.s, 123123);
		assert_eq!(e.ns, 123123000);

		let e = Epoch::from_line("123123.123123123123 foo");
		assert_eq!(e.s, 123123);
		assert_eq!(e.ns, 123123123);

		let e = Epoch::from_line("0.1");
		assert_eq!(e.s, 0);
		assert_eq!(e.ns, 100000000);

		let e = Epoch::from_line("0.");
		assert_eq!(e.s, 0);
		assert_eq!(e.ns, 0);

		let e = Epoch::from_line(".");
		assert_eq!(e.s, 0);
		assert_eq!(e.ns, 0);

		let e = Epoch::from_line("...");
		assert_eq!(e.s, 0);
		assert_eq!(e.ns, 0);

		let e = Epoch::from_line(".001");
		assert_eq!(e.s, 0);
		assert_eq!(e.ns, 1000000);
	}
}
