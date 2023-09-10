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


pub trait Api {
	fn new_bitmap(&self)
	              -> unsafe extern "C" fn(width: c_int, height: c_int, bgcolor: LCDColor) -> *mut LCDBitmap {
		*sys::api!(graphics.newBitmap)
	}

	fn free_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) { *sys::api!(graphics.freeBitmap) }


	fn load_bitmap(&self)
	               -> unsafe extern "C" fn(path: *const c_char, outerr: *mut *const c_char) -> *mut LCDBitmap {
		*sys::api!(graphics.loadBitmap)
	}

	fn copy_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
		*sys::api!(graphics.copyBitmap)
	}

	fn load_into_bitmap(
		&self)
		-> unsafe extern "C" fn(path: *const c_char, bitmap: *mut LCDBitmap, out_err: *mut *const c_char) {
		*sys::api!(graphics.loadIntoBitmap)
	}


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


	fn clear_bitmap(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, bgcolor: LCDColor) {
		*sys::api!(graphics.clearBitmap)
	}

	fn rotated_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap,
		                        rotation: c_float,
		                        x_scale: c_float,
		                        y_scale: c_float,
		                        allocedSize: *mut c_int) -> *mut LCDBitmap {
		*sys::api!(graphics.rotatedBitmap)
	}

	fn set_bitmap_mask(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, mask: *mut LCDBitmap) -> c_int {
		*sys::api!(graphics.setBitmapMask)
	}

	fn get_bitmap_mask(&self) -> unsafe extern "C" fn(bitmap: *mut LCDBitmap) -> *mut LCDBitmap {
		*sys::api!(graphics.getBitmapMask)
	}

	fn draw_bitmap(&self)
	               -> unsafe extern "C" fn(bitmap: *mut LCDBitmap, x: c_int, y: c_int, flip: LCDBitmapFlip) {
		*sys::api!(graphics.drawBitmap)
	}

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

	fn draw_scaled_bitmap(
		&self)
		-> unsafe extern "C" fn(bitmap: *mut LCDBitmap, x: c_int, y: c_int, x_scale: c_float, y_scale: c_float) {
		*sys::api!(graphics.drawScaledBitmap)
	}

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

	fn set_color_to_pattern(
		&self)
		-> unsafe extern "C" fn(color: *mut LCDColor,
		                        bitmap: *mut LCDBitmap,
		                        x: core::ffi::c_int,
		                        y: core::ffi::c_int) {
		*sys::api!(graphics.setColorToPattern)
	}
}
