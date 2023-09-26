use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;

use sys::ffi::LCDColor;
use sys::ffi::LCDBitmap;
use sys::ffi::LCDBitmapFlip;
use sys::ffi::LCDRect;


#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;
impl Api for Default {}


/// End-point with methods about ops over bitmap.
pub trait Api {
	/// Equivalent to [`sys::ffi::playdate_graphics::newBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::newBitmap")]
	fn new_bitmap(&self)
	              -> unsafe extern "C" fn(width: c_int, height: c_int, bgcolor: LCDColor) -> *mut LCDBitmap {
		*sys::api!(graphics.newBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::freeBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::freeBitmap")]
	fn free_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) { *sys::api!(graphics.freeBitmap) }

	/// Equivalent to [`sys::ffi::playdate_graphics::loadBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::loadBitmap")]
	fn load_bitmap(&self)
	               -> unsafe extern "C" fn(path: *const c_char, outerr: *mut *const c_char) -> *mut LCDBitmap {
		*sys::api!(graphics.loadBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::copyBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::copyBitmap")]
	fn copy_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
		*sys::api!(graphics.copyBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::loadIntoBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::loadIntoBitmap")]
	fn load_into_bitmap(
		&self)
		-> unsafe extern "C" fn(path: *const c_char, bitmap: *mut LCDBitmap, out_err: *mut *const c_char) {
		*sys::api!(graphics.loadIntoBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getBitmapData`]
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapData")]
	fn get_bitmap_data(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        width: *mut c_int,
		                        height: *mut c_int,
		                        row_bytes: *mut c_int,
		                        mask: *mut *mut u8,
		                        data: *mut *mut u8) {
		*sys::api!(graphics.getBitmapData)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::clearBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::clearBitmap")]
	fn clear_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, bgcolor: LCDColor) {
		*sys::api!(graphics.clearBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::rotatedBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::rotatedBitmap")]
	fn rotated_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        rotation: c_float,
		                        x_scale: c_float,
		                        y_scale: c_float,
		                        allocedSize: *mut c_int) -> *mut LCDBitmap {
		*sys::api!(graphics.rotatedBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setBitmapMask`]
	#[doc(alias = "sys::ffi::playdate_graphics::setBitmapMask")]
	fn set_bitmap_mask(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, mask: *mut LCDBitmap) -> c_int {
		*sys::api!(graphics.setBitmapMask)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::getBitmapMask`]
	#[doc(alias = "sys::ffi::playdate_graphics::getBitmapMask")]
	fn get_bitmap_mask(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
		*sys::api!(graphics.getBitmapMask)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawBitmap")]
	fn draw_bitmap(&self)
	               -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, x: c_int, y: c_int, flip: LCDBitmapFlip) {
		*sys::api!(graphics.drawBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::tileBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::tileBitmap")]
	fn tile_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        x: c_int,
		                        y: c_int,
		                        width: c_int,
		                        height: c_int,
		                        flip: LCDBitmapFlip) {
		*sys::api!(graphics.tileBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawRotatedBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawRotatedBitmap")]
	fn draw_rotated_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        x: c_int,
		                        y: c_int,
		                        rotation: c_float,
		                        center_x: c_float,
		                        center_y: c_float,
		                        x_scale: c_float,
		                        y_scale: c_float) {
		*sys::api!(graphics.drawRotatedBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::drawScaledBitmap`]
	#[doc(alias = "sys::ffi::playdate_graphics::drawScaledBitmap")]
	fn draw_scaled_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
		*sys::api!(graphics.drawScaledBitmap)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::checkMaskCollision`]
	#[doc(alias = "sys::ffi::playdate_graphics::checkMaskCollision")]
	fn check_mask_collision(
		&self)
		-> unsafe extern "C" fn(bitmap1: *mut LCDBitmap,
		                        x1: c_int,
		                        y1: c_int,
		                        flip1: LCDBitmapFlip,
		                        bitmap2: *mut LCDBitmap,
		                        x2: c_int,
		                        y2: c_int,
		                        flip2: LCDBitmapFlip,
		                        rect: LCDRect) -> c_int {
		*sys::api!(graphics.checkMaskCollision)
	}

	/// Equivalent to [`sys::ffi::playdate_graphics::setColorToPattern`]
	#[doc(alias = "sys::ffi::playdate_graphics::setColorToPattern")]
	fn set_color_to_pattern(
		&self)
		-> unsafe extern "C" fn(color: *mut LCDColor,
		                        bitmap: *mut LCDBitmap,
		                        x: core::ffi::c_int,
		                        y: core::ffi::c_int) {
		*sys::api!(graphics.setColorToPattern)
	}
}
