//! Playdate bitmap API

use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::fmt::Write;
use core::marker::PhantomData;
use alloc::boxed::Box;

use sys::error::NullPtrError;
use sys::error::OkMutOrNullErr;
use sys::error::OkOrNullFnErr;
use sys::ffi::LCDPattern;
use sys::ffi::LCDSolidColor;
use sys::traits::AsRaw;
use sys::ffi::CString;
use sys::ffi::LCDColor;
use sys::ffi::LCDRect;
use sys::ffi::LCDBitmap;
use fs::Path;

use crate::Graphics;
use crate::error::ApiError;
use crate::error::Error;
use super::api;

pub use color::*;
pub use sys::ffi::LCDBitmapFlip as BitmapFlip;
pub use sys::ffi::LCDBitmapDrawMode as BitmapDrawMode;
pub use crate::{BitmapFlipExt, BitmapDrawModeExt};


pub trait AnyBitmap: AsRaw<Type = LCDBitmap> + BitmapApi {}
impl<T: AnyBitmap> AnyBitmap for &'_ T {}
impl AnyBitmap for BitmapRef<'_> {}
impl<Api: api::Api, const FOD: bool> AnyBitmap for Bitmap<Api, FOD> {}


pub trait BitmapApi {
	type Api: api::Api;
	fn api(&self) -> Self::Api
		where Self::Api: Copy;
	fn api_ref(&self) -> &Self::Api;
}

impl BitmapApi for BitmapRef<'_> {
	type Api = api::Default;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		api::Default::default()
	}

	fn api_ref(&self) -> &Self::Api { &self.1 }
}

impl<Api: api::Api, const FOD: bool> BitmapApi for Bitmap<Api, FOD> {
	type Api = Api;
	fn api(&self) -> Api
		where Self::Api: Copy {
		self.1
	}

	fn api_ref(&self) -> &Self::Api { &self.1 }
}

impl<T: BitmapApi> BitmapApi for &'_ T {
	type Api = T::Api;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		(*self).api()
	}

	fn api_ref(&self) -> &Self::Api { (*self).api_ref() }
}


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Bitmap<Api: api::Api = api::Default, const FREE_ON_DROP: bool = true>(pub(crate) *mut LCDBitmap,
                                                                                 pub(crate) Api);

impl<Api: api::Api, const FOD: bool> AsRaw for Bitmap<Api, FOD> {
	type Type = LCDBitmap;
	unsafe fn as_raw(&self) -> *mut LCDBitmap { self.0 }
}

impl<Api: api::Api + Default, const FOD: bool> From<*mut LCDBitmap> for Bitmap<Api, FOD> {
	fn from(ptr: *mut LCDBitmap) -> Self { Self(ptr, Api::default()) }
}

impl<Api: api::Api + Copy> Bitmap<Api, true> {
	/// Convert this bitmap into the same bitmap that will not be freed on drop.
	/// That means that only C-part of the bitmap will __not__ be freed.
	///
	/// __Safety is guaranteed by the caller.__
	pub fn into_shared(mut self) -> Bitmap<Api, false> {
		let res = Bitmap(self.0, self.1);
		self.0 = core::ptr::null_mut();
		res
	}
}


#[repr(transparent)]
pub struct BitmapRef<'owner>(*mut LCDBitmap, api::Default, PhantomData<&'owner ()>);

impl AsRaw for BitmapRef<'_> {
	type Type = LCDBitmap;
	unsafe fn as_raw(&self) -> *mut LCDBitmap { self.0 }
}

impl From<*mut LCDBitmap> for BitmapRef<'_> {
	fn from(ptr: *mut LCDBitmap) -> Self { Self(ptr, Default::default(), PhantomData) }
}

impl<'owner> BitmapRef<'owner> {
	pub fn into_bitmap(self) -> Result<Bitmap<<Self as BitmapApi>::Api, false>, NullPtrError> {
		let ptr = unsafe { self.as_raw() }.ok_or_null()?;
		Ok(Bitmap(ptr, self.api()))
	}

	pub fn into_bitmap_with<Api: api::Api>(self, api: Api) -> Result<Bitmap<Api, false>, NullPtrError> {
		let ptr = unsafe { self.as_raw() }.ok_or_null()?;
		Ok(Bitmap(ptr, api))
	}

	pub fn null() -> Self { Self::from(core::ptr::null_mut()) }
}


impl<Api: api::Api> Bitmap<Api, true> {
	/// Allocates and returns a new `width` by `height` Bitmap filled with `bg` color.
	///
	/// Calls [`sys::ffi::playdate_graphics::newBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::newBitmap")]
	pub fn new(width: c_int, height: c_int, bg: Color) -> Result<Self, Error>
		where Api: Default {
		let api = Api::default();
		Self::new_with(api, width, height, bg)
	}

	/// Allocates and returns a new `width` by `height` Bitmap filled with `bg` color,
	/// using the given `api`.
	///
	/// Calls [`sys::ffi::playdate_graphics::newBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::newBitmap")]
	pub fn new_with(api: Api, width: c_int, height: c_int, bg: Color) -> Result<Self, Error> {
		let f = api.new_bitmap();
		let ptr = unsafe { f(width, height, bg.into()) };
		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Self(ptr, api))
		}
	}


	/// Load a bitmap from a file.
	///
	/// Calls [`sys::ffi::playdate_graphics::loadBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadBitmap")]
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ApiError>
		where Api: Default {
		let api = Api::default();
		Self::load_with(api, path)
	}

	/// Load a bitmap from a file,
	/// create new bitmap with given `api`.
	///
	/// Calls [`sys::ffi::playdate_graphics::loadBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadBitmap")]
	pub fn load_with<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, ApiError> {
		let mut err = Box::new(core::ptr::null() as *const c_char);
		let out_err = Box::into_raw(err);

		let path = CString::new(path.as_ref())?;

		let f = api.load_bitmap();
		let ptr = unsafe { f(path.as_ptr() as *mut c_char, out_err as _) };
		if ptr.is_null() {
			err = unsafe { Box::from_raw(out_err) };
			if let Some(err) = fs::error::Error::from_ptr(*err) {
				Err(Error::Fs(err).into())
			} else {
				Err(Error::Alloc.into())
			}
		} else {
			Ok(Self(ptr, api))
		}
	}
}


impl<Api: api::Api, const FOD: bool> Bitmap<Api, FOD> {
	/// Load a bitmap from a file into `self`.
	///
	/// Calls [`sys::ffi::playdate_graphics::loadIntoBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadIntoBitmap")]
	pub fn load_into<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ApiError> {
		let mut err = Box::new(core::ptr::null() as *const c_char);
		let out_err = Box::into_raw(err);

		let path = CString::new(path.as_ref())?;

		let f = self.1.load_into_bitmap();
		unsafe { f(path.as_ptr() as *mut c_char, self.0, out_err as _) };
		err = unsafe { Box::from_raw(out_err) };
		if let Some(err) = fs::error::Error::from_ptr(*err) {
			Err(Error::Fs(err).into())
		} else {
			Ok(())
		}
	}
}


impl<Api: api::Api, const FOD: bool> Drop for Bitmap<Api, FOD> {
	fn drop(&mut self) {
		if FOD && !self.0.is_null() {
			let f = self.1.free_bitmap();
			unsafe { f(self.0) };
			self.0 = core::ptr::null_mut();
		}
	}
}

impl<Api: api::Api + Clone> Clone for Bitmap<Api, true> {
	/// Allocates and returns a new `Bitmap` that is an exact copy of `self`,
	/// __not a reference__.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::copyBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::copyBitmap")]
	fn clone(&self) -> Self {
		let f = self.1.copy_bitmap();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			panic!("{}: bitmap clone", Error::Alloc)
		} else {
			Self(ptr, self.1.clone())
		}
	}
}


impl<Api: api::Api, const FOD: bool> Bitmap<Api, FOD> {
	/// Clears bitmap, filling with the given `bg` color.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::clearBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::clearBitmap")]
	pub fn clear(&self, bg: Color) {
		let f = self.1.clear_bitmap();
		unsafe { f(self.0, bg.into()) };
	}


	/// Returns `(width, height)` of the bitmap.
	///
	/// Can return error if there is no bitmap-data or any internal error occurred.
	///
	/// Calls [`sys::ffi::playdate_graphics::getBitmapData`].
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapData")]
	pub fn size(&self) -> (c_int, c_int) {
		let mut width: c_int = 0;
		let mut height: c_int = 0;
		let mut row_bytes: c_int = 0;

		let f = self.1.get_bitmap_data();
		unsafe {
			f(
			  self.0,
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
	/// Calls [`sys::ffi::playdate_graphics::getBitmapData`].
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapData")]
	pub fn bitmap_data<'bitmap>(&'bitmap mut self) -> BitmapData<'bitmap> {
		let mut width: c_int = 0;
		let mut height: c_int = 0;
		let mut row_bytes: c_int = 0;


		let mut boxed_data = Box::new(core::ptr::null_mut());
		let data = Box::into_raw(boxed_data);
		let mut boxed_mask = Box::new(core::ptr::null_mut());
		let mask = Box::into_raw(boxed_mask);

		let f = self.1.get_bitmap_data();
		unsafe { f(self.0, &mut width, &mut height, &mut row_bytes, mask, data) };

		let len = row_bytes * height;

		boxed_data = unsafe { Box::from_raw(data) };
		boxed_mask = unsafe { Box::from_raw(mask) };

		// get mask:
		let mask = {
			if !boxed_mask.is_null() && !(*boxed_mask).is_null() {
				let mask = unsafe { core::slice::from_raw_parts_mut::<u8>(*boxed_mask, len as usize) };
				Some(mask)
			} else {
				None
			}
		};

		// get data:
		let data = unsafe { core::slice::from_raw_parts_mut::<u8>(*boxed_data, len as usize) };

		BitmapData { width,
		             height,
		             row_bytes,
		             mask,
		             data }
	}


	/// Sets a mask image for the bitmap.
	/// The set mask must be the same size as the `self` bitmap.
	///
	/// Calls [`sys::ffi::playdate_graphics::setBitmapMask`].
	#[doc(alias = "sys::ffi::playdate_graphics::setBitmapMask")]
	pub fn set_mask<Api2: api::Api, const FREE: bool>(&self, mask: &mut Bitmap<Api2, FREE>) -> Result<(), Error> {
		// TODO: investigate is it correct "res == 0 => Ok"
		let f = self.1.set_bitmap_mask();
		let res = unsafe { f(self.0, mask.0) };
		if res == 0 { Ok(()) } else { Err(Error::InvalidMask) }
	}

	/// Gets a mask image for the given bitmap.
	/// If the image doesn’t have a mask, returns None.
	///
	/// Clones inner api-access.
	///
	/// Calls [`sys::ffi::playdate_graphics::getBitmapMask`].
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapMask")]
	#[inline(always)]
	pub fn mask(&self) -> Option<Bitmap<Api, false>>
		where Api: Clone {
		self.mask_with(self.1.clone())
	}

	/// Gets a mask image for the given bitmap.
	/// If the image doesn’t have a mask, returns None.
	///
	/// Produced `Bitmap` uses passed `api` api-access.
	///
	/// Calls [`sys::ffi::playdate_graphics::getBitmapMask`].
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapMask")]
	// XXX: investigate is it should be free-on-drop?
	pub fn mask_with<NewApi: api::Api>(&self, api: NewApi) -> Option<Bitmap<NewApi, false>> {
		let f = self.1.get_bitmap_mask();
		let ptr = unsafe { f(self.0) };
		if !ptr.is_null() {
			Some(Bitmap(ptr, api))
		} else {
			None
		}
	}

	/// Returns a new, rotated and scaled Bitmap based on the bitmap.
	///
	/// Calls [`sys::ffi::playdate_graphics::rotatedBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::rotatedBitmap")]
	#[inline(always)]
	pub fn rotated_clone(&self,
	                     rotation: c_float,
	                     x_scale: c_float,
	                     y_scale: c_float)
	                     -> Result<Bitmap<Api, true>, Error>
		where Api: Clone
	{
		self.rotated_clone_with(self.1.clone(), rotation, x_scale, y_scale)
	}

	/// Returns a new, rotated and scaled Bitmap based on the bitmap using given `api`.
	///
	/// Calls [`sys::ffi::playdate_graphics::rotatedBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::rotatedBitmap")]
	pub fn rotated_clone_with<NewApi: api::Api>(&self,
	                                            api: NewApi,
	                                            rotation: c_float,
	                                            x_scale: c_float,
	                                            y_scale: c_float)
	                                            -> Result<Bitmap<NewApi, true>, Error>
		where Api: Clone
	{
		let mut alloced_size: c_int = 0;
		let alloced_size_ref = &mut alloced_size;
		let f = self.1.rotated_bitmap();
		let ptr = unsafe { f(self.0, rotation, x_scale, y_scale, alloced_size_ref) };

		if alloced_size == 0 || ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Bitmap(ptr, api))
		}
	}


	/// Draws `self` with its upper-left corner at location `x`, `y`,
	/// using the given `flip` orientation.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawBitmap")]
	#[inline(always)]
	pub fn draw(&self, x: c_int, y: c_int, flip: BitmapFlip) {
		let f = self.1.draw_bitmap();
		unsafe { f(self.0, x, y, flip) }
	}

	/// Draws `self` with its upper-left corner at location `x`, `y`
	/// __tiled inside a `width` by `height` rectangle__.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::tileBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::tileBitmap")]
	#[inline(always)]
	pub fn draw_tiled(&self, x: c_int, y: c_int, width: c_int, height: c_int, flip: BitmapFlip) {
		let f = self.1.tile_bitmap();
		unsafe { f(self.0, x, y, width, height, flip) }
	}

	/// Draws the *bitmap* scaled to `x_scale` and `y_scale`
	/// then rotated by `degrees` with its center as given by proportions `center_x` and `center_y` at `x`, `y`;
	///
	/// that is:
	/// * if `center_x` and `center_y` are both 0.5 the center of the image is at (`x`,`y`),
	/// * if `center_x` and `center_y` are both 0 the top left corner of the image (before rotation) is at (`x`,`y`), etc.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawRotatedBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawRotatedBitmap")]
	#[inline(always)]
	pub fn draw_rotated(&self,
	                    x: c_int,
	                    y: c_int,
	                    degrees: c_float,
	                    center_x: c_float,
	                    center_y: c_float,
	                    x_scale: c_float,
	                    y_scale: c_float) {
		let f = self.1.draw_rotated_bitmap();
		unsafe { f(self.0, x, y, degrees, center_x, center_y, x_scale, y_scale) }
	}

	/// Draws this bitmap scaled to `x_scale` and `y_scale` with its upper-left corner at location `x`, `y`.
	///
	/// Note that flip is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawScaledBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawScaledBitmap")]
	#[inline(always)]
	pub fn draw_scaled(&self, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
		let f = self.1.draw_scaled_bitmap();
		unsafe { f(self.0, x, y, x_scale, y_scale) }
	}


	/// Returns `true` if any of the opaque pixels in this bitmap when positioned at `x, y` with `flip`
	/// overlap any of the opaque pixels in `other` bitmap at `x_other`, `y_other` with `flip_other`
	/// within the non-empty `rect`,
	/// or `false` if no pixels overlap or if one or both fall completely outside of `rect`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::checkMaskCollision`].
	#[doc(alias = "sys::ffi::playdate_graphics::checkMaskCollision")]
	#[inline(always)]
	pub fn check_mask_collision<OApi: api::Api, const OFOD: bool>(&self,
	                                                              x: c_int,
	                                                              y: c_int,
	                                                              flip: BitmapFlip,
	                                                              other: Bitmap<OApi, OFOD>,
	                                                              x_other: c_int,
	                                                              y_other: c_int,
	                                                              flip_other: BitmapFlip,
	                                                              rect: LCDRect)
	                                                              -> bool {
		let f = self.1.check_mask_collision();
		unsafe { f(self.0, x, y, flip, other.0, x_other, y_other, flip_other, rect) == 1 }
	}


	/// Returns pattern `8 x 8` from this bitmap.
	///
	/// `x, y` indicates the top left corner of the 8 x 8 pattern in bitmap's coordinates.
	///
	/// Returned pattern is owned by rust and can be dropped freely.
	///
	/// Uses [`sys::ffi::playdate_graphics::setColorToPattern`].
	#[doc(alias = "sys::ffi::playdate_graphics::setColorToPattern")]
	pub fn pattern_at(&self, x: c_int, y: c_int) -> LCDPattern {
		let mut color = LCDColor::default();
		let f = self.1.set_color_to_pattern();

		unsafe {
			f(core::ptr::addr_of_mut!(color), self.0, x, y);
			*(color as *mut u8 as *mut LCDPattern)
		}
	}

	/// Sets `color` to an `8 x 8` pattern using this bitmap.
	///
	/// `x, y` indicates the top left corner of the 8 x 8 pattern.
	///
	/// After this operation inner pointer is owned by the system.
	/// To get owned pattern use [`Bitmap::pattern_at`].
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setColorToPattern`].
	#[doc(alias = "sys::ffi::playdate_graphics::setColorToPattern")]
	pub fn set_color_to_pattern(&self, color: &mut LCDColor, x: c_int, y: c_int) {
		let f = self.1.set_color_to_pattern();
		unsafe { f(color as _, self.0, x, y) }
	}

	/// Gets the color of the pixel at `(x,y)` in this bitmap.
	/// If the coordinate is outside the bounds of the bitmap,
	/// or if the bitmap has a mask and the pixel is marked transparent,
	/// the function returns [`Clear`][LCDSolidColor::kColorClear];
	/// otherwise the return value is [`White`][LCDSolidColor::kColorWhite] or [`Black`][LCDSolidColor::kColorBlack].
	///
	/// Calls [`sys::ffi::playdate_graphics::getBitmapPixel`].
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapPixel")]
	#[inline(always)]
	pub fn pixel_at(&self, x: c_int, y: c_int) -> LCDSolidColor {
		let f = self.1.get_pixel();
		unsafe { f(self.0, x, y) }
	}
}


/// The data is 1 bit per pixel packed format, in MSB order; in other words,
/// the high bit of the first byte in data is the top left pixel of the image.
///
/// The `mask` data is in same format but means transparency.
pub struct BitmapData<'bitmap> {
	pub width: c_int,
	pub height: c_int,
	pub row_bytes: c_int,
	mask: Option<&'bitmap mut [u8]>,
	data: &'bitmap mut [u8],
}

impl<'bitmap> BitmapData<'bitmap> {
	pub const fn width(&self) -> c_int { self.width }
	pub const fn height(&self) -> c_int { self.height }
	pub const fn row_bytes(&self) -> c_int { self.row_bytes }
	pub fn mask(&self) -> Option<&[u8]> { self.mask.as_deref() }
	pub fn mask_mut(&mut self) -> Option<&mut [u8]> { self.mask.as_deref_mut() }
	pub const fn data(&self) -> &[u8] { self.data }
	pub fn data_mut(&mut self) -> &mut [u8] { self.data }
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
						f.write_char('\n')?;
						f.write_char('\t')?;
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

pub use frame_buffer_bitmap as copy_frame_buffer_bitmap;

#[gen_api_shorthands::gen_shorthands]
impl<Api: crate::api::Api> Graphics<Api> {
	/// Only valid in the Simulator,
	/// returns the debug framebuffer as a bitmap.
	///
	/// Returns error on device.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getDebugBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::getDebugBitmap")]
	pub fn debug_bitmap(&self) -> Result<Bitmap<api::Default, false>, ApiError> {
		let f = self.0.get_debug_bitmap().ok_or_null()?;
		let ptr = unsafe { f() };
		if ptr.is_null() {
			Err(Error::Alloc.into())
		} else {
			Ok(Bitmap(ptr, Default::default()))
		}
	}

	/// Returns a bitmap containing the contents of the display buffer.
	///
	/// __The system owns this bitmap—​do not free it.__
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getDisplayBufferBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::getDisplayBufferBitmap")]
	pub fn display_buffer_bitmap(&self) -> Result<Bitmap<api::Default, false>, Error> {
		let f = self.0.get_display_buffer_bitmap();
		let ptr = unsafe { f() };
		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Bitmap(ptr, Default::default()))
		}
	}

	/// Returns a __copy__ the contents of the working frame buffer as a bitmap.
	///
	/// The caller is responsible for freeing the returned bitmap, it will automatically on drop.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::copyFrameBufferBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::copyFrameBufferBitmap")]
	pub fn frame_buffer_bitmap(&self) -> Result<Bitmap<api::Default, true>, Error> {
		let f = self.0.copy_frame_buffer_bitmap();
		let ptr = unsafe { f() };
		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Bitmap(ptr, Default::default()))
		}
	}


	/// Sets the stencil used for drawing.
	///
	/// If the `tile` is `true` the stencil image will be tiled.
	///
	/// Tiled stencils must have width equal to a multiple of 32 pixels.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setStencilImage`].
	#[doc(alias = "sys::ffi::playdate_graphics::setStencilImage")]
	pub fn set_stencil_tiled(&self, image: &impl AnyBitmap, tile: bool) {
		let f = self.0.set_stencil_image();
		unsafe { f(image.as_raw(), tile as _) };
	}

	/// Sets the stencil used for drawing.
	/// For a tiled stencil, use [`set_stencil_tiled`] instead.
	///
	/// NOTE: Officially deprecated in favor of [`set_stencil_tiled`], which adds a `tile` flag
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setStencil`].
	#[doc(alias = "sys::ffi::playdate_graphics::setStencil")]
	pub fn set_stencil(&self, image: &impl AnyBitmap) {
		let f = self.0.set_stencil();
		unsafe { f(image.as_raw()) };
	}

	/// Sets the mode used for drawing bitmaps.
	///
	/// Returns the previous draw mode.
	///
	/// Note that text drawing uses bitmaps, so this affects how fonts are displayed as well.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::setDrawMode`].
	#[doc(alias = "sys::ffi::playdate_graphics::setDrawMode")]
	pub fn set_draw_mode(&self, mode: BitmapDrawMode) -> BitmapDrawMode {
		let f = self.0.set_draw_mode();
		unsafe { f(mode) }
	}

	/// Push a new drawing context for drawing into the given bitmap.
	///
	/// If `target` is [`BitmapRef::null()`], the drawing functions will use the display framebuffer.
	///
	/// To push framebuffer to context use [`Graphics::push_framebuffer_to_context`].
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::pushContext`].
	#[doc(alias = "sys::ffi::playdate_graphics::pushContext")]
	pub fn push_context(&self, target: &impl AnyBitmap) {
		let f = self.0.push_context();
		unsafe { f(target.as_raw()) };
	}

	/// Push a new drawing context for drawing into the display framebuffer.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::pushContext`].
	#[doc(alias = "sys::ffi::playdate_graphics::pushContext")]
	pub fn push_framebuffer_to_context(&self) {
		let f = self.0.push_context();
		unsafe { f(core::ptr::null_mut()) };
	}

	/// Pops a context off the stack (if any are left),
	/// restoring the drawing settings from before the context was pushed.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::popContext`].
	#[doc(alias = "sys::ffi::playdate_graphics::popContext")]
	pub fn pop_context(&self) {
		let f = self.0.pop_context();
		unsafe { f() };
	}
}


impl<Api: crate::api::Api> Graphics<Api> {
	/// Draws `self` with its upper-left corner at location `x`, `y`,
	/// using the given `flip` orientation.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawBitmap")]
	#[inline(always)]
	pub fn draw(&self, bitmap: &impl AnyBitmap, x: c_int, y: c_int, flip: BitmapFlip) {
		let f = self.0.draw_bitmap();
		unsafe { f(bitmap.as_raw(), x, y, flip) }
	}

	/// Draws `self` with its upper-left corner at location `x`, `y`
	/// __tiled inside a `width` by `height` rectangle__.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::tileBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::tileBitmap")]
	#[inline(always)]
	pub fn draw_tiled(&self,
	                  bitmap: &impl AnyBitmap,
	                  x: c_int,
	                  y: c_int,
	                  width: c_int,
	                  height: c_int,
	                  flip: BitmapFlip) {
		let f = self.0.tile_bitmap();
		unsafe { f(bitmap.as_raw(), x, y, width, height, flip) }
	}

	/// Draws the *bitmap* scaled to `x_scale` and `y_scale`
	/// then rotated by `degrees` with its center as given by proportions `center_x` and `center_y` at `x`, `y`;
	///
	/// that is:
	/// * if `center_x` and `center_y` are both 0.5 the center of the image is at (`x`,`y`),
	/// * if `center_x` and `center_y` are both 0 the top left corner of the image (before rotation) is at (`x`,`y`), etc.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawRotatedBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawRotatedBitmap")]
	#[inline(always)]
	pub fn draw_rotated(&self,
	                    bitmap: &impl AnyBitmap,
	                    x: c_int,
	                    y: c_int,
	                    degrees: c_float,
	                    center_x: c_float,
	                    center_y: c_float,
	                    x_scale: c_float,
	                    y_scale: c_float) {
		let f = self.0.draw_rotated_bitmap();
		unsafe {
			f(
			  bitmap.as_raw(),
			  x,
			  y,
			  degrees,
			  center_x,
			  center_y,
			  x_scale,
			  y_scale,
			)
		}
	}

	/// Draws this bitmap scaled to `x_scale` and `y_scale` with its upper-left corner at location `x`, `y`.
	///
	/// Note that flip is not available when drawing scaled bitmaps but negative scale values will achieve the same effect.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::drawScaledBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::drawScaledBitmap")]
	#[inline(always)]
	pub fn draw_scaled(&self, bitmap: &impl AnyBitmap, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
		let f = self.0.draw_scaled_bitmap();
		unsafe { f(bitmap.as_raw(), x, y, x_scale, y_scale) }
	}
}
