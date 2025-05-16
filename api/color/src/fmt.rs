use sys::ffi::Color;
use sys::ffi::SolidColor;


#[const_trait]
pub trait ColorFmt<'t> {
	type Display: 't + core::fmt::Debug + core::fmt::Display;
	fn display(&'t self) -> Self::Display;
}

impl<'t> const ColorFmt<'t> for SolidColor {
	type Display = ColorDisplay<'t, Self>;
	fn display(&self) -> ColorDisplay<'_, Self> { ColorDisplay(self) }
}

pub struct ColorDisplay<'t, T>(&'t T);

impl core::fmt::Debug for ColorDisplay<'_, SolidColor> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.write_str("Solid")?;
		let name = match self.0 {
			SolidColor::Black => "Black",
			SolidColor::White => "White",
			SolidColor::Clear => "Clear",
			SolidColor::XOR => "XOR",
		};
		f.write_str(name)
	}
}

impl core::fmt::Display for ColorDisplay<'_, SolidColor> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let ch = match self.0 {
			SolidColor::Black => 'B',
			SolidColor::White => 'W',
			SolidColor::Clear => 'C',
			SolidColor::XOR => 'X',
		};
		write!(f, "{ch}")
	}
}

impl core::fmt::Debug for ColorDisplay<'_, Color> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.0 {
			n if *n == SolidColor::Black as _ => SolidColor::Black.display().fmt(f),
			n if *n == SolidColor::White as _ => SolidColor::White.display().fmt(f),
			n if *n == SolidColor::Clear as _ => SolidColor::Clear.display().fmt(f),
			n if *n == SolidColor::XOR as _ => SolidColor::XOR.display().fmt(f),
			p => write!(f, "Pattern({:p})", *p as *const u8),
		}
	}
}

impl core::fmt::Display for ColorDisplay<'_, Color> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self.0 {
			n if *n == SolidColor::Black as _ => SolidColor::Black.display().fmt(f),
			n if *n == SolidColor::White as _ => SolidColor::White.display().fmt(f),
			n if *n == SolidColor::Clear as _ => SolidColor::Clear.display().fmt(f),
			n if *n == SolidColor::XOR as _ => SolidColor::XOR.display().fmt(f),
			_ => write!(f, "Pattern"),
		}
	}
}
