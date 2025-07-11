//! Playdate bitmap API

use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ptr::NonNull;

use ::color::IntoColor;
use sys::error::ApiError;
use sys::ffi::Aabb;
use sys::macros::*;
use sys::ffi::Pattern;
use sys::ffi::SolidColor;
use sys::ffi::Color;
use sys::ffi::Bitmap as SysBitmap;
use fs::path::Path;

use crate::Api;
use crate::Graphics;
use crate::error;

pub use sys::ffi::BitmapFlip;
pub use sys::ffi::BitmapDrawMode;
pub use any::AsBitmap;


pub mod table;
pub mod tilemap;

mod any {
	use sys::utils::AsRaw;
	use super::*;


	#[const_trait]
	pub trait AsBitmap: AsRaw<Output = SysBitmap> {}
	impl<T: ~const AsRaw<Output = SysBitmap>> const AsBitmap for T {}
}


pub use ty::{Bitmap, Borrowed, Pointing as BitmapView};
use ty::*;


mod ty {
	use core::marker::PhantomData;
	use core::mem::ManuallyDrop;
	use core::ops::Deref;
	use core::ops::DerefMut;
	use core::ptr::NonNull;
	use sys::utils::AsRaw;
	use sys::ffi::Bitmap as SysBitmap;

	use crate::{AsRef, AsMut};


	#[must_use]
	#[repr(transparent)]
	pub struct Bitmap(NonNull<SysBitmap>);

	impl Bitmap {
		pub const unsafe fn from_ptr(ptr: NonNull<SysBitmap>) -> Self { Self(ptr) }
		pub(super) const fn as_ptr(&self) -> *mut SysBitmap { self.0.as_ptr() }
	}

	impl const AsRaw for Bitmap {
		type Output = SysBitmap;
		#[inline(always)]
		unsafe fn as_raw(&self) -> NonNull<Self::Output> { self.0 }
	}


	#[must_use]
	#[repr(transparent)]
	pub struct Borrowed<'owner>(ManuallyDrop<Bitmap>, PhantomData<&'owner ()>);

	impl Borrowed<'_> {
		pub const fn from_ptr(ptr: NonNull<SysBitmap>) -> Self { Self(ManuallyDrop::new(Bitmap(ptr)), PhantomData) }
	}

	impl<'o> const AsRef<'o, Bitmap> for Borrowed<'o> where ManuallyDrop<Bitmap>: ~const Deref {
		fn as_ref<'t>(&'t self) -> &'t Bitmap
			where 'o: 't {
			&self.0
		}
	}
	impl<'o> const AsMut<'o, Bitmap> for Borrowed<'o> where ManuallyDrop<Bitmap>: ~const DerefMut {
		fn as_mut<'t>(&'t mut self) -> &'t mut Bitmap
			where 'o: 't {
			&mut self.0
		}
	}

	impl<'t, 'l> const Deref for Borrowed<'t> where Self: ~const AsRef<'t, Bitmap> {
		type Target = Bitmap;
		fn deref(&self) -> &Self::Target { self.as_ref() }
	}
	impl<'t, 'l> const DerefMut for Borrowed<'t>
		where Self: ~const AsMut<'t, Bitmap> + ~const Deref<Target = Bitmap>
	{
		fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
	}

	// impl const AsRaw for Borrowed<'_> where ManuallyDrop<Bitmap>: ~const Deref {
	// 	type Output = SysBitmap;
	// 	#[inline(always)]
	// 	unsafe fn as_raw(&self) -> NonNull<Self::Output> { self.0.0 }
	// }


	/// Owned [`Bitmap`], internally pointing to other bitmap's internals.
	#[must_use]
	#[repr(transparent)]
	pub struct Pointing<'owner>(Bitmap, PhantomData<&'owner Bitmap>);

	impl Pointing<'_> {
		pub const unsafe fn from_ptr(ptr: NonNull<SysBitmap>) -> Self { Self(Bitmap(ptr), PhantomData) }
	}

	impl<'o> const AsRef<'o, Bitmap> for Pointing<'o> {
		#[inline(always)]
		fn as_ref<'t>(&'t self) -> &'t Bitmap
			where 'o: 't {
			&self.0
		}
	}

	impl<'t, 'l> const Deref for Pointing<'t> where Self: ~const AsRef<'t, Bitmap> {
		type Target = Bitmap;
		fn deref(&self) -> &Self::Target { self.as_ref() }
	}

	// impl const AsRaw for Pointing<'_> where ManuallyDrop<Bitmap>: ~const Deref {
	// 	type Output = SysBitmap;
	// 	#[inline(always)]
	// 	unsafe fn as_raw(&self) -> NonNull<Self::Output> { self.0.0 }
	// }
}


// impl<'src: 't, 't> AsRef<ManuallyDrop<Bitmap>> for Borrowed<'src> {
// 	fn as_ref(&self) -> &ManuallyDrop<Bitmap> { &self.0 }
// }
// impl<'src: 't, 't> AsMut<ManuallyDrop<Bitmap>> for Borrowed<'src> {
// 	fn as_mut(&mut self) -> &mut ManuallyDrop<Bitmap> { &mut self.0 }
// }

// impl<'src: 't, 't> Borrow<ManuallyDrop<Bitmap>> for Borrowed<'src> where for<'l> &'l Self: 't {
// 	fn borrow(&self) -> &ManuallyDrop<Bitmap> { &self.0 }
// }
// impl<'src: 't, 't> BorrowMut<ManuallyDrop<Bitmap>> for Borrowed<'src> where for<'l> &'l mut Self: 't {
// 	fn borrow_mut(&mut self) -> &mut ManuallyDrop<Bitmap> { &mut self.0 }
// }

// impl ToOwned for Borrowed<'_> {
// 	type Owned = Bitmap;
// 	fn to_owned(&self) -> Self::Owned { self.0.clone(api!(graphics)).unwrap() }
// }

// impl<'src> Borrow<Borrowed<'src>> for Bitmap {
// 	fn borrow(&self) -> &Borrowed<'src> { unsafe { core::mem::transmute(self) } }
// }
// impl<'src> BorrowMut<Borrowed<'src>> for Bitmap {
// 	fn borrow_mut(&mut self) -> &mut Borrowed<'src> { unsafe { core::mem::transmute(self) } }
// }


impl Bitmap {
	/// Allocates and returns a new `width` by `height` Bitmap filled with `bg` color.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::newBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::newBitmap")]
	pub fn new<'c>(api: Api, width: c_int, height: c_int, bg: impl IntoColor<'c>) -> Result<Self, error::Alloc> {
		let ptr = unsafe { (api.newBitmap)(width, height, bg.into_color().into_raw()) };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(unsafe { Self::from_ptr(NonNull::new_unchecked(ptr)) })
		}
	}


	/// Load a bitmap from a file,
	///
	/// Calls [`sys::ffi::PlaydateGraphics::loadBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::loadBitmap")]
	pub fn load<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, error::LoadError> {
		let path = path.as_ref();
		let mut err: *const c_char = core::ptr::null();

		let ptr = unsafe { (api.loadBitmap)(path.as_ptr(), &raw mut err) };

		if ptr.is_null() {
			if let Some(err) = unsafe { fs::error::Owned::from_ptr(err) } {
				Err(error::LoadError::Fs(err))
			} else {
				Err(error::LoadError::Alloc(error::Alloc))
			}
		} else {
			Ok(unsafe { Self::from_ptr(NonNull::new_unchecked(ptr)) })
		}
	}
}


impl Bitmap {
	/// Load a bitmap from a file into `self`.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::loadIntoBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::loadIntoBitmap")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn load_into<'t, P: AsRef<Path>>(&'t mut self, api: Api, path: P) -> Result<(), fs::error::Borrowed<'t>> {
		let path = path.as_ref();
		let mut err: *const c_char = core::ptr::null();

		unsafe { (api.loadIntoBitmap)(path.as_ptr(), self.as_ptr(), &raw mut err) };

		if let Some(err) = unsafe { fs::error::Error::from_ptr(err) } {
			Err(err)
		} else {
			Ok(())
		}
	}
}


impl Drop for Bitmap {
	fn drop(&mut self) {
		if let Some(f) = api_opt!(graphics.freeBitmap) {
			unsafe { f(self.as_ptr()) };
		}
	}
}

impl Bitmap {
	/// Allocates and returns a new `Bitmap` that is an exact copy of `self`,
	/// __not a reference__.
	///
	/// For [`Pointing`] bitmap it also copies data of the pointed bitmap into the new bitmap,
	/// so the _clone of "pointing bitmap" is not a pointing, but normal owned bitmap_.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::copyBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::copyBitmap")]
	pub fn clone(&self, api: Api) -> Result<Self, error::Alloc> {
		let f = api.copyBitmap;
		let ptr = unsafe { f(self.as_ptr()) };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(unsafe { Self::from_ptr(NonNull::new_unchecked(ptr)) })
		}
	}
}


impl Bitmap {
	/// Clears bitmap, filling with the given `bg` color.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::clearBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::clearBitmap")]
	pub fn clear<'c>(&mut self, api: Api, bg: impl IntoColor<'c>) {
		let f = api.clearBitmap;
		unsafe { f(self.as_ptr(), bg.into_color().into_raw()) };
	}


	/// Returns `(width, height)` of the bitmap.
	///
	/// Can return error if there is no bitmap-data or any internal error occurred.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::getBitmapData`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapData")]
	pub fn size(&self, api: Api) -> (c_int, c_int) {
		let mut width: c_int = 0;
		let mut height: c_int = 0;
		let mut row_bytes: c_int = 0;

		let f = api.getBitmapData;
		unsafe {
			f(
			  self.as_ptr(),
			  &mut width,
			  &mut height,
			  &mut row_bytes,
			  core::ptr::null_mut(),
			  core::ptr::null_mut(),
			)
		};

		(width, height)
	}

	/// Returns mutable borrow of bitmap-data by this bitmap.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::getBitmapData`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapData")]
	pub fn bitmap_data<'src>(&'src mut self, api: Api) -> BitmapData<'src> {
		let mut width: c_int = 0;
		let mut height: c_int = 0;
		let mut row_bytes: c_int = 0;
		let mut mask = core::ptr::null_mut();
		let mut data = core::ptr::null_mut();

		let f = api.getBitmapData;
		unsafe {
			f(
			  self.as_ptr(),
			  &mut width,
			  &mut height,
			  &mut row_bytes,
			  &raw mut mask,
			  &raw mut data,
			)
		};

		let len = row_bytes * height;

		// get mask:
		let mask = if mask.is_null() {
			None
		} else {
			Some(unsafe { core::slice::from_raw_parts_mut::<u8>(mask, len as usize) })
		};

		// get data:
		let data = unsafe { core::slice::from_raw_parts_mut::<u8>(data, len as usize) };

		BitmapData { width,
		             height,
		             row_bytes,
		             mask,
		             data }
	}


	/// Sets a mask image for the bitmap.
	/// The set mask must be the same size as the `self` bitmap.
	///
	/// Behaviour:
	/// internally __copies__ the `mask`'s data to the `self` bitmap.
	/// The `mask` is not borrowed, so it can be freed or modified freely.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::setBitmapMask`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setBitmapMask")]
	pub fn set_mask(&mut self, api: Api, mask: &impl AsBitmap) -> Result<(), error::InvalidMask> {
		let res = unsafe { (api.setBitmapMask)(self.as_ptr(), mask.as_raw().as_ptr()) };
		if res == 0 { Err(error::InvalidMask) } else { Ok(()) }
	}

	/// Gets a mask layer wrapped into `Bitmap` for the `self` bitmap.
	/// If the `self` bitmap doesn’t have a mask layer, returns None.
	///
	/// The returned bitmap points to bitmap's data, so drawing into that bitmap affects the source (`self`) bitmap's mask directly.
	///
	/// See also [`bitmap_data`](Self::bitmap_data), it doesn’t allocates new `Bitmap`.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::getBitmapMask`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapMask")]
	pub fn mask<'t>(&'t self, api: Api) -> Option<Pointing<'t>> {
		let ptr = unsafe { (api.getBitmapMask)(self.as_ptr()) };
		NonNull::new(ptr).map(|ptr| unsafe { Pointing::from_ptr(ptr) })
	}


	/// Returns a new, rotated and scaled Bitmap based on the bitmap using given `api`.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::rotatedBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::rotatedBitmap")]
	pub fn rotated_clone(&self,
	                     api: Api,
	                     rotation: c_float,
	                     x_scale: c_float,
	                     y_scale: c_float)
	                     -> Result<Self, error::Alloc> {
		let mut alloced_size: c_int = 0;
		let f = api.rotatedBitmap;
		let ptr = unsafe { f(self.as_ptr(), rotation, x_scale, y_scale, &raw mut alloced_size) };

		if alloced_size == 0 || ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(unsafe { Self::from_ptr(NonNull::new_unchecked(ptr)) })
		}
	}


	/// Draws `self` with its upper-left corner at location `x`, `y`,
	/// using the given `flip` orientation.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawBitmap")]
	#[inline(always)]
	pub fn draw(&self, api: Api, x: c_int, y: c_int, flip: BitmapFlip) {
		unsafe { (api.drawBitmap)(self.as_ptr(), x, y, flip) }
	}

	/// Draws `self` with its upper-left corner at location `x`, `y`
	/// __tiled inside a `width` by `height` rectangle__.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::tileBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::tileBitmap")]
	#[inline(always)]
	pub fn draw_tiled(&self, api: Api, x: c_int, y: c_int, width: c_int, height: c_int, flip: BitmapFlip) {
		unsafe { (api.tileBitmap)(self.as_ptr(), x, y, width, height, flip) }
	}

	/// Draws the *bitmap* scaled to `x_scale` and `y_scale`
	/// then rotated by `degrees` with its center as given by proportions `center_x` and `center_y` at `x`, `y`;
	///
	/// that is:
	/// * if `center_x` and `center_y` are both 0.5 the center of the image is at (`x`,`y`),
	/// * if `center_x` and `center_y` are both 0 the top left corner of the image (before rotation) is at (`x`,`y`), etc.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawRotatedBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawRotatedBitmap")]
	#[inline(always)]
	pub fn draw_rotated(&self,
	                    api: Api,
	                    x: c_int,
	                    y: c_int,
	                    degrees: c_float,
	                    center_x: c_float,
	                    center_y: c_float,
	                    x_scale: c_float,
	                    y_scale: c_float) {
		unsafe { (api.drawRotatedBitmap)(self.as_ptr(), x, y, degrees, center_x, center_y, x_scale, y_scale) }
	}

	/// Draws this bitmap scaled to `x_scale` and `y_scale` with its upper-left corner at location `x`, `y`.
	///
	/// Note that flip is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawScaledBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::drawScaledBitmap")]
	#[inline(always)]
	pub fn draw_scaled(&self, api: Api, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
		unsafe { (api.drawScaledBitmap)(self.as_ptr(), x, y, x_scale, y_scale) }
	}


	/// Returns `true` if any of the opaque pixels in this bitmap when positioned at `x, y` with `flip`
	/// overlap any of the opaque pixels in `other` bitmap at `x_other`, `y_other` with `flip_other`
	/// within the non-empty `rect`,
	/// or `false` if no pixels overlap or if one or both fall completely outside of `rect`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::checkMaskCollision`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::checkMaskCollision")]
	#[inline(always)]
	pub fn check_mask_collision(&self,
	                            api: Api,
	                            x: c_int,
	                            y: c_int,
	                            flip: BitmapFlip,
	                            other: impl AsBitmap,
	                            x_other: c_int,
	                            y_other: c_int,
	                            flip_other: BitmapFlip,
	                            rect: Aabb)
	                            -> bool {
		unsafe {
			(api.checkMaskCollision)(
			                         self.as_ptr(),
			                         x,
			                         y,
			                         flip,
			                         other.as_raw().as_ptr(),
			                         x_other,
			                         y_other,
			                         flip_other,
			                         rect,
			) == 1
		}
	}


	/// Returns pattern `8 x 8` from this bitmap.
	///
	/// `x, y` indicates the top left corner of the 8 x 8 pattern in bitmap's coordinates.
	///
	/// Returned pattern is owned by rust-side and can be dropped freely.
	///
	/// Uses [`sys::ffi::PlaydateGraphics::setColorToPattern`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setColorToPattern")]
	pub fn pattern_at(&self, api: Api, x: c_int, y: c_int) -> Pattern {
		let mut color = Color::default();
		let f = api.setColorToPattern;

		unsafe {
			f(&raw mut color, self.as_ptr(), x, y);
			*(color as *mut u8 as *mut Pattern)
		}
	}

	/// Sets `color` to an `8 x 8` pattern using this bitmap.
	///
	/// `x, y` indicates the top left corner of the 8 x 8 pattern.
	///
	/// After this operation inner pointer is owned by the system.
	/// To get owned pattern use [`Bitmap::pattern_at`].
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setColorToPattern`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setColorToPattern")]
	pub fn set_color_to_pattern(&self, api: Api, color: &mut Color, x: c_int, y: c_int) {
		unsafe { (api.setColorToPattern)(color, self.as_ptr(), x, y) }
	}

	/// Gets the color of the pixel at `(x,y)` in this bitmap.
	/// If the coordinate is outside the bounds of the bitmap,
	/// or if the bitmap has a mask and the pixel is marked transparent,
	/// the function returns [`Clear`][SolidColor::kColorClear];
	/// otherwise the return value is [`White`][SolidColor::kColorWhite] or [`Black`][SolidColor::kColorBlack].
	///
	/// Calls [`sys::ffi::PlaydateGraphics::getBitmapPixel`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapPixel")]
	#[inline(always)]
	pub fn pixel_at(&self, api: Api, x: c_int, y: c_int) -> SolidColor {
		unsafe { (api.getBitmapPixel)(self.as_ptr(), x, y) }
	}
}


/// The data is 1 bit per pixel packed format, in MSB order; in other words,
/// the high bit of the first byte in data is the top left pixel of the image.
///
/// The `mask` data is in same format but means transparency.
pub struct BitmapData<'src> {
	pub width: c_int,
	pub height: c_int,
	pub row_bytes: c_int,
	mask: Option<&'src mut [u8]>,
	data: &'src mut [u8],
}

impl<'src> BitmapData<'src> {
	pub const fn width(&self) -> c_int { self.width }
	pub const fn height(&self) -> c_int { self.height }
	pub const fn row_bytes(&self) -> c_int { self.row_bytes }
	pub fn mask(&self) -> Option<&[u8]> { self.mask.as_deref() }
	pub fn mask_mut(&mut self) -> Option<&mut [u8]> { self.mask.as_deref_mut() }
	pub const fn data(&self) -> &[u8] { self.data }
	pub const fn data_mut(&mut self) -> &mut [u8] { self.data }
}

impl core::fmt::Display for BitmapData<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "BitmapData({}, {}", self.width(), self.height())?;
		if self.mask.is_some() {
			write!(f, ", masked)")
		} else {
			write!(f, ")")
		}
	}
}

impl core::fmt::Debug for BitmapData<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let alternate = f.alternate();
		if alternate {
			let fmt_bd = |f: &mut core::fmt::Formatter<'_>, data: &[u8], row_len: c_int| {
				for (i, b) in data.iter().enumerate() {
					if i % row_len as usize == 0 {
						f.write_str("\n\t")?;
					}
					f.write_fmt(format_args!("{b:08b} "))?;
				}
				Ok(())
			};

			write!(f, "BitmapData({}, {}", self.width(), self.height())?;
			if self.mask.is_some() {
				write!(f, ", masked")?;
			}
			write!(f, ", data:")?;
			fmt_bd(f, self.data, self.row_bytes)?;
			write!(f, ")")
		} else {
			let mut res = f.debug_struct("BitmapData");
			res.field("width", &self.width)
			   .field("height", &self.height)
			   .field("row_bytes", &self.row_bytes);
			res.field("data", &self.data).field("mask", &self.mask).finish()
		}
	}
}


//
// Global Bitmap-related methods
//


impl Graphics {
	/// Only valid in the Simulator,
	/// returns the debug framebuffer as a bitmap.
	///
	/// Returns error on device.
	///
	/// __The system owns this bitmap—​do not free it.__
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getDebugBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getDebugBitmap")]
	pub fn debug_frame_buffer(&self) -> Result<Borrowed<'static>, error::ApiError> {
		let f = self.0.getDebugBitmap.ok_or(ApiError)?;
		let ptr = unsafe { f() };
		if ptr.is_null() {
			Err(error::ApiError)
		} else {
			Ok(Borrowed::from_ptr(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}

	/// Returns a bitmap containing the contents of the display buffer.
	///
	/// __The system owns this bitmap—​do not free it.__
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getDisplayBufferBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getDisplayBufferBitmap")]
	pub fn frame_buffer(&self) -> Result<Borrowed<'static>, error::Alloc> {
		let ptr = unsafe { (self.0.getDisplayBufferBitmap)() };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(Borrowed::from_ptr(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}

	/// Returns a __copy__ the contents of the working frame buffer as a bitmap.
	///
	/// The caller is responsible for freeing the returned bitmap, it will automatically on drop.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::copyFrameBufferBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::copyFrameBufferBitmap")]
	pub fn frame_buffer_clone(&self) -> Result<Bitmap, error::Alloc> {
		let ptr = unsafe { (self.0.copyFrameBufferBitmap)() };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(unsafe { Bitmap::from_ptr(NonNull::new_unchecked(ptr)) })
		}
	}


	/// Sets the stencil used for drawing.
	///
	/// If the `tile` is `true` the stencil image will be tiled.
	///
	/// Tiled stencils must have width equal to a multiple of 32 pixels.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setStencilImage`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setStencilImage")]
	pub fn set_stencil_tiled(&self, image: &impl AsBitmap, tile: bool) {
		unsafe { (self.0.setStencilImage)(image.as_raw().as_ptr(), tile as _) };
	}

	/// Sets the stencil used for drawing.
	/// For a tiled stencil, use [`set_stencil_tiled`][] instead.
	///
	/// NOTE: Officially deprecated in favor of [`set_stencil_tiled`][], which adds a `tile` flag
	///
	/// [`set_stencil_tiled`]: Self::set_stencil_tiled
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setStencil`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setStencil")]
	pub fn set_stencil(&self, image: &impl AsBitmap) { unsafe { (self.0.setStencil)(image.as_raw().as_ptr()) }; }

	/// Sets the mode used for drawing bitmaps.
	///
	/// Returns the previous draw mode.
	///
	/// Note that text drawing uses bitmaps, so this affects how fonts are displayed as well.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::setDrawMode`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::setDrawMode")]
	pub fn set_draw_mode(&self, mode: BitmapDrawMode) -> BitmapDrawMode { unsafe { (self.0.setDrawMode)(mode) } }

	/// Push a new drawing context for drawing into the given bitmap.
	///
	/// If `target` is [`Borrowed::null()`], the drawing functions will use the display framebuffer.
	///
	/// To push framebuffer to context use [`push_framebuffer_to_context`](Self::push_framebuffer_to_context).
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::pushContext`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::pushContext")]
	pub fn push_context(&self, target: &impl AsBitmap) {
		unsafe { (self.0.pushContext)(target.as_raw().as_ptr()) };
	}

	/// Push a new drawing context for drawing into the display framebuffer.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::pushContext`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::pushContext")]
	pub fn push_frame_buffer_to_context(&self) { unsafe { (self.0.pushContext)(core::ptr::null_mut()) }; }

	/// Pops a context off the stack (if any are left),
	/// restoring the drawing settings from before the context was pushed.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::popContext`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::popContext")]
	pub fn pop_context(&self) { unsafe { (self.0.popContext)() }; }
}


// impl Graphics {
// 	/// Draws `self` with its upper-left corner at location `x`, `y`,
// 	/// using the given `flip` orientation.
// 	///
// 	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawBitmap`].
// 	#[doc(alias = "sys::ffi::PlaydateGraphics::drawBitmap")]
// 	#[inline(always)]
// 	pub fn draw(&self, bitmap: &impl AsBitmap, x: c_int, y: c_int, flip: BitmapFlip) {
// 		unsafe { (self.0.drawBitmap)(bitmap.as_raw().as_ptr(), x, y, flip) }
// 	}

// 	/// Draws `self` with its upper-left corner at location `x`, `y`
// 	/// __tiled inside a `width` by `height` rectangle__.
// 	///
// 	/// Equivalent to [`sys::ffi::PlaydateGraphics::tileBitmap`].
// 	#[doc(alias = "sys::ffi::PlaydateGraphics::tileBitmap")]
// 	#[inline(always)]
// 	pub fn draw_tiled(&self,
// 	                  bitmap: &impl AsBitmap,
// 	                  x: c_int,
// 	                  y: c_int,
// 	                  width: c_int,
// 	                  height: c_int,
// 	                  flip: BitmapFlip) {
// 		unsafe { (self.0.tileBitmap)(bitmap.as_raw().as_ptr(), x, y, width, height, flip) }
// 	}

// 	/// Draws the *bitmap* scaled to `x_scale` and `y_scale`
// 	/// then rotated by `degrees` with its center as given by proportions `center_x` and `center_y` at `x`, `y`;
// 	///
// 	/// that is:
// 	/// * if `center_x` and `center_y` are both 0.5 the center of the image is at (`x`,`y`),
// 	/// * if `center_x` and `center_y` are both 0 the top left corner of the image (before rotation) is at (`x`,`y`), etc.
// 	///
// 	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawRotatedBitmap`].
// 	#[doc(alias = "sys::ffi::PlaydateGraphics::drawRotatedBitmap")]
// 	#[inline(always)]
// 	pub fn draw_rotated(&self,
// 	                    bitmap: &impl AsBitmap,
// 	                    x: c_int,
// 	                    y: c_int,
// 	                    degrees: c_float,
// 	                    center_x: c_float,
// 	                    center_y: c_float,
// 	                    x_scale: c_float,
// 	                    y_scale: c_float) {
// 		unsafe {
// 			(self.0.drawRotatedBitmap)(
// 			                           bitmap.as_raw().as_ptr(),
// 			                           x,
// 			                           y,
// 			                           degrees,
// 			                           center_x,
// 			                           center_y,
// 			                           x_scale,
// 			                           y_scale,
// 			)
// 		}
// 	}

// 	/// Draws this bitmap scaled to `x_scale` and `y_scale` with its upper-left corner at location `x`, `y`.
// 	///
// 	/// Note that flip is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.
// 	///
// 	/// Equivalent to [`sys::ffi::PlaydateGraphics::drawScaledBitmap`].
// 	#[doc(alias = "sys::ffi::PlaydateGraphics::drawScaledBitmap")]
// 	#[inline(always)]
// 	pub fn draw_scaled(&self, bitmap: &impl AsBitmap, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
// 		unsafe { (self.0.drawScaledBitmap)(bitmap.as_raw().as_ptr(), x, y, x_scale, y_scale) }
// 	}
// }
