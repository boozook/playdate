//! Playdate bitmap-table API

use alloc::boxed::Box;
use core::ffi::c_char;
use core::ffi::c_int;

use sys::ffi::CString;
use sys::ffi::LCDBitmapTable;
use fs::Path;

use crate::error::ApiError;
use crate::error::Error;
use super::Bitmap;
use super::api::Api as BitmapApi;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct BitmapTable<Api: api::Api = api::Default, const FREE_ON_DROP: bool = true>(*mut LCDBitmapTable, Api);

impl<Api: api::Api, const FOD: bool> Drop for BitmapTable<Api, FOD> {
	fn drop(&mut self) {
		if FOD && !self.0.is_null() {
			let f = self.1.free_bitmap_table();
			unsafe { f(self.0) };
			self.0 = core::ptr::null_mut();
		}
	}
}


impl<Api: api::Api> BitmapTable<Api, true> {
	/// Allocates and returns a new [`BitmapTable`] that can hold count `width` by `height` [`Bitmap`]s.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::newBitmapTable`].
	#[doc(alias = "sys::ffi::playdate_graphics::newBitmapTable")]
	pub fn new(count: c_int, width: c_int, height: c_int) -> Result<Self, Error>
		where Api: Default {
		let api = Api::default();
		Self::new_with(api, count, width, height)
	}

	/// Allocates and returns a new [`BitmapTable`] that can hold count `width` by `height` [`Bitmap`]s,
	/// using the given `api`.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::newBitmapTable`].
	#[doc(alias = "sys::ffi::playdate_graphics::newBitmapTable")]
	pub fn new_with(api: Api, count: c_int, width: c_int, height: c_int) -> Result<Self, Error> {
		let f = api.new_bitmap_table();
		let ptr = unsafe { f(count, width, height) };
		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Self(ptr, api))
		}
	}


	/// Allocates and returns a new [`BitmapTable`] from the file at `path`.
	///
	/// If there is no file at `path`, the function returns error.
	///
	/// Calls [`sys::ffi::playdate_graphics::loadBitmapTable`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadBitmapTable")]
	pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, ApiError>
		where Api: Default {
		let api = Api::default();
		Self::load_with(api, path)
	}

	/// Allocates and returns a new [`BitmapTable`] from the file at `path`.
	///
	/// If there is no file at `path`, the function returns error.
	///
	/// Calls [`sys::ffi::playdate_graphics::loadBitmapTable`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadBitmapTable")]
	pub fn load_with<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, ApiError> {
		let mut err = Box::new(core::ptr::null() as *const c_char);
		let out_err = Box::into_raw(err);

		let path = CString::new(path.as_ref())?;

		let f = api.load_bitmap_table();
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

impl<Api: api::Api, const FOD: bool> BitmapTable<Api, FOD> {
	/// Loads the image-table at `path` into the previously allocated this table.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::loadIntoBitmapTable`].
	#[doc(alias = "sys::ffi::playdate_graphics::loadIntoBitmapTable")]
	pub fn load_into<P: AsRef<Path>>(&mut self, path: P) -> Result<(), ApiError> {
		let mut err = Box::new(core::ptr::null() as *const c_char);
		let out_err = Box::into_raw(err);

		let path = CString::new(path.as_ref())?;

		let f = self.1.load_into_bitmap_table();
		unsafe { f(path.as_ptr() as *mut c_char, self.0, out_err as _) };
		err = unsafe { Box::from_raw(out_err) };
		if let Some(err) = fs::error::Error::from_ptr(*err) {
			Err(Error::Fs(err).into())
		} else {
			Ok(())
		}
	}


	/// Returns the `index` bitmap in this table,
	/// if `index` is out of bounds, the function returns `None`.
	///
	/// Creates new default api access-point.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getTableBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::getTableBitmap")]
	pub fn get<'table, BitApi: BitmapApi>(&'table self, index: c_int) -> Option<Bitmap<BitApi, true>>
		where Bitmap<BitApi, true>: 'table,
		      BitApi: Default {
		self.get_with(BitApi::default(), index)
	}

	/// Returns the `index` bitmap in this table,
	/// if `index` is out of bounds, the function returns `None`.
	///
	/// Produced `Bitmap` uses passed `api` access-point.
	///
	/// Equivalent to [`sys::ffi::playdate_graphics::getTableBitmap`].
	#[doc(alias = "sys::ffi::playdate_graphics::getTableBitmap")]
	pub fn get_with<'table, BitApi: BitmapApi>(&'table self,
	                                           api: BitApi,
	                                           index: c_int)
	                                           -> Option<Bitmap<BitApi, true>>
		where Bitmap<BitApi, true>: 'table
	{
		let f = self.1.get_table_bitmap();
		let ptr = unsafe { f(self.0, index) };
		if ptr.is_null() {
			None
		} else {
			Some(Bitmap(ptr, api))
		}
	}
}


pub mod api {
	use core::ffi::c_char;
	use core::ffi::c_int;
	use sys::ffi::LCDBitmap;
	use sys::ffi::LCDBitmapTable;


	/// Default graphics bitmap table api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	pub type Default = crate::api::Default;

	/// Cached graphics bitmap table api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	pub type Cache = crate::api::Cache;


	/// End-point with methods about ops over bitmap-table.
	pub trait Api {
		/// Equivalent to [`sys::ffi::playdate_graphics::newBitmapTable`]
		#[doc(alias = "sys::ffi::playdate_graphics::newBitmapTable")]
		fn new_bitmap_table(
			&self)
			-> unsafe extern "C" fn(count: c_int, width: c_int, height: c_int) -> *mut LCDBitmapTable {
			*sys::api!(graphics.newBitmapTable)
		}


		/// Equivalent to [`sys::ffi::playdate_graphics::freeBitmapTable`]
		#[doc(alias = "sys::ffi::playdate_graphics::freeBitmapTable")]
		fn free_bitmap_table(&self) -> unsafe extern "C" fn(table: *mut LCDBitmapTable) {
			*sys::api!(graphics.freeBitmapTable)
		}


		/// Equivalent to [`sys::ffi::playdate_graphics::loadBitmapTable`]
		#[doc(alias = "sys::ffi::playdate_graphics::loadBitmapTable")]
		fn load_bitmap_table(
			&self)
			-> unsafe extern "C" fn(path: *const c_char, out_err: *mut *const c_char) -> *mut LCDBitmapTable {
			*sys::api!(graphics.loadBitmapTable)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::loadIntoBitmapTable`]
		#[doc(alias = "sys::ffi::playdate_graphics::loadIntoBitmapTable")]
		fn load_into_bitmap_table(
			&self)
			-> unsafe extern "C" fn(path: *const c_char, table: *mut LCDBitmapTable, out_err: *mut *const c_char) {
			*sys::api!(graphics.loadIntoBitmapTable)
		}

		/// Equivalent to [`sys::ffi::playdate_graphics::getTableBitmap`]
		#[doc(alias = "sys::ffi::playdate_graphics::getTableBitmap")]
		fn get_table_bitmap(&self)
		                    -> unsafe extern "C" fn(table: *mut LCDBitmapTable, idx: c_int) -> *mut LCDBitmap {
			*sys::api!(graphics.getTableBitmap)
		}
	}
}
