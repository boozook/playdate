#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]
#![feature(impl_trait_in_assoc_type)]

extern crate sys;
use core::ptr::NonNull;
use core::usize;

use sys::error::NullPtrError;
use sys::ffi::LCDColor;
use sys::ffi::LCDPattern;
use sys::ffi::LCDSolidColor;


#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub enum Color<'t> {
	Solid(LCDSolidColor),
	Pattern(&'t LCDPattern),
}

impl Color<'_> {
	pub const WHITE: Self = Self::Solid(LCDSolidColor::kColorWhite);
	pub const BLACK: Self = Self::Solid(LCDSolidColor::kColorBlack);
	pub const CLEAR: Self = Self::Solid(LCDSolidColor::kColorClear);
	pub const XOR: Self = Self::Solid(LCDSolidColor::kColorXOR);
}

impl<'t> From<Color<'t>> for LCDColor
	where Self: 't,
	      LCDColor: 't
{
	fn from(color: Color) -> Self {
		match color {
			Color::Solid(color) => color as LCDColor,
			Color::Pattern(pattern) => (pattern as *const u8) as LCDColor,
		}
	}
}

impl<'t> TryFrom<LCDColor> for Color<'t>
	where LCDColor: 't,
	      Self: 't
{
	type Error = NullPtrError;

	fn try_from(color: LCDColor) -> Result<Self, Self::Error> {
		match color {
			0 => Ok(Self::Solid(LCDSolidColor::Black())),
			1 => Ok(Self::Solid(LCDSolidColor::White())),
			2 => Ok(Self::Solid(LCDSolidColor::Clear())),
			3 => Ok(Self::Solid(LCDSolidColor::XOR())),
			color => {
				NonNull::new(color as *mut LCDPattern).ok_or(NullPtrError)
				                                      .map(|nn| Self::Pattern(unsafe { nn.as_ref() }))
			},
		}
	}
}

impl<'t> From<&'t LCDPattern> for Color<'t> {
	fn from(pattern: &'t LCDPattern) -> Self { Color::Pattern(pattern) }
}


#[const_trait]
pub trait LCDColorExt {
	#![allow(non_snake_case)]
	fn White() -> Self;
	fn Black() -> Self;
	fn Clear() -> Self;
	fn XOR() -> Self;
}

impl const LCDColorExt for LCDColor {
	#![allow(non_snake_case)]
	fn White() -> Self { LCDSolidColor::kColorWhite as Self }
	fn Black() -> Self { LCDSolidColor::kColorBlack as Self }
	fn Clear() -> Self { LCDSolidColor::kColorClear as Self }
	fn XOR() -> Self { LCDSolidColor::kColorXOR as Self }
}

impl const LCDColorExt for LCDSolidColor {
	#![allow(non_snake_case)]
	fn White() -> Self { LCDSolidColor::kColorWhite }
	fn Black() -> Self { LCDSolidColor::kColorBlack }
	fn Clear() -> Self { LCDSolidColor::kColorClear }
	fn XOR() -> Self { LCDSolidColor::kColorXOR }
}


#[const_trait]
pub trait LCDColorIs {
	fn is_solid(&self) -> bool;
	fn is_pattern(&self) -> bool;
}

impl const LCDColorIs for LCDColor {
	fn is_solid(&self) -> bool {
		let color = *self as usize;
		color >= LCDSolidColor::kColorBlack as _ && color <= LCDSolidColor::kColorXOR as _
	}
	fn is_pattern(&self) -> bool { !self.is_solid() }
}


#[const_trait]
pub trait IntoLCDColor {
	fn into_color(self) -> LCDColor;
}

impl const IntoLCDColor for LCDSolidColor {
	fn into_color(self) -> LCDColor { self as LCDColor }
}

impl<'t> IntoLCDColor for &'t LCDPattern where LCDColor: 't {
	#[inline(always)]
	fn into_color(self) -> LCDColor { self as *const u8 as _ }
}


#[const_trait]
pub trait LCDColorFmt<'t> {
	type Display: 't + core::fmt::Debug + core::fmt::Display;
	fn display(&'t self) -> Self::Display;
}

impl<'t> const LCDColorFmt<'t> for LCDSolidColor {
	type Display = LCDColorDisplay<'t, Self>;
	fn display(&self) -> LCDColorDisplay<'_, Self> { LCDColorDisplay(self) }
}

pub struct LCDColorDisplay<'t, T>(&'t T);

impl core::fmt::Debug for LCDColorDisplay<'_, LCDSolidColor> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str("Solid")?;
		let name = match self.0 {
			LCDSolidColor::kColorBlack => "Black",
			LCDSolidColor::kColorWhite => "White",
			LCDSolidColor::kColorClear => "Clear",
			LCDSolidColor::kColorXOR => "XOR",
		};
		f.write_str(name)
	}
}

impl core::fmt::Display for LCDColorDisplay<'_, LCDSolidColor> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let ch = match self.0 {
			LCDSolidColor::kColorBlack => 'B',
			LCDSolidColor::kColorWhite => 'W',
			LCDSolidColor::kColorClear => 'C',
			LCDSolidColor::kColorXOR => 'X',
		};
		write!(f, "{ch}")
	}
}

impl core::fmt::Debug for LCDColorDisplay<'_, LCDColor> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.0 {
			n if *n == LCDSolidColor::kColorBlack as _ => LCDSolidColor::kColorBlack.display().fmt(f),
			n if *n == LCDSolidColor::kColorWhite as _ => LCDSolidColor::kColorWhite.display().fmt(f),
			n if *n == LCDSolidColor::kColorClear as _ => LCDSolidColor::kColorClear.display().fmt(f),
			n if *n == LCDSolidColor::kColorXOR as _ => LCDSolidColor::kColorXOR.display().fmt(f),
			p => write!(f, "Pattern({:p})", *p as *const u8),
		}
	}
}

impl core::fmt::Display for LCDColorDisplay<'_, LCDColor> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.0 {
			n if *n == LCDSolidColor::kColorBlack as _ => LCDSolidColor::kColorBlack.display().fmt(f),
			n if *n == LCDSolidColor::kColorWhite as _ => LCDSolidColor::kColorWhite.display().fmt(f),
			n if *n == LCDSolidColor::kColorClear as _ => LCDSolidColor::kColorClear.display().fmt(f),
			n if *n == LCDSolidColor::kColorXOR as _ => LCDSolidColor::kColorXOR.display().fmt(f),
			_ => write!(f, "Pattern"),
		}
	}
}
