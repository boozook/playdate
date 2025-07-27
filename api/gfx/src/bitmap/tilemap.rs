//! Playdate tilemap API

use core::ffi::c_float;
use core::ffi::c_int;
use core::ptr::NonNull;

use sys::ffi::TileMap as SysTileMap;
use sys::macros::api_opt;
use sys::utils::AsRaw;

use crate::error;
use super::table::BitmapTable;


type Api = &'static sys::ffi::PlaydateTilemap;


#[must_use]
#[repr(transparent)]
pub struct TileMap(NonNull<SysTileMap>);

impl TileMap {
	pub const unsafe fn from_ptr(ptr: NonNull<SysTileMap>) -> Self { Self(ptr) }
}

impl const AsRaw for TileMap {
	type Output = SysTileMap;
	#[inline]
	unsafe fn as_raw(&self) -> NonNull<Self::Output> { self.0 }
}


impl Drop for TileMap {
	fn drop(&mut self) {
		if let Some(f) = api_opt!(graphics.tilemap.freeTilemap) {
			unsafe { f(self.0.as_ptr()) };
		}
	}
}


pub mod borrow {
	use core::marker::PhantomData;
	use core::mem::ManuallyDrop;
	use core::ops::Deref;
	use core::ops::DerefMut;

	use super::*;
	use crate::{AsRef, AsMut};


	#[must_use]
	#[repr(transparent)]
	pub struct Borrowed<'owner>(ManuallyDrop<TileMap>, PhantomData<&'owner ()>);

	impl Borrowed<'_> {
		pub const fn from_ptr(ptr: NonNull<SysTileMap>) -> Self {
			Self(ManuallyDrop::new(TileMap(ptr)), PhantomData)
		}
	}

	impl<'o> const AsRef<'o, TileMap> for Borrowed<'o> where ManuallyDrop<TileMap>: [const] Deref {
		fn as_ref<'t>(&'t self) -> &'t TileMap
			where 'o: 't {
			&self.0
		}
	}
	impl<'o> const AsMut<'o, TileMap> for Borrowed<'o> where ManuallyDrop<TileMap>: [const] DerefMut {
		fn as_mut<'t>(&'t mut self) -> &'t mut TileMap
			where 'o: 't {
			&mut self.0
		}
	}

	impl<'t, 'l> const Deref for Borrowed<'t> where Self: [const] AsRef<'t, TileMap> {
		type Target = TileMap;
		fn deref(&self) -> &Self::Target { self.as_ref() }
	}
	impl<'t, 'l> const DerefMut for Borrowed<'t>
		where Self: [const] AsMut<'t, TileMap> + [const] Deref<Target = TileMap>
	{
		fn deref_mut(&mut self) -> &mut Self::Target { self.as_mut() }
	}
}


impl TileMap {
	/// Creates a new, empty [`TileMap`] object.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::newTilemap`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::newTilemap")]
	pub fn new(api: Api) -> Result<Self, error::Alloc> {
		let ptr = unsafe { (api.newTilemap)() };
		if ptr.is_null() {
			Err(error::Alloc)
		} else {
			Ok(Self(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Sets the image table to use for the tilemap’s tiles.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::setImageTable`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::setImageTable")]
	pub fn set_image_table(&mut self, api: Api, table: &mut BitmapTable) {
		unsafe { (api.setImageTable)(self.0.as_ptr(), table.0.as_ptr()) }
	}

	/// Returns used for the tilemap’s tiles.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::getImageTable`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::getImageTable")]
	pub fn image_table(&mut self, api: Api) -> Option<BitmapTable> {
		let ptr = unsafe { (api.getImageTable)(self.0.as_ptr()) };
		if ptr.is_null() {
			None
		} else {
			Some(BitmapTable(unsafe { NonNull::new_unchecked(ptr) }))
		}
	}


	/// Sets the tilemap’s width and height, in number of tiles.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::setSize`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::setSize")]
	pub fn set_size(&mut self, api: Api, tiles_wide: c_int, tiles_high: c_int) {
		unsafe { (api.setSize)(self.0.as_ptr(), tiles_wide, tiles_high) }
	}

	/// Returns the size of the tile map, in tiles as `(tiles_wide, tiles_high)`.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::getSize`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::getSize")]
	pub fn size(&mut self, api: Api) -> (c_int, c_int) {
		let mut w = 0;
		let mut h = 0;
		self.size_to(api, &mut w, &mut h);
		(w, h)
	}

	/// Writes the size of the tile map, in tiles to the given `tiles_wide` and `tiles_high`.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::getSize`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::getSize")]
	pub fn size_to(&mut self, api: Api, tiles_wide: &mut c_int, tiles_high: &mut c_int) {
		unsafe { (api.getSize)(self.0.as_ptr(), tiles_wide, tiles_high) }
	}


	/// Returns the size of the tilemap in pixels;
	/// that is, the size of the tile image multiplied by the number of rows and columns in the tilemap.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::getPixelSize`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::getPixelSize")]
	pub fn pixel_size(&mut self, api: Api) -> (u32, u32) {
		let mut w = 0;
		let mut h = 0;
		self.pixel_size_to(api, &mut w, &mut h);
		(w, h)
	}

	/// Writes the size of the tilemap in pixels;
	/// that is, the size of the tile image multiplied by the number of rows and columns in the tilemap.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::getPixelSize`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::getPixelSize")]
	pub fn pixel_size_to(&mut self, api: Api, width: &mut u32, height: &mut u32) {
		unsafe { (api.getPixelSize)(self.0.as_ptr(), width, height) }
	}


	/// Sets the tilemap’s width to `row_width` and height to `indexes.len()`/`row_width`
	/// (len of indexes must be evenly divisible by `row_width`),
	/// then sets the tiles' indexes to the given list.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::setTiles`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::setTiles")]
	pub fn set_tiles(&mut self, api: Api, indexes: &mut [u16], row_width: c_int) {
		let count = indexes.len() as _;
		let indexes = indexes.as_mut_ptr();
		unsafe { (api.setTiles)(self.0.as_ptr(), indexes, count, row_width) }
	}


	/// Sets the index of the tile at tilemap position `(x, y)`.
	///
	/// Index is the (0-based) index of the cell in the tilemap’s image table.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::setTileAtPosition`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::setTileAtPosition")]
	pub fn set_tile_at(&mut self, api: Api, x: c_int, y: c_int, index: u16) {
		unsafe { (api.setTileAtPosition)(self.0.as_ptr(), x, y, index) }
	}

	/// Returns the image index of the tile at the given `x` and `y` coordinate.
	/// If `x` or `y` is out of bounds, returns `-1`.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::getTileAtPosition`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::getTileAtPosition")]
	pub fn tile_at(&mut self, api: Api, x: c_int, y: c_int) -> c_int {
		unsafe { (api.getTileAtPosition)(self.0.as_ptr(), x, y) }
	}


	/// Draws the tile map at coordinate `(x, y)`.
	///
	/// Equivalent to [`sys::ffi::PlaydateTilemap::drawAtPoint`].
	#[doc(alias = "sys::ffi::PlaydateTilemap::drawAtPoint")]
	pub fn draw(&mut self, api: Api, x: c_float, y: c_float) {
		unsafe { (api.drawAtPoint)(self.0.as_ptr(), x, y) }
	}
}


pub mod api {
	use core::ops::Deref;

	use crate::Graphics;
	use super::Api;


	impl Graphics {
		pub const fn tilemap(&self) -> TileMap { TileMap(self.0.tilemap) }
	}


	#[derive(Clone, Copy)]
	pub struct TileMap(Api);

	impl Deref for TileMap {
		type Target = Api;
		fn deref(&self) -> &Self::Target { &self.0 }
	}

	impl Default for TileMap {
		fn default() -> Self { Self(sys::macros::api!(graphics.tilemap)) }
	}
}
