#![allow(non_upper_case_globals)]
use core::ops::*;
use sys::ffi;


#[const_trait]
pub trait Buttons: ButtonsExt + BitOr + BitAnd + BitOrAssign + BitAndAssign {
	const All: Self;

	fn display(&self) -> impl core::fmt::Display + core::fmt::Debug;
}


#[const_trait]
pub trait ButtonsExt: Sized {
	fn raw(&self) -> u8;


	/// Semantically same as `PartialEq`.
	///
	/// Returns `true` if `self` contains __any__ of `other`.
	#[inline(always)]
	fn intersects(&self, other: &Self) -> bool { self.raw() == other.raw() || (self.raw() & other.raw()) != 0 }

	/// Same as [`contains_all`][Self::contains_all], but for single button.
	#[inline]
	fn contains(&self, other: &Button) -> bool { self.raw() & other.to_buttons().raw() != 0 }

	/// Semantically same as `Eq`.
	///
	/// Returns `true` if `self` contains __all__ of `other`.
	///
	/// See also [`contains`][Self::contains] for single button.
	#[inline(always)]
	fn contains_all(&self, other: &Self) -> bool {
		self.raw() == other.raw() || (self.raw() & other.raw()) == other.raw()
	}


	#[inline(always)]
	fn any(&self) -> bool { self.raw() != 0 }

	#[inline(always)]
	fn is_empty(&self) -> bool { self.raw() == 0 }


	/// Contains [`Left`][Self::Left] button.
	#[inline(always)]
	fn left(&self) -> bool { self.contains(&Button::Left) }

	/// Contains [`Right`][Self::Right] button.
	#[inline(always)]
	fn right(&self) -> bool { self.contains(&Button::Right) }

	/// Contains [`Up`][Self::Up] button.
	#[inline(always)]
	fn up(&self) -> bool { self.contains(&Button::Up) }

	/// Contains [`Down`][Self::Down] button.
	#[inline(always)]
	fn down(&self) -> bool { self.contains(&Button::Down) }

	/// Contains [`A`][Self::A] button.
	#[inline(always)]
	fn a(&self) -> bool { self.contains(&Button::A) }

	/// Contains [`B`][Self::B] button.
	#[inline(always)]
	fn b(&self) -> bool { self.contains(&Button::B) }


	const A: Self;
	const B: Self;
	const Left: Self;
	const Right: Self;
	const Up: Self;
	const Down: Self;

	const Menu: Self;
}


pub trait ButtonsIntoIter: ButtonsExt {
	const ALL: [Self; 6];

	fn into_iter(self) -> impl Iterator<Item = Self> {
		Self::ALL.into_iter()
		         .filter(move |possible| (self.raw() & possible.raw()) != 0)
	}

	fn into_iter_btns(self) -> impl Iterator<Item = ffi::Buttons>;

	fn into_iter_flatten(self) -> impl Iterator<Item = Button> {
		Button::ALL.into_iter().filter(move |btn| self.contains(btn))
	}
}
impl ButtonsIntoIter for ffi::Buttons {
	/// All buttons sorted.
	const ALL: [Self; 6] = [Self::Left, Self::Right, Self::Up, Self::Down, Self::B, Self::A];

	#[inline(always)]
	fn into_iter_btns(self) -> impl Iterator<Item = ffi::Buttons> { self.into_iter() }
}


impl const ButtonsExt for ffi::Buttons {
	const A: Self = ffi::Buttons::A;
	const B: Self = ffi::Buttons::B;
	const Left: Self = ffi::Buttons::Left;
	const Right: Self = ffi::Buttons::Right;
	const Up: Self = ffi::Buttons::Up;
	const Down: Self = ffi::Buttons::Down;
	const Menu: Self = ffi::Buttons(ffi::Buttons::A.0 << 1); // 01000000

	fn raw(&self) -> u8 { self.0 as _ }
}

impl const Buttons for ffi::Buttons {
	const All: Self = ffi::Buttons(
	                               ffi::Buttons::A.0 |
	                               ffi::Buttons::B.0 |
	                               ffi::Buttons::Left.0 |
	                               ffi::Buttons::Right.0 |
	                               ffi::Buttons::Up.0 |
	                               ffi::Buttons::Down.0,
	);

	fn display(&self) -> impl core::fmt::Display + core::fmt::Debug { fmt::Display(*self) }
}


/// Represents single button of [`Buttons`][ffi::Buttons].
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Button {
	Left,
	Right,
	Up,
	Down,
	B,
	A,
	Menu,
}

impl Button {
	pub const fn first_from(btns: ffi::Buttons) -> Option<Self> {
		match btns {
			ffi::Buttons::A => Some(Self::A),
			ffi::Buttons::B => Some(Self::B),
			ffi::Buttons::Left => Some(Self::Left),
			ffi::Buttons::Right => Some(Self::Right),
			ffi::Buttons::Up => Some(Self::Up),
			ffi::Buttons::Down => Some(Self::Down),
			ffi::Buttons::Menu => Some(Self::Menu),
			_ => None,
		}
	}
}

impl const ButtonsExt for Button {
	const A: Self = Button::A;
	const B: Self = Button::B;
	const Left: Self = Button::Left;
	const Right: Self = Button::Right;
	const Up: Self = Button::Up;
	const Down: Self = Button::Down;
	const Menu: Self = Button::Menu;

	fn raw(&self) -> u8 { self.to_buttons().0 as u8 }
}

impl ButtonsIntoIter for Button {
	const ALL: [Self; 6] = [Self::Left, Self::Right, Self::Up, Self::Down, Self::B, Self::A];

	#[inline(always)]
	fn into_iter(self) -> impl Iterator<Item = Self> { [self].into_iter() }

	#[inline(always)]
	fn into_iter_flatten(self) -> impl Iterator<Item = Button> { [self].into_iter() }

	#[inline(always)]
	fn into_iter_btns(self) -> impl Iterator<Item = ffi::Buttons> { self.into_iter().map(|b| b.to_buttons()) }
}


impl Button {
	pub const fn to_buttons(&self) -> ffi::Buttons {
		match self {
			Button::Left => ffi::Buttons::Left,
			Button::Right => ffi::Buttons::Right,
			Button::Up => ffi::Buttons::Up,
			Button::Down => ffi::Buttons::Down,
			Button::B => ffi::Buttons::B,
			Button::A => ffi::Buttons::A,
			Button::Menu => ffi::Buttons::Menu,
		}
	}

	pub const fn is_in(&self, other: &ffi::Buttons) -> bool {
		match self {
			Button::Left => other.intersects(&ffi::Buttons::Left),
			Button::Right => other.intersects(&ffi::Buttons::Right),
			Button::Up => other.intersects(&ffi::Buttons::Up),
			Button::Down => other.intersects(&ffi::Buttons::Down),
			Button::B => other.intersects(&ffi::Buttons::B),
			Button::A => other.intersects(&ffi::Buttons::A),
			Button::Menu => other.intersects(&ffi::Buttons::Menu),
		}
	}


	const ALL: [Button; 6] = [
	                          Button::Left,
	                          Button::Right,
	                          Button::Up,
	                          Button::Down,
	                          Button::B,
	                          Button::A,
	];
}

impl From<Button> for ffi::Buttons {
	fn from(btn: Button) -> Self { btn.to_buttons() }
}


pub trait IterButtons {
	/// Map each [`Btns`] to sequence of [`Btn`]s.
	fn flat_btn(self) -> impl Iterator<Item = Button>;
}

impl<T: Iterator<Item = ffi::Buttons>> IterButtons for T {
	fn flat_btn(self) -> impl Iterator<Item = Button> {
		self.flat_map(|btns| Button::ALL.into_iter().filter(move |btn| btn.is_in(&btns)))
	}
}


mod fmt {
	use super::*;
	use core::fmt;


	#[must_use]
	#[repr(transparent)]
	#[derive(Clone, Copy, PartialEq, Eq, Hash)]
	pub struct Display(pub(crate) ffi::Buttons);


	impl fmt::Debug for Display {
		fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
			let mut fmt = f.debug_tuple(&format!("Buttons[{:08b}]", self.0.0));
			let iter = self.0.into_iter_flatten();
			for item in iter {
				fmt.field(&item);
			}
			fmt.finish()
		}
	}

	impl fmt::Display for Display {
		fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
			write!(f, "(")?;
			let mut iter = self.0.into_iter_flatten();
			if let Some(first) = iter.next() {
				write!(f, "{first:?}")?;
			}
			for item in iter {
				write!(f, ", {:?}", item)?;
			}
			write!(f, ")")
		}
	}


	#[cfg(test)]
	mod tests {
		use super::*;
		use sys::ffi::Buttons as Btns;


		#[test]
		fn debug() {
			let btns = {
				let all = (Btns::All, "Buttons[00111111](Left, Right, Up, Down, B, A)");
				let ab = (Btns::A | Btns::B, "Buttons[00110000](B, A)");
				let empty = (Btns(0), "Buttons[00000000]");
				[all, ab, empty]
			};

			for (btns, expected) in btns {
				let btns = Display(btns);
				assert_eq!(expected, format!("{btns:?}"));
			}
		}

		#[test]
		fn display() {
			let btns = [
			            (Btns::All, "(Left, Right, Up, Down, B, A)"),
			            (Btns::A | Btns::B, "(B, A)"),
			            (Btns(0), "()"),
			];

			for (btns, expected) in btns {
				let btns = Display(btns);
				assert_eq!(expected, format!("{btns}"));
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use sys::ffi::Buttons as Btns;


	#[test]
	fn contains() {
		let all = Btns::All;
		let ab = Btns::A | Btns::B;
		let no = Btns(0);

		assert!(all.contains(&Button::A));
		assert!(all.contains(&Button::B));
		assert!(all.contains(&Button::Left));

		assert!(ab.contains(&Button::A));
		assert!(ab.contains(&Button::B));
		assert!(!ab.contains(&Button::Left));

		assert!(!no.contains(&Button::A));
		assert!(!no.contains(&Button::B));
		assert!(!no.contains(&Button::Left));
		assert!(no.is_empty());
	}

	#[test]
	fn contains_all() {
		let all = Btns::All;
		let ab = Btns::A | Btns::B;
		let no = Btns(0);

		assert!(all.contains_all(&ab));
		assert!(!ab.contains_all(&all));

		assert!(all.contains_all(&no));
		assert!(!no.contains_all(&all));

		assert!(ab.contains_all(&no));
		assert!(!no.contains_all(&ab));
	}


	#[test]
	fn intersects_one() {
		let all = Btns::All;

		assert!(all.a());
		assert!(all.b());
		assert!(all.left());
		assert!(all.right());
		assert!(all.up());
		assert!(all.down());
	}

	#[test]
	fn intersects_self() {
		let btns = [Btns::All, Btns::A | Btns::B, Btns(0)];

		for btns in btns {
			assert!(btns.intersects(&btns));
		}
	}

	#[test]
	fn intersects_some() {
		let all = Btns::All;
		let ab = Btns::A | Btns::B;
		let no = Btns(0);

		assert_ne!(all, ab);
		assert_ne!(ab, all);
		assert_ne!(ab, no);

		assert!(all.intersects(&ab));
		assert!(ab.intersects(&all));

		assert!(!all.intersects(&no));
		assert!(!no.intersects(&all));
		assert!(!ab.intersects(&no));
		assert!(!no.intersects(&ab));
	}


	mod iter {
		use super::*;
		use alloc::vec::Vec;


		#[test]
		fn into_iter() {
			let btns = [(Btns::All, 6), (Btns::A | Btns::B, 2), (Btns(0), 0)];

			for (btns, expected) in btns {
				assert_eq!(expected, btns.into_iter().count());
			}


			let expected = [Btns::B, Btns::A];
			assert_eq!(
			           expected.as_slice(),
			           (Btns::A | Btns::B).into_iter().collect::<Vec<_>>().as_slice()
			);
		}

		#[test]
		fn into_iter_btn() {
			let btns = [(Btns::All, 6), (Btns::A | Btns::B, 2), (Btns(0), 0)];

			for (btns, expected) in btns {
				assert_eq!(expected, btns.into_iter_flatten().count());
			}


			let expected = [Button::B, Button::A];
			assert_eq!(
			           expected.as_slice(),
			           (Btns::A | Btns::B).into_iter_flatten()
			                              .collect::<Vec<_>>()
			                              .as_slice()
			);
		}

		#[test]
		fn flat_btn() {
			let btns = [(Btns::All, 6), (Btns::A | Btns::B, 2), (Btns(0), 0)];

			for (btns, _) in btns {
				btns.into_iter()
				    .flat_btn()
				    .zip(btns.into_iter_flatten())
				    .for_each(|(a, b)| assert_eq!(a, b));
			}
		}
	}
}
