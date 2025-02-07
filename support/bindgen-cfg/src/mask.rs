use crate::Derive;


#[derive(Debug, Clone)]
pub struct DerivesMask {
	values: Vec<bool>,
}

impl DerivesMask {
	pub fn push(&mut self, value: bool) { self.values.push(value) }


	pub fn from_ascii(mask: &[u8]) -> Result<Self, ParseMaskError> {
		let mut values = vec![false; mask.len()];
		for (i, v) in mask.iter().enumerate() {
			match v {
				b'0' => { /* already false-filled */ },
				b'1' => values[i] = true,
				_ => return Err(ParseMaskError),
			}
		}
		Ok(Self { values })
	}

	pub fn from_str(mask: &str) -> Result<Self, ParseMaskError> {
		if mask.is_ascii() {
			Self::from_ascii(mask.as_bytes())
		} else {
			Err(ParseMaskError)
		}
	}
}


impl Default for DerivesMask {
	fn default() -> Self { Self::from(Derive::empty()) }
}

impl From<Derive> for DerivesMask {
	fn from(values: Derive) -> Self {
		// Caution: do not change the order of items!
		Self { values: vec![
		                    values.default,
		                    values.eq,
		                    values.copy,
		                    values.debug,
		                    values.hash,
		                    values.ord,
		                    values.partialeq,
		                    values.partialord,
		                    values.constparamty,
		] }
	}
}

impl From<&'_ Derive> for DerivesMask {
	fn from(value: &'_ Derive) -> Self { (*value).into() }
}

impl std::fmt::Display for DerivesMask {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let iter = self.values.iter().map(|v| if *v { "1" } else { "0" });
		write!(f, "{}", iter.collect::<String>())
	}
}


impl PartialEq for DerivesMask {
	fn eq(&self, other: &Self) -> bool { self.values == other.values }
}

impl PartialOrd for DerivesMask {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if other.values.len() > self.values.len() && other.values[self.values.len()..].contains(&true) {
			return Some(std::cmp::Ordering::Less);
		}

		let len = self.values.len().min(other.values.len());
		let a = &self.values[..len];
		let b = &other.values[..len];

		let res = if a == b {
			if self.values.len() > other.values.len() && self.values[other.values.len()..].contains(&true) {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		} else if b.iter().enumerate().filter(|(_, v)| **v).all(|(i, v)| &a[i] == v) {
			std::cmp::Ordering::Greater
		} else {
			std::cmp::Ordering::Less
		};

		Some(res)
	}
}


impl DerivesMask {
	/// The cost of positive remainder of `other`.
	const REST_DISTANCE: isize = 100;

	/// Calc "distance" between two masks.
	///
	/// __Non-commutative function.__
	///
	/// Here the name "distance" means mesure of "how enough `a` (not) covers `b`".
	///
	/// Result is signed, so if "`a` covers `b`" result is `0 - n` where `n = a - b`.
	/// If distance between `a` & `b` is gt then `0`, that means `a` doesn't covers `b`, so if `a > b` => `d < 0`.
	///
	/// Actually it's almost same as `xor` (`a ^ b`),
	/// e.g. `0b1000 ^ 0b1010 = 0b10` and the "distance" is `1` which means
	/// "`b` has one feature uncovered by `a`".
	///
	///
	/// - `100 d 000 = -1`
	/// - `100 d 111 = 2`
	/// - `100 d 1111 = >2`
	/// - `1001 d 111 = 2`
	///
	/// Various len:
	/// - if `a = 10010` and `b = 100`, the remainder of `a` (10) is not contributes to the distance because b doesn't extend that far
	///   and "`a` covers `b`",
	/// - if `a = 100` and `b = 10010`, the remainder of `b` (10) __is__ contributes to the distance significantly.
	pub fn distance(&self, other: &Self) -> isize {
		if self == other {
			0
		} else {
			let a = self.values.as_slice();
			let b = other.values.as_slice();

			let prefix = {
				let len = self.values.len().min(other.values.len());
				let a = &self.values[..len];
				let b = &other.values[..len];
				a.iter().zip(b).fold(0, |acc, (a, b)| {
					               acc +
					               match (a, b) {
						               (true, false) => -1,
					                  (false, true) => 1,
					                  _ => 0,
					               }
				               })
			};

			prefix +
			if b.len() > a.len() && b[a.len()..].contains(&true) {
				Self::REST_DISTANCE // cost of positive remainder of `b`
			}
			// We do not take into account the remainder of `a` because we do not consider the net (real) distance,
			// but the difference, meaning “by how much `a` covers `b`”.
			// Otherwise it could be like that:
			// else if a[b.len()..].contains(&true) { -Self::REST_DISTANCE }
			else {
				0
			}
		}
	}
}


#[derive(Debug)]
pub struct ParseMaskError;
impl std::error::Error for ParseMaskError {}
impl std::fmt::Display for ParseMaskError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "ParseMaskError") }
}


#[cfg(test)]
mod tests {
	use super::*;


	#[test]
	fn fmt() {
		let mut mask = DerivesMask::default();
		assert!(!mask.to_string().contains('1'));

		mask.values[0] = true;
		assert_eq!("100000000", &mask.to_string());

		mask.values.fill(true);
		assert_eq!("111111111", &mask.to_string());
	}


	#[test]
	fn err() {
		assert!(DerivesMask::from_str("123456789").is_err());
		assert!(DerivesMask::from_str("-").is_err());
		assert!(DerivesMask::from_str("xyz").is_err());
	}

	#[test]
	fn from_str() {
		let empty = DerivesMask::default();
		let full = DerivesMask::from_str("111111111").unwrap();
		assert_ne!(empty, full);
		assert_eq!(empty, DerivesMask::from_str("000000000").unwrap());
		assert!(DerivesMask::from_str("001000000").unwrap().values[2]);

		assert!(DerivesMask::from_str("").unwrap().values.is_empty());
		assert_eq!(3, DerivesMask::from_str("111").unwrap().values.len());
	}

	#[test]
	fn from_ascii() {
		let empty = DerivesMask::default();
		let full = DerivesMask::from_ascii(b"111111111").unwrap();
		assert_ne!(empty, full);
		assert_eq!(empty, DerivesMask::from_ascii(b"000000000").unwrap());
		assert!(DerivesMask::from_ascii(b"001000000").unwrap().values[2]);

		assert!(DerivesMask::from_ascii(b"").unwrap().values.is_empty());
		assert_eq!(3, DerivesMask::from_ascii(b"111").unwrap().values.len());
	}


	#[test]
	fn eq() {
		let mut a = DerivesMask::default();
		let mut b = DerivesMask::default();
		assert_eq!(a, b);

		a.values[0] = true;
		assert_ne!(a, b);

		b.values[0] = true;
		assert_eq!(a, b);

		let last = b.values.len() - 1;
		b.values[last] = true;
		assert_ne!(a, b);
	}

	#[test]
	fn ord() {
		let mut a = DerivesMask::default();
		let mut b = DerivesMask::default();
		assert!(a == b, "{a} == {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(a <= b, "{a} <= {b}");

		a.values[0] = true;
		assert!(a != b, "{a} != {b}");
		assert!(a > b, "{a} > {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(!(a <= b), "{a} <= {b}");
		assert!(b <= a, "{b} <= {a}");
		assert!(b < a, "{b} < {a}");

		b.values[0] = true;
		assert!(a == b, "{a} == {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(a <= b, "{a} <= {b}");

		let last = b.values.len() - 1;
		b.values[last] = true;
		assert!(a != b, "{a} != {b}");
		assert!(a < b, "{a} < {b}");
		assert!(a <= b, "{a} <= {b}");

		a.values.fill(true);
		assert!(a != b, "{a} != {b}");
		assert!(a > b, "{a} > {b}");
		assert!(a >= b, "{a} >= {b}");
	}

	#[test]
	fn rest() {
		let mut a = DerivesMask::default();
		let mut b = DerivesMask::default();
		b.push(true);
		assert!(!(a == b), "{a} != {b}");
		assert!(!(a >= b), "{a} <= {b}");
		assert!(a <= b, "{a} <= {b}");
		assert!(a < b, "{a} < {b}");

		a.push(true);
		a.push(true);
		assert!(a != b, "{a} != {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(a > b, "{a} > {b}");

		b.push(true);
		assert!(a == b, "{a} == {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(a <= b, "{a} <= {b}");
	}


	#[test]
	fn distance() {
		const TAIL: isize = DerivesMask::REST_DISTANCE;

		let mut a = DerivesMask::default();
		let mut b = DerivesMask::default();
		assert_eq!(0, a.distance(&b));
		assert_eq!(0, b.distance(&a));

		a.values[0] = true;
		assert_eq!(-1, a.distance(&b));

		b.values[0] = true;
		assert_eq!(0, a.distance(&b));

		let last = b.values.len() - 1;
		b.values[last] = true;
		assert_eq!(1, a.distance(&b));
		assert_eq!(-1, b.distance(&a));

		b.push(true);
		assert_eq!(TAIL + 1, a.distance(&b));

		b.values[0] = false;
		assert_eq!(TAIL, a.distance(&b));

		a.values[last] = true;
		assert_eq!(TAIL - 1, a.distance(&b));
		a.values[1] = true;
		assert_eq!(TAIL - 2, a.distance(&b));


		// 1001 d 111:
		let a = DerivesMask { values: vec![true, false, false, true] };
		let b = DerivesMask { values: vec![true, true, true] };
		assert_eq!(2, a.distance(&b));
		assert_eq!(TAIL - 2, b.distance(&a));
	}
}
