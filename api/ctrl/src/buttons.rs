use core::fmt::Debug;
use core::fmt::Display;
use core::ops::BitAnd;
use sys::ffi::PDButtons;

pub use crate::peripherals::buttons_shorthands::*;

pub trait PDButtonsExt: Sized + BitAnd<Self> {
	#![allow(non_snake_case)]

	/// Contains `other` button.
	///
	/// Note, `other` can contains one or many buttons.
	/// In case of there's many buttons, returns `true` if
	/// at least one button of `other` is contained in this.
	fn contains(&self, other: Self) -> bool;

	/// Contains `other` button.
	/// Same as [`Self::contains`] but not taking ownership of `other`.
	fn contains_ref(&self, other: &Self) -> bool;

	/// Contains any buttons, opposite of [`Self::is_empty`].
	fn any(&self) -> bool;

	/// There's no buttons
	fn is_empty(&self) -> bool;

	#[inline(always)]
	/// Contains `left` button.
	fn left(&self) -> bool { self.contains(Self::Left()) }

	#[inline(always)]
	/// Contains `right` button.
	fn right(&self) -> bool { self.contains(Self::Right()) }

	#[inline(always)]
	/// Contains `up` button.
	fn up(&self) -> bool { self.contains(Self::Up()) }

	#[inline(always)]
	/// Contains `down` button.
	fn down(&self) -> bool { self.contains(Self::Down()) }

	#[inline(always)]
	/// Contains `b` button.
	fn b(&self) -> bool { self.contains(Self::B()) }

	#[inline(always)]
	/// Contains `a` button.
	fn a(&self) -> bool { self.contains(Self::A()) }


	fn Left() -> Self;
	fn Right() -> Self;
	fn Up() -> Self;
	fn Down() -> Self;
	fn B() -> Self;
	fn A() -> Self;
}


impl PDButtonsExt for PDButtons {
	#![allow(non_snake_case)]

	#[inline(always)]
	fn contains(&self, other: Self) -> bool { (self.0 & other.0) != 0 }

	#[inline(always)]
	fn contains_ref(&self, other: &Self) -> bool { (self.0 & other.0) != 0 }

	#[inline(always)]
	fn any(&self) -> bool { self.0 > 0 }

	#[inline(always)]
	fn is_empty(&self) -> bool { self.0 == 0 }


	#[inline(always)]
	fn Left() -> Self { Self::kButtonLeft }

	#[inline(always)]
	fn Right() -> Self { Self::kButtonRight }

	#[inline(always)]
	fn Up() -> Self { Self::kButtonUp }

	#[inline(always)]
	fn Down() -> Self { Self::kButtonDown }

	#[inline(always)]
	fn B() -> Self { Self::kButtonB }

	#[inline(always)]
	fn A() -> Self { Self::kButtonA }
}


pub trait PDButtonsIntoIter {
	type Item;
	type IntoIter: Iterator<Item = Self::Item>;

	fn into_iter(self) -> Self::IntoIter;
}

impl PDButtonsIntoIter for PDButtons {
	type Item = &'static Self;
	type IntoIter = impl Iterator<Item = Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		static ALL: [PDButtons; 6] = [
		                              PDButtons::kButtonLeft,
		                              PDButtons::kButtonRight,
		                              PDButtons::kButtonUp,
		                              PDButtons::kButtonDown,
		                              PDButtons::kButtonB,
		                              PDButtons::kButtonA,
		];
		ALL[..].into_iter()
		       .filter(move |possible| (self.0 & possible.0) != 0)
	}
}

pub trait PDButtonsIter<'t> {
	type Item;
	type Iter: Iterator<Item = Self::Item>;

	fn iter(&'t self) -> Self::Iter;
}

impl<'t> PDButtonsIter<'t> for PDButtons {
	type Item = &'static PDButtons;
	type Iter = impl Iterator<Item = Self::Item> + 't;

	fn iter(&'t self) -> Self::Iter {
		static ALL: [PDButtons; 6] = [
		                              PDButtons::kButtonLeft,
		                              PDButtons::kButtonRight,
		                              PDButtons::kButtonUp,
		                              PDButtons::kButtonDown,
		                              PDButtons::kButtonB,
		                              PDButtons::kButtonA,
		];
		ALL[..].into_iter()
		       .filter(move |possible| self.contains_ref(possible))
	}
}


pub trait PDButtonsFmt: for<'t> PDButtonsIter<'t> {
	fn display(&self) -> PDButtonsDisplay<'_>;
}

impl PDButtonsFmt for PDButtons {
	fn display(&self) -> PDButtonsDisplay<'_> { PDButtonsDisplay(self) }
}

pub struct PDButtonsDisplay<'t>(&'t PDButtons);

impl<'t> Debug for PDButtonsDisplay<'t> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut fmt = f.debug_tuple(&format!("PDButtons[{:08b}]", self.0.0));
		let iter = self.0.iter().singles();
		for item in iter {
			fmt.field(&item);
		}
		fmt.finish()
	}
}

impl<'t> Display for PDButtonsDisplay<'t> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "(")?;
		let mut iter = self.0.iter().singles();
		if let Some(first) = iter.next() {
			write!(f, "{first:?}")?;
		}
		for item in iter {
			write!(f, ", {:?}", item)?;
		}
		write!(f, ")")
	}
}


/// Represents single button of [`PDButtons`].
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Button {
	Left,
	Right,
	Up,
	Down,
	B,
	A,
}

impl Into<PDButtons> for Button {
	fn into(self) -> PDButtons {
		match self {
			Button::Left => PDButtons::Left(),
			Button::Right => PDButtons::Right(),
			Button::Up => PDButtons::Up(),
			Button::Down => PDButtons::Down(),
			Button::B => PDButtons::B(),
			Button::A => PDButtons::A(),
		}
	}
}

impl PartialEq<PDButtons> for Button {
	fn eq(&self, other: &PDButtons) -> bool {
		match self {
			Button::Left => other.contains_ref(&PDButtons::Left()),
			Button::Right => other.contains_ref(&PDButtons::Right()),
			Button::Up => other.contains_ref(&PDButtons::Up()),
			Button::Down => other.contains_ref(&PDButtons::Down()),
			Button::B => other.contains_ref(&PDButtons::B()),
			Button::A => other.contains_ref(&PDButtons::A()),
		}
	}
}


pub trait IterSingleButtons {
	type Iter: Iterator<Item = Button>;

	/// Map each [`PDButtons`] to many [`Button`]s.
	fn singles(self) -> Self::Iter;
}

impl<T: Iterator<Item = <PDButtons as PDButtonsIntoIter>::Item>> IterSingleButtons for T {
	type Iter = impl Iterator<Item = Button>;

	fn singles(self) -> Self::Iter {
		static ALL: [Button; 6] = [
		                           Button::Left,
		                           Button::Right,
		                           Button::Up,
		                           Button::Down,
		                           Button::B,
		                           Button::A,
		];
		self.flat_map(|buttons| ALL.into_iter().filter(move |single| single == buttons))
	}
}
