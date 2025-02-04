use crate::Derive;


#[derive(Debug, Clone)]
pub struct DerivesMask {
	inner: Vec<bool>,
}

impl DerivesMask {
	pub fn push(&mut self, value: bool) { self.inner.push(value) }


	pub fn from_ascii(mask: &[u8]) -> Result<Self, ParseMaskError> {
		let mut values = vec![false; mask.len()];
		for (i, v) in mask.into_iter().enumerate() {
			match v {
				b'0' => { /* already false-filled */ },
				b'1' => values[i] = true,
				_ => return Err(ParseMaskError),
			}
		}
		Ok(Self { inner: values })
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
		Self { inner: vec![
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
		let iter = self.inner.iter().map(|v| if *v { "1" } else { "0" });
		write!(f, "{}", iter.collect::<String>())
	}
}


impl PartialEq for DerivesMask {
	fn eq(&self, other: &Self) -> bool { self.inner == other.inner }
}

impl PartialOrd for DerivesMask {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		if other.inner.len() > self.inner.len() && other.inner[self.inner.len()..].contains(&true) {
			return Some(std::cmp::Ordering::Less);
		}

		let len = self.inner.len().min(other.inner.len());
		let a = &self.inner[..len];
		let b = &other.inner[..len];

		let res = if a == b {
			if self.inner.len() > other.inner.len() && self.inner[other.inner.len()..].contains(&true) {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		} else if b.into_iter()
		           .enumerate()
		           .filter(|(_, v)| **v)
		           .all(|(i, v)| &a[i] == v)
		{
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

	/// Calc distance between two masks.
	///
	/// __Non-commutative function.__
	///
	/// If distance between `a` & `b` is gt then `0`, that means `a` doesn't covers `b`.
	///
	/// e.g. if `a > b` => `d < 0`, so
	/// - `100 d 000 = -1`
	/// - `100 d 111 = 2`
	/// - `100 d 1111 = >2`
	/// - `1001 d 111 = >2`
	pub fn distance(&self, other: &Self) -> isize {
		if self == other {
			0
		} else {
			let a = self.inner.as_slice();
			let b = other.inner.as_slice();

			let prefix = {
				let len = self.inner.len().min(other.inner.len());
				let a = &self.inner[..len];
				let b = &other.inner[..len];
				a.into_iter().zip(b.into_iter()).fold(0, |acc, (a, b)| {
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
			// Otherwise it will be like that:
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
	fn eq() {
		let mut a = DerivesMask::default();
		let mut b = DerivesMask::default();
		assert_eq!(a, b);

		a.inner[0] = true;
		assert_ne!(a, b);

		b.inner[0] = true;
		assert_eq!(a, b);

		let last = b.inner.len() - 1;
		b.inner[last] = true;
		assert_ne!(a, b);
	}

	#[test]
	fn ord() {
		let mut a = DerivesMask::default();
		let mut b = DerivesMask::default();
		assert!(a == b, "{a} == {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(a <= b, "{a} <= {b}");

		a.inner[0] = true;
		assert!(a != b, "{a} != {b}");
		assert!(a > b, "{a} > {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(!(a <= b), "{a} <= {b}");
		assert!(b <= a, "{b} <= {a}");
		assert!(b < a, "{b} < {a}");

		b.inner[0] = true;
		assert!(a == b, "{a} == {b}");
		assert!(a >= b, "{a} >= {b}");
		assert!(a <= b, "{a} <= {b}");

		let last = b.inner.len() - 1;
		b.inner[last] = true;
		assert!(a != b, "{a} != {b}");
		assert!(a < b, "{a} < {b}");
		assert!(a <= b, "{a} <= {b}");

		a.inner.fill(true);
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

		a.inner[0] = true;
		assert_eq!(-1, a.distance(&b));

		b.inner[0] = true;
		assert_eq!(0, a.distance(&b));

		let last = b.inner.len() - 1;
		b.inner[last] = true;
		assert_eq!(1, a.distance(&b));
		assert_eq!(-1, b.distance(&a));

		b.push(true);
		assert_eq!(TAIL + 1, a.distance(&b));

		b.inner[0] = false;
		assert_eq!(TAIL, a.distance(&b));

		a.inner[last] = true;
		assert_eq!(TAIL - 1, a.distance(&b));
		a.inner[1] = true;
		assert_eq!(TAIL - 2, a.distance(&b));


		// 1001 d 111:
		let a = DerivesMask { inner: vec![true, false, false, true] };
		let b = DerivesMask { inner: vec![true, true, true] };
		assert_eq!(2, a.distance(&b));
		assert_eq!(TAIL - 2, b.distance(&a));
	}
}
