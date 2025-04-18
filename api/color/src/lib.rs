#![no_std]
#![cfg_attr(not(test), no_main)]
#![feature(const_trait_impl, const_deref, ptr_as_uninit)]

extern crate sys;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::ops::Deref;
use core::ops::DerefMut;
use core::ptr::NonNull;

use sys::ffi::Color as UnsafeLcdColor;
use sys::ffi::Pattern;
use sys::ffi::SolidColor;

pub mod fmt;
pub mod pattern;


/// Safe impl of [`LcdColor`](sys::ffi::Color) with preserved lifetime of [`Pattern`].
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
// May be better to use PhantomInvariantLifetime in a future
pub struct LcdColor<'t>(UnsafeLcdColor, PhantomData<&'t ()>);

impl LcdColor<'_> {
	const fn new(value: UnsafeLcdColor) -> Self { Self(value, PhantomData) }
}
impl From<UnsafeLcdColor> for LcdColor<'_> {
	fn from(value: UnsafeLcdColor) -> Self { Self::new(value) }
}
#[allow(clippy::from_over_into)]
impl Into<UnsafeLcdColor> for LcdColor<'_> {
	fn into(self) -> UnsafeLcdColor { self.0 }
}
impl const Deref for LcdColor<'_> {
	type Target = UnsafeLcdColor;
	fn deref(&self) -> &Self::Target { &self.0 }
}
impl const DerefMut for LcdColor<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}


#[derive(Clone, Debug)]
pub enum Color<'t> {
	Solid(SolidColor),
	Pattern(&'t MaybeUninit<Pattern>),
}

impl<'a, 'b> PartialEq<Color<'b>> for Color<'a> {
	fn eq(&self, other: &Color<'b>) -> bool {
		match (self, other) {
			(Self::Solid(a), Color::Solid(b)) => a == b,
			(Self::Pattern(a), Color::Pattern(b)) => core::ptr::eq(a.as_ptr(), b.as_ptr()),
			_ => false,
		}
	}
}

impl Color<'_> {
	pub const WHITE: Self = Self::Solid(SolidColor::White);
	pub const BLACK: Self = Self::Solid(SolidColor::Black);
	pub const CLEAR: Self = Self::Solid(SolidColor::Clear);
	pub const XOR: Self = Self::Solid(SolidColor::XOR);
}

impl<'t> From<Color<'t>> for LcdColor<'t> where Self: 't {
	fn from(color: Color) -> Self {
		match color {
			Color::Solid(color) => Self::from(color as UnsafeLcdColor),
			Color::Pattern(pattern) => Self::from(pattern.as_ptr() as UnsafeLcdColor),
		}
	}
}

impl<'t> From<LcdColor<'t>> for Color<'t> where Self: 't {
	fn from(color: LcdColor) -> Self {
		match color.0 {
			0 => Self::Solid(SolidColor::Black),
			1 => Self::Solid(SolidColor::White),
			2 => Self::Solid(SolidColor::Clear),
			3 => Self::Solid(SolidColor::XOR),
			color => {
				// SAFETY: The value `color` is already checked and is not zero, so is not null-ptr.
				// Of course it may be misaligned. 🤷🏻‍♂️
				let ptr = unsafe { NonNull::new_unchecked(color as *mut Pattern) };
				Self::Pattern(unsafe { ptr.as_uninit_ref() })
			},
		}
	}
}

impl<'t> From<&'t Pattern> for Color<'t> {
	fn from(pattern: &'t Pattern) -> Self {
		// Same as MaybeUninit::transpose.
		// SAFETY: T and MaybeUninit<T> have the same layout
		#[allow(clippy::missing_transmute_annotations)]
		Color::Pattern(unsafe { core::mem::transmute(pattern) })
	}
}


#[const_trait]
pub trait ColorExt {
	fn is_solid(&self) -> bool;
	fn is_pattern(&self) -> bool;
}

impl const ColorExt for LcdColor<'_> {
	fn is_solid(&self) -> bool { self.0 >= SolidColor::Black as _ && self.0 <= SolidColor::XOR as _ }
	fn is_pattern(&self) -> bool { !self.is_solid() }
}


#[const_trait]
pub trait IntoColor<'t> {
	fn into_color(self) -> LcdColor<'t>;
}

impl const IntoColor<'static> for SolidColor {
	fn into_color(self) -> LcdColor<'static> { LcdColor::new(self as UnsafeLcdColor) }
}

impl<'t> IntoColor<'t> for &'t Pattern {
	#[inline(always)]
	fn into_color(self) -> LcdColor<'t> { LcdColor::new(self.as_ptr() as _) }
}
