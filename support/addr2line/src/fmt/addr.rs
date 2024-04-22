use std::ops::Range;
use std::fmt::LowerHex;
use std::fmt::Display;
use std::fmt::Debug;

use super::report::WriteReport;
use super::report::INDENT;


pub const FLASH_MEM_REV1: Range<u64> = 0x08000000..0x08100000;
pub const SYS_MEM_REV1: Range<u64> = 0x20000000..0x20050000;
pub const USER_HEAP_REV1: Range<u64> = 0x60000000..0x61000000;

pub(crate) type DEF = u64;


pub fn parse_addr(addr: &str) -> anyhow::Result<Addr<DEF>> {
	use anyhow::Context;

	match addr.strip_prefix("0x") {
		Some(addr) => DEF::from_str_radix(addr, 16).map(Into::into),
		None => addr.parse::<DEF>().map(Into::into),
	}.context("Unable to parse address")
}


#[derive(Clone, Copy, PartialOrd, PartialEq, Eq, Hash, Ord)]
pub struct Addr<T = DEF> {
	/// Originally passed address.
	value: T,
	/// Fixed address by rev, relative to elf's load address.
	pub fixed: Option<T>,
}

impl<T: Copy + LowerHex> WriteReport for Addr<T> {
	fn default_print<W: std::io::prelude::Write>(&self,
	                                             mut out: W,
	                                             _: bool,
	                                             indent: usize,
	                                             _: bool,
	                                             _: bool,
	                                             _: bool,
	                                             _: bool)
	                                             -> std::io::Result<()> {
		let indent = INDENT.repeat(indent);
		if self.is_fixed() {
			write!(out, "{indent}{:#08x} [=>{:#08x}]", self.as_value(), self.fixed())
		} else {
			write!(out, "{indent}{:#08x}", self.as_value())
		}
	}
}

impl<T: LowerHex + Copy> LowerHex for Addr<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.fixed().fmt(f) }
}
impl<T> Addr<T> {
	pub fn new(value: T) -> Self { Addr { value, fixed: None } }
	pub fn is_fixed(&self) -> bool { self.fixed.is_some() }
	pub fn set_fixed(&mut self, value: T) { self.fixed = Some(value) }
}
impl<T: Copy> Addr<T> {
	/// Initial address.
	pub fn value(&self) -> T { self.value }
	/// Fixed address relative to elf's load address, or initial.
	pub fn fixed(&self) -> T { self.fixed.unwrap_or(self.value) }
	pub fn fixed_ref(&self) -> &T { self.fixed.as_ref().unwrap_or(&self.value) }

	pub fn as_value(&self) -> Self { Self::new(self.value) }
	pub fn as_fixed(&self) -> Self { Self::new(self.fixed()) }
}

impl<T> From<T> for Addr<T> {
	fn from(value: T) -> Self { Self::new(value) }
}

impl<T: Debug + Copy + LowerHex> Debug for Addr<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "({:#08x}", self.value())?;
		if let Some(v) = self.fixed {
			write!(f, "=>{v:#08x})")?;
		} else {
			write!(f, ")")?;
		}
		Ok(())
	}
}
impl<T: Display + Copy> Display for Addr<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.fixed().fmt(f) }
}

impl<T: Copy> AsRef<T> for Addr<T> {
	fn as_ref(&self) -> &T { self.fixed_ref() }
}

impl<T: PartialEq + Copy> PartialEq<T> for Addr<T> {
	#[inline]
	fn eq(&self, other: &T) -> bool { self.fixed().eq(other) }
}
impl PartialEq<Addr<u64>> for u64 {
	#[inline]
	fn eq(&self, other: &Addr<u64>) -> bool { self.eq(other.fixed_ref()) }
}
impl PartialEq<Addr<u32>> for u32 {
	#[inline]
	fn eq(&self, other: &Addr<u32>) -> bool { self.eq(other.fixed_ref()) }
}

impl<T: PartialOrd + Copy> PartialOrd<T> for Addr<T> {
	#[inline]
	fn partial_cmp(&self, other: &T) -> Option<std::cmp::Ordering> { self.fixed().partial_cmp(other) }
}
impl PartialOrd<Addr<u64>> for u64 {
	#[inline]
	fn partial_cmp(&self, other: &Addr<u64>) -> Option<std::cmp::Ordering> { self.partial_cmp(other.fixed_ref()) }
}
impl PartialOrd<Addr<u32>> for u32 {
	#[inline]
	fn partial_cmp(&self, other: &Addr<u32>) -> Option<std::cmp::Ordering> { self.partial_cmp(other.fixed_ref()) }
}
