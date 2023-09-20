//! Basic Sprite implementations.

use core::ffi::c_int;
use core::ffi::c_void;
use core::ffi::c_float;
use core::marker::PhantomData;
use alloc::boxed::Box;

use sys::traits::AsRaw;

use sys::ffi::SpriteCollisionInfo;
use sys::ffi::LCDRect;
use sys::ffi::LCDSprite;
use sys::ffi::PDRect;

use gfx::bitmap::AnyBitmap;
use gfx::bitmap::BitmapRef;
use gfx::bitmap::BitmapDrawMode;
use gfx::bitmap::BitmapFlip;

use crate::AnySprite;
use crate::SpriteApi;
use crate::TypedSprite;
use crate::api;


pub type OwnedSprite<Userdata, Api> = Sprite<Userdata, Api, true>;
pub type SharedSprite<Userdata, Api> = Sprite<Userdata, Api, false>;


impl<UD, Api: api::Api, const FOD: bool> TypedSprite for Sprite<UD, Api, FOD> {
	type Userdata = UD;
	const FREE_ON_DROP: bool = FOD;
}


impl AnySprite for SpriteRef {}
impl<UD, Api: api::Api, const FOD: bool> AnySprite for Sprite<UD, Api, FOD> {}


impl SpriteApi for SpriteRef {
	type Api = api::Default;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		api::Default::default()
	}

	fn api_ref(&self) -> &Self::Api {
		static API: api::Default = api::Default;
		&API
	}
}

impl<UD, Api: api::Api, const FOD: bool> SpriteApi for Sprite<UD, Api, FOD> {
	type Api = Api;
	fn api(&self) -> Api
		where Self::Api: Copy {
		self.1
	}

	fn api_ref(&self) -> &Self::Api { &self.1 }
}


#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct SpriteRef(*mut LCDSprite);

impl From<*mut LCDSprite> for SpriteRef {
	fn from(ptr: *mut LCDSprite) -> Self { Self(ptr) }
}

impl AsRaw for SpriteRef {
	type Type = LCDSprite;
	unsafe fn as_raw(&self) -> *mut LCDSprite { self.0 }
}

impl SpriteRef {
	pub fn into_sprite<UD>(self) -> Sprite<UD, <Self as SpriteApi>::Api, false> {
		Sprite(unsafe { self.as_raw() }, self.api(), PhantomData)
	}

	pub fn into_sprite_with<UD, Api: api::Api>(self, api: Api) -> Sprite<UD, Api, false> {
		Sprite(unsafe { self.as_raw() }, api, PhantomData)
	}
}


#[derive(Debug)]
pub struct Sprite<Userdata = (), Api: api::Api = api::Default, const FREE_ON_DROP: bool = true>(*mut LCDSprite, Api, PhantomData<Userdata>);

impl<UD, Api: api::Api, const FOD: bool> AsRaw for Sprite<UD, Api, FOD> {
	type Type = LCDSprite;
	unsafe fn as_raw(&self) -> *mut LCDSprite { self.0 }
}

impl<UD, Api: api::Api, const FOD: bool> AsRef<Self> for Sprite<UD, Api, FOD> {
	fn as_ref(&self) -> &Self { self }
}
impl<UD, Api: api::Api, const FOD: bool> AsMut<Self> for Sprite<UD, Api, FOD> {
	fn as_mut(&mut self) -> &mut Self { self }
}

impl<UD, Api, const FOD: bool> From<SpriteRef> for Sprite<UD, Api, FOD> where Api: api::Api + Default {
	fn from(sprite: SpriteRef) -> Self { Self(unsafe { sprite.as_raw() }, Api::default(), PhantomData) }
}

impl<UD, Api: api::Api + Clone, const FOD: bool> Clone for Sprite<UD, Api, FOD> {
	fn clone(&self) -> Self {
		let f = self.1.copy();
		let ptr = unsafe { f(self.0) };
		Self(ptr, self.1.clone(), PhantomData)
	}
}

impl<UD, Api: api::Api, const FOD: bool> Drop for Sprite<UD, Api, FOD> {
	fn drop(&mut self) {
		if FOD && !self.0.is_null() {
			if let Some(ud) = self.take_userdata() {
				drop(ud);
				let f = self.1.set_userdata();
				unsafe { f(self.0, core::ptr::null_mut()) }
			}

			let f = self.1.free_sprite();
			unsafe { f(self.0) }
			self.0 = core::ptr::null_mut();
		}
	}
}

impl<UD, Api: api::Api + Copy> Sprite<UD, Api, true> {
	/// Convert this sprite into the same sprite that will not be freed on drop.
	/// That means that only C-part of the sprite will be freed.
	///
	/// __Safety is guaranteed by the caller.__
	pub fn into_shared(mut self) -> Sprite<UD, Api, false> {
		let res = Sprite(self.0, self.1, self.2);
		self.0 = core::ptr::null_mut();
		res
	}
}


impl<UD, Api: Default + api::Api, const FOD: bool> Sprite<UD, Api, FOD> {
	/// Allocates and returns a new Sprite with [`default`](api::Default) api access-point.
	///
	/// To create a sprite with a custom api access-point, use [`with_api`](Sprite::with_api).
	///
	/// See also [`sys::ffi::playdate_sprite::newSprite`]
	#[doc(alias = "sys::ffi::playdate_sprite::newSprite")]
	pub fn new() -> Self {
		let api = Default::default();
		Self::new_with(api)
	}
}

impl<UD, Api: api::Api, const FOD: bool> Sprite<UD, Api, FOD> {
	/// Allocates and returns a new Sprite with given `api`.
	///
	/// See also [`sys::ffi::playdate_sprite::newSprite`]
	#[doc(alias = "sys::ffi::playdate_sprite::newSprite")]
	pub fn new_with(api: Api) -> Self {
		let f = api.new_sprite();
		let ptr = unsafe { f() };
		Self(ptr, api, PhantomData)
	}
}


impl<Userdata, Api: api::Api, const FOD: bool> Sprite<Userdata, Api, FOD> {
	/// Adds the this sprite to the display list, so that it is drawn in the current scene.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::addSprite`]
	#[doc(alias = "sys::ffi::playdate_sprite::addSprite")]
	pub fn add(&self) {
		let f = self.1.add_sprite();
		unsafe { f(self.0) }
	}

	/// Removes the this sprite from the display list.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::removeSprite`]
	#[doc(alias = "sys::ffi::playdate_sprite::removeSprite")]
	pub fn remove(&self) {
		let f = self.1.remove_sprite();
		unsafe { f(self.0) }
	}


	/// Sets the bounds of the sprite with bounds.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setBounds`]
	#[doc(alias = "sys::ffi::playdate_sprite::setBounds")]
	pub fn set_bounds(&self, bounds: PDRect) {
		let f = self.1.set_bounds();
		unsafe { f(self.0, bounds) }
	}

	/// Returns the bounds of the sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getBounds`]
	#[doc(alias = "sys::ffi::playdate_sprite::getBounds")]
	pub fn bounds(&self) -> PDRect {
		let f = self.1.get_bounds();
		unsafe { f(self.0) }
	}


	/// Moves the sprite to `x`, `y` and resets its bounds based on the bitmap dimensions and center.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::moveTo`]
	#[doc(alias = "sys::ffi::playdate_sprite::moveTo")]
	pub fn move_to(&self, x: c_float, y: c_float) {
		let f = self.1.move_to();
		unsafe { f(self.0, x, y) }
	}

	/// Moves the sprite to by offsetting its current position by `dx`, `dy`.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::moveBy`]
	#[doc(alias = "sys::ffi::playdate_sprite::moveBy")]
	pub fn move_by(&self, dx: c_float, dy: c_float) {
		let f = self.1.move_by();
		unsafe { f(self.0, dx, dy) }
	}


	/// Sets the sprite's image to the given bitmap.
	///
	/// ⚠️ Caution: Using with draw function, call this method __before__ set callback.
	/// Setting image __after__ setting draw callback is mostly crashes with SIGBUS.
	///
	/// See also [`set_opaque`].
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setImage`]
	#[doc(alias = "sys::ffi::playdate_sprite::setImage")]
	pub fn set_image(&self, image: impl AnyBitmap, flip: BitmapFlip) {
		let f = self.1.set_image();
		unsafe { f(self.0, image.as_raw(), flip) }
	}

	/// Returns the bitmap currently assigned to the given sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getImage`]
	#[doc(alias = "sys::ffi::playdate_sprite::getImage")]
	pub fn image<'t>(&'t self) -> Option<BitmapRef<'t>> {
		let f = self.1.get_image();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			None
		} else {
			Some(BitmapRef::from(ptr))
		}
	}


	/// Sets the size.
	/// The size is used to set the sprite’s bounds when calling [`move_to`].
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setSize`]
	#[doc(alias = "sys::ffi::playdate_sprite::setSize")]
	pub fn set_size(&self, width: c_float, height: c_float) {
		let f = self.1.set_size();
		unsafe { f(self.0, width, height) }
	}


	/// Sets the Z order of the sprite.
	/// Higher Z sprites are drawn on top of those with lower Z order.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setZIndex`]
	#[doc(alias = "sys::ffi::playdate_sprite::setZIndex")]
	pub fn set_z_index(&self, z_index: i16) {
		let f = self.1.set_z_index();
		unsafe { f(self.0, z_index) }
	}

	/// Returns the Z index of the sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getZIndex`]
	#[doc(alias = "sys::ffi::playdate_sprite::getZIndex")]
	pub fn z_index(&self) -> i16 {
		let f = self.1.get_z_index();
		unsafe { f(self.0) }
	}


	/// Sets the mode for drawing the sprite’s bitmap.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setDrawMode`]
	#[doc(alias = "sys::ffi::playdate_sprite::setDrawMode")]
	pub fn set_draw_mode(&self, mode: BitmapDrawMode) {
		let f = self.1.set_draw_mode();
		unsafe { f(self.0, mode) }
	}


	/// Flips the sprite's bitmap.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setImageFlip`]
	#[doc(alias = "sys::ffi::playdate_sprite::setImageFlip")]
	pub fn set_image_flip(&self, flip: BitmapFlip) {
		let f = self.1.set_image_flip();
		unsafe { f(self.0, flip) }
	}

	/// Returns the flip setting of the sprite’s bitmap.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getImageFlip`]
	#[doc(alias = "sys::ffi::playdate_sprite::getImageFlip")]
	pub fn image_flip(&self) -> BitmapFlip {
		let f = self.1.get_image_flip();
		unsafe { f(self.0) }
	}


	/// Specifies a stencil image to be set on the frame buffer before the sprite is drawn.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setStencil`]
	#[doc(alias = "sys::ffi::playdate_sprite::setStencil")]
	pub fn set_stencil(&self, stencil: impl AnyBitmap) {
		let f = self.1.set_stencil();
		unsafe { f(self.0, stencil.as_raw()) }
	}


	/// Sets the clipping rectangle for sprite drawing.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setClipRect`]
	#[doc(alias = "sys::ffi::playdate_sprite::setClipRect")]
	pub fn set_clip_rect(&self, clip: LCDRect) {
		let f = self.1.set_clip_rect();
		unsafe { f(self.0, clip) }
	}

	/// Clears the sprite’s clipping rectangle.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::clearClipRect`]
	#[doc(alias = "sys::ffi::playdate_sprite::clearClipRect")]
	pub fn clear_clip_rect(&self) {
		let f = self.1.clear_clip_rect();
		unsafe { f(self.0) }
	}


	/// Set the `updates_enabled` flag of the sprite
	/// (determines whether the sprite has its update function called).
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setUpdatesEnabled`]
	#[doc(alias = "sys::ffi::playdate_sprite::setUpdatesEnabled")]
	pub fn set_updates_enabled(&self, value: bool) {
		let f = self.1.set_updates_enabled();
		unsafe { f(self.0, value.into()) }
	}

	/// Get the `updates_enabled` flag of the sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::updatesEnabled`]
	#[doc(alias = "sys::ffi::playdate_sprite::updatesEnabled")]
	pub fn updates_enabled(&self) -> bool {
		let f = self.1.updates_enabled();
		unsafe { f(self.0) == 1 }
	}

	/// Set the collisions_enabled flag of the sprite
	/// (along with the `collide_rect`, this determines whether the sprite participates in collisions).
	///
	/// Set to `true` by default.
	///
	/// See also [`set_collide_rect`], [`get_collide_rect`], [`clear_collide_rect`].
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setCollisionsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sprite::setCollisionsEnabled")]
	pub fn set_collisions_enabled(&self, value: bool) {
		let f = self.1.set_collisions_enabled();
		unsafe { f(self.0, value.into()) }
	}

	/// Get the `collisions_enabled` flag of the sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::collisionsEnabled`]
	#[doc(alias = "sys::ffi::playdate_sprite::collisionsEnabled")]
	pub fn collisions_enabled(&self) -> bool {
		let f = self.1.collisions_enabled();
		unsafe { f(self.0) == 1 }
	}

	/// Set the visible flag of the given sprite
	/// (determines whether the sprite has its draw function called).
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setVisible`]
	#[doc(alias = "sys::ffi::playdate_sprite::setVisible")]
	pub fn set_visible(&self, value: bool) {
		let f = self.1.set_visible();
		unsafe { f(self.0, value.into()) }
	}

	/// Get the visible flag of the sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::isVisible`]
	#[doc(alias = "sys::ffi::playdate_sprite::isVisible")]
	pub fn is_visible(&self) -> bool {
		let f = self.1.is_visible();
		unsafe { f(self.0) == 1 }
	}

	/// Marking a sprite opaque tells the sprite system that it doesn’t need to draw anything underneath the sprite,
	/// since it will be overdrawn anyway.
	///
	/// If you set an image without a mask/alpha channel on the sprite, it automatically sets the opaque flag.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setOpaque`]
	#[doc(alias = "sys::ffi::playdate_sprite::setOpaque")]
	pub fn set_opaque(&self, value: bool) {
		let f = self.1.set_opaque();
		unsafe { f(self.0, value.into()) }
	}

	/// Forces the sprite to redraw.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::markDirty`]
	#[doc(alias = "sys::ffi::playdate_sprite::markDirty")]
	pub fn mark_dirty(&self) {
		let f = self.1.mark_dirty();
		unsafe { f(self.0) }
	}

	/// Sets the tag of the sprite.
	///
	/// This can be useful for identifying sprites or types of sprites when using the [collision][] API.
	///
	/// [collision]: crate::callback::collision
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setTag`]
	#[doc(alias = "sys::ffi::playdate_sprite::setTag")]
	pub fn set_tag(&self, tag: u8) {
		let f = self.1.set_tag();
		unsafe { f(self.0, tag) }
	}

	/// Returns the tag of the given sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getTag`]
	#[doc(alias = "sys::ffi::playdate_sprite::getTag")]
	pub fn tag(&self) -> u8 {
		let f = self.1.get_tag();
		unsafe { f(self.0) }
	}

	/// When flag is set to `true`,
	/// the sprite will draw in screen coordinates,
	/// ignoring the currently-set `draw_offset`.
	///
	/// This only affects drawing,
	/// and should not be used on sprites being used for collisions,
	/// which will still happen in world-space.
	///
	/// See also [`playdate_graphics::set_draw_offset`].
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setIgnoresDrawOffset`]
	#[doc(alias = "sys::ffi::playdate_sprite::setIgnoresDrawOffset")]
	pub fn set_ignores_draw_offset(&self, value: bool) {
		let f = self.1.set_ignores_draw_offset();
		unsafe { f(self.0, value.into()) }
	}


	/// Sets `x` and `y` to the current position of sprite.
	///
	/// Equivalent to [`get_position_to`] and [`sys::ffi::playdate_sprite::getPosition`]
	#[doc(alias = "sys::ffi::playdate_sprite::getPosition")]
	pub fn position(&self) -> (c_float, c_float) {
		let (mut x, mut y) = Default::default();
		self.position_to(&mut x, &mut y);
		(x, y)
	}

	/// Sets `x` and `y` to the current position of sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getPosition`]
	#[doc(alias = "sys::ffi::playdate_sprite::getPosition")]
	pub fn position_to(&self, x: &mut c_float, y: &mut c_float) {
		let f = self.1.get_position();
		unsafe { f(self.0, x, y) }
	}


	//
	//
	//
	//
	//


	// TODO: rename to more convenient names


	/// Marks the area of the sprite, relative to its bounds,
	/// to be checked for collisions with other sprites' collide rects.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setCollideRect`]
	#[doc(alias = "sys::ffi::playdate_sprite::setCollideRect")]
	pub fn set_collide_rect(&self, collide: PDRect) {
		let f = self.1.set_collide_rect();
		unsafe { f(self.0, collide) }
	}

	/// Returns the sprite’s collide rect.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getCollideRect`]
	#[doc(alias = "sys::ffi::playdate_sprite::getCollideRect")]
	pub fn collide_rect(&self) -> PDRect {
		let f = self.1.get_collide_rect();
		unsafe { f(self.0) }
	}

	/// Clears the sprite’s collide rect.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::clearCollideRect`]
	#[doc(alias = "sys::ffi::playdate_sprite::clearCollideRect")]
	pub fn clear_collide_rect(&self) {
		let f = self.1.clear_collide_rect();
		unsafe { f(self.0) }
	}

	/// Returns the same values as [`move_with_collisions`] but does not actually move the sprite.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::checkCollisions`]
	#[doc(alias = "sys::ffi::playdate_sprite::check_collisions")]
	#[must_use = "Result is borrowed by C-API"]
	pub fn check_collisions(&self,
	                        goal_x: c_float,
	                        goal_y: c_float,
	                        actual_x: &mut c_float,
	                        actual_y: &mut c_float)
	                        -> &[SpriteCollisionInfo] {
		let f = self.1.check_collisions();
		let mut len: c_int = 0;
		let ptr = unsafe { f(self.0, goal_x, goal_y, actual_x, actual_y, &mut len) };

		if ptr.is_null() || len == 0 {
			&[]
		} else {
			let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
			slice
		}
	}

	/// Moves the sprite towards `goal_x`, `goal_y` taking collisions into account
	/// and returns a slice of [`SpriteCollisionInfo`].
	///
	/// `actual_x`, `actual_y` are set to the sprite’s position after collisions.
	/// If no collisions occurred, this will be the same as `goal_x`, `goal_y`.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::moveWithCollisions`]
	#[doc(alias = "sys::ffi::playdate_sprite::moveWithCollisions")]
	#[must_use = "Result is borrowed by C-API"]
	pub fn move_with_collisions(&self,
	                            goal_x: c_float,
	                            goal_y: c_float,
	                            actual_x: &mut c_float,
	                            actual_y: &mut c_float)
	                            -> &[SpriteCollisionInfo] {
		let f = self.1.move_with_collisions();
		let mut len: c_int = 0;
		let ptr = unsafe { f(self.0, goal_x, goal_y, actual_x, actual_y, &mut len) };

		if ptr.is_null() || len == 0 {
			&[]
		} else {
			let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
			slice
		}
	}


	/// Returns an slice of sprites that have collide rects
	/// that are currently overlapping the given sprite’s collide rect.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::overlappingSprites`]
	#[doc(alias = "sys::ffi::playdate_sprite::overlapping_sprites")]
	#[must_use = "Result is borrowed by C-API"]
	pub fn overlapping_sprites(&self) -> &[SpriteRef] {
		let f = self.1.overlapping_sprites();
		let mut len: c_int = 0;
		let ptr = unsafe { f(self.0, &mut len) };
		let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
		unsafe { core::mem::transmute(slice) }
	}


	/// Sets the sprite’s stencil to the given pattern.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setStencilPattern`]
	#[doc(alias = "sys::ffi::playdate_sprite::setStencilPattern")]
	pub fn set_stencil_pattern(&self, pattern: &mut [u8; 8]) {
		let f = self.1.set_stencil_pattern();
		unsafe { f(self.0, pattern) }
	}

	/// Specifies a stencil image to be set on the frame buffer before the sprite is drawn.
	///
	/// If tile is set, the stencil will be tiled.
	///
	/// Tiled stencils must have __width__ evenly __divisible by 32__.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setStencilImage`]
	#[doc(alias = "sys::ffi::playdate_sprite::setStencilImage")]
	pub fn set_stencil_image(&self, stencil: impl AnyBitmap, tile: bool) {
		let f = self.1.set_stencil_image();
		unsafe { f(self.0, stencil.as_raw(), tile.into()) }
	}

	/// Clears the sprite’s stencil.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::clearStencil`]
	#[doc(alias = "sys::ffi::playdate_sprite::clearStencil")]
	pub fn clear_stencil(&self) {
		let f = self.1.clear_stencil();
		unsafe { f(self.0) }
	}


	/// Sets custom data to the sprite.
	///
	/// Used for associating the sprite with other data.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::setUserdata`]
	#[doc(alias = "sys::ffi::playdate_sprite::setUserdata")]
	pub fn set_userdata(&self, data: Userdata) {
		let f = self.1.set_userdata();
		let userdata = Box::into_raw(Box::new(data));
		let ptr = userdata as *mut c_void;
		unsafe { f(self.0, ptr) }
	}

	/// Gets the _mutable__ reference to sprite’s userdata.
	///
	/// Used for associating the sprite with other data.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getUserdata`]
	#[doc(alias = "sys::ffi::playdate_sprite::get_userdata")]
	pub fn userdata(&self) -> Option<&mut Userdata> {
		let f = self.1.get_userdata();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			None
		} else {
			let ptr = ptr as *mut Userdata;
			// TODO: check ptr is aligned to `UD`
			unsafe { ptr.as_mut() }
		}
	}

	/// Returns __taken__ value the sprite’s userdata.
	///
	/// Equivalent to [`sys::ffi::playdate_sprite::getUserdata`]
	#[doc(alias = "sys::ffi::playdate_sprite::get_userdata")]
	pub(crate) fn take_userdata(&self) -> Option<Box<Userdata>> {
		let f = self.1.get_userdata();
		let ptr = unsafe { f(self.0) };
		if ptr.is_null() {
			None
		} else {
			// TODO: check ptr is aligned to `UD`
			let ud = unsafe { Box::from_raw(ptr as *mut Userdata) };
			Some(ud)
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	/// Ensure that SpriteRef have same size as LCDSprite.
	fn sprite_ref_layout() {
		assert_eq!(
		           core::mem::size_of::<SpriteRef>(),
		           core::mem::size_of::<*mut LCDSprite>()
		);
		assert_eq!(
		           core::mem::size_of::<&[SpriteRef]>(),
		           core::mem::size_of::<&[*mut LCDSprite]>()
		);
	}
}
