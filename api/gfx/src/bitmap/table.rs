//! Playdate bitmap-table API

use core::ffi::c_char;
use core::ffi::c_int;
use core::ptr::null_mut;
use core::ptr::NonNull;

use sys::ffi::BitmapTable as SysBitmapTable;
use sys::macros::api_opt;
use fs::path::Path;

use crate::error;
use crate::Api;
use super::Bitmap;


#[must_use]
#[repr(transparent)]
pub struct BitmapTable(pub(super) NonNull<SysBitmapTable>);

impl Drop for BitmapTable {
	fn drop(&mut self) {
		if let Some(f) = api_opt!(graphics.freeBitmapTable) {
			unsafe { f(self.0.as_ptr()) };
		}
	}
}


impl BitmapTable {
	/// Allocates and returns a new [`BitmapTable`] that can hold count `width` by `height` [`Bitmap`]s.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::newBitmapTable`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::newBitmapTable")]
	pub fn new(api: Api, count: c_int, width: c_int, height: c_int) -> Result<Self, error::Alloc> {
		let ptr = unsafe { (api.newBitmapTable)(count, width, height) };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(Self(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Allocates and returns a new [`BitmapTable`] from the file at `path`.
	///
	/// If there is no file at `path`, the function returns error.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::loadBitmapTable`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::loadBitmapTable")]
	pub fn load<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, error::LoadError> {
		let path = path.as_ref();
		let mut err: *const c_char = core::ptr::null();

		let ptr = unsafe { (api.loadBitmapTable)(path.as_ptr(), &raw mut err) };

		if ptr.is_null() {
			if let Some(err) = unsafe { fs::error::Owned::from_ptr(err) } {
				Err(error::LoadError::Fs(err))
			} else {
				Err(error::LoadError::Alloc(error::Alloc))
			}
		} else {
			Ok(Self(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Loads the image-table at `path` into the previously allocated this table.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::loadIntoBitmapTable`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::loadIntoBitmapTable")]
	#[must_use = "Error message is borrowed from C part, must be used immediately or converted to owned string."]
	pub fn load_into<'t, P: AsRef<Path>>(&'t mut self, api: Api, path: P) -> Result<(), fs::error::Borrowed<'t>> {
		let path = path.as_ref();
		let mut err: *const c_char = core::ptr::null();

		unsafe { (api.loadIntoBitmapTable)(path.as_ptr(), self.0.as_ptr(), &raw mut err) };

		if let Some(err) = unsafe { fs::error::Error::from_ptr(err) } {
			Err(err)
		} else {
			Ok(())
		}
	}


	/// Returns the `index` bitmap in this table,
	/// if `index` is out of bounds, the function returns `None`.
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getTableBitmap`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getTableBitmap")]
	pub fn get(&self, api: Api, index: c_int) -> Option<Bitmap> {
		let ptr = unsafe { (api.getTableBitmap)(self.0.as_ptr(), index) };
		if ptr.is_null() {
			None
		} else {
			Some(Bitmap(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}

	/// Returns `(count, width)` - the bitmap table’s image count
	/// and number of cells across in the `width`.
	///
	/// Calls [`sys::ffi::PlaydateGraphics::getBitmapTableInfo`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapTableInfo")]
	pub fn info(&self, api: Api) -> (c_int, c_int) {
		let mut count = 0;
		let mut width = 0;
		self.info_to(api, &mut count, &mut width);
		(count, width)
	}

	/// Returns the bitmap table’s image count in the `count` if not `None`
	/// and number of cells across in the `width` (ditto) if not `None` .
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getBitmapTableInfo`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapTableInfo")]
	pub fn info_to(&self, api: Api, count: &mut c_int, width: &mut c_int) {
		unsafe { (api.getBitmapTableInfo)(self.0.as_ptr(), count, width) }
	}

	/// Returns the bitmap table’s image count in the `count` if not `None`
	/// and number of cells across in the `width` (ditto) if not `None` .
	///
	/// Equivalent to [`sys::ffi::PlaydateGraphics::getBitmapTableInfo`].
	#[doc(alias = "sys::ffi::PlaydateGraphics::getBitmapTableInfo")]
	pub fn info_to_opt(&self, api: Api, count: Option<&mut c_int>, width: Option<&mut c_int>) {
		unsafe {
			(api.getBitmapTableInfo)(
			                         self.0.as_ptr(),
			                         count.map_or(null_mut(), |v| v),
			                         width.map_or(null_mut(), |v| v),
			)
		}
	}
}
