#![cfg_attr(not(test), no_std)]

extern crate sys;
extern crate alloc;

use core::ffi::c_int;
use core::ffi::c_float;

use sys::traits::AsRaw;
use sys::ffi::SpriteQueryInfo;
use sys::ffi::LCDRect;
use sys::ffi::LCDSprite;

pub mod ext;
mod sprite;
pub mod api;

pub mod callback {
	pub mod draw;
	pub mod update;
	pub mod collision;
}

pub mod prelude {
	pub use super::sprite::*;
	pub use super::api::Api as _;
	pub use super::api::Default as DefaultSpriteApi;
	pub use super::callback::draw::SpriteDraw;
	pub use super::callback::update::SpriteUpdate;
	pub use super::callback::collision::SpriteCollisionResponse;

	pub use super::{TypedSprite, SpriteApi};
}

pub use sprite::*;
use crate::api::Api;


/// If set to `true`, causes all sprites to draw each frame, whether or not they have been marked dirty.
/// This may speed up the performance of your game if the system’s dirty rect tracking is taking up too much time -
/// for example if there are many sprites moving around on screen at once.
///
/// Equivalent to [`sys::ffi::playdate_sprite::setAlwaysRedraw`]
#[doc(alias = "sys::ffi::playdate_sprite::setAlwaysRedraw")]
pub fn set_always_redraw(value: bool) {
	let f = api::Api::set_always_redraw(&api::Default);
	unsafe { f(value.into()) }
}

/// Marks the given dirty_rect (in screen coordinates) as needing a redraw.
///
/// Graphics drawing functions now call this automatically,
/// adding their drawn areas to the sprite’s dirty list,
/// so there’s usually no need to call this manually.
///
/// Equivalent to [`sys::ffi::playdate_sprite::addDirtyRect`]
#[doc(alias = "sys::ffi::playdate_sprite::addDirtyRect")]
pub fn add_dirty_rect(rect: LCDRect) {
	let f = api::Api::add_dirty_rect(&api::Default);
	unsafe { f(rect) }
}

/// Draws every sprite in the display list.
///
/// Equivalent to [`sys::ffi::playdate_sprite::drawSprites`]
#[doc(alias = "sys::ffi::playdate_sprite::drawSprites")]
pub fn draw_sprites() {
	let f = api::Api::draw_sprites(&api::Default);
	unsafe { f() }
}

/// Updates and draws every sprite in the display list.
///
/// Equivalent to [`sys::ffi::playdate_sprite::updateAndDrawSprites`]
#[doc(alias = "sys::ffi::playdate_sprite::updateAndDrawSprites")]
pub fn update_and_draw_sprites() {
	let f = api::Api::update_and_draw_sprites(&api::Default);
	unsafe { f() }
}


/// Adds the given sprite to the display list,
/// so that it is drawn in the current scene.
///
/// See also [`Sprite::add`]
///
/// Equivalent to [`sys::ffi::playdate_sprite::addSprite`]
#[doc(alias = "sys::ffi::playdate_sprite::addSprite")]
pub fn add_sprite(sprite: &impl AnySprite) {
	let f = sprite.api_ref().add_sprite();
	unsafe { f(sprite.as_raw()) }
}

/// Removes the given sprite from the display list.
///
/// See also [`Sprite::remove`]
///
/// Equivalent to [`sys::ffi::playdate_sprite::removeSprite`]
#[doc(alias = "sys::ffi::playdate_sprite::removeSprite")]
pub fn remove_sprite(sprite: &impl AnySprite) {
	let f = sprite.api_ref().remove_sprite();
	unsafe { f(sprite.as_raw()) }
}

/// Removes all of the given sprites from the display list.
///
/// Equivalent to [`sys::ffi::playdate_sprite::removeSprites`]
#[doc(alias = "sys::ffi::playdate_sprite::removeSprites")]
pub fn remove_sprites(sprites: &[impl AnySprite]) {
	let mut ptrs = alloc::vec::Vec::with_capacity(sprites.len());
	ptrs.extend(sprites.into_iter().map(|sp| unsafe { sp.as_raw() }));
	let f = sprites.first()
	               .map(|sp| sp.api_ref().remove_sprites())
	               .unwrap_or(api::Default.remove_sprites());
	unsafe { f(ptrs.as_mut_ptr(), sprites.len() as _) }
	drop(ptrs);
}


/// Removes all sprites from the display list.
///
/// Equivalent to [`sys::ffi::playdate_sprite::removeAllSprites`]
#[doc(alias = "sys::ffi::playdate_sprite::removeAllSprites")]
pub fn remove_all_sprites() {
	let f = api::Api::remove_all_sprites(&api::Default);
	unsafe { f() }
}


/// Returns the total number of sprites in the display list.
///
/// Equivalent to [`sys::ffi::playdate_sprite::getSpriteCount`]
#[doc(alias = "sys::ffi::playdate_sprite::getSpriteCount")]
pub fn sprite_count() -> c_int {
	let f = api::Api::get_sprite_count(&api::Default);
	unsafe { f() }
}


/// Sets the clipping rectangle for all sprites with a Z index within `start_z` and `end_z` __inclusive__.
///
/// Equivalent to [`sys::ffi::playdate_sprite::setClipRectsInRange`]
#[doc(alias = "sys::ffi::playdate_sprite::setClipRectsInRange")]
pub fn set_clip_rects_in_range(clip: LCDRect, start_z: c_int, end_z: c_int) {
	let f = api::Api::set_clip_rects_in_range(&api::Default);
	unsafe { f(clip, start_z, end_z) }
}

/// Clears the clipping rectangle for all sprites with a Z index within `start_z` and `end_z` __inclusive__.
///
/// Equivalent to [`sys::ffi::playdate_sprite::clearClipRectsInRange`]
#[doc(alias = "sys::ffi::playdate_sprite::clearClipRectsInRange")]
pub fn clear_clip_rects_in_range(start_z: c_int, end_z: c_int) {
	let f = api::Api::clear_clip_rects_in_range(&api::Default);
	unsafe { f(start_z, end_z) }
}


/// Frees and reallocates internal collision data, resetting everything to its default state.
///
/// Equivalent to [`sys::ffi::playdate_sprite::resetCollisionWorld`]
#[doc(alias = "sys::ffi::playdate_sprite::resetCollisionWorld")]
pub fn reset_collision_world() {
	let f = api::Api::reset_collision_world(&api::Default);
	unsafe { f() }
}


/// Returns an slice of all sprites with collision rects containing the point at `x`, `y`.
///
/// Equivalent to [`sys::ffi::playdate_sprite::querySpritesAtPoint`]
#[doc(alias = "sys::ffi::playdate_sprite::querySpritesAtPoint")]
pub fn query_sprites_at_point(x: c_float, y: c_float) -> &'static [SpriteRef] {
	let mut len: c_int = 0;
	let api = api::Default;
	let f = api.query_sprites_at_point();
	let ptr = unsafe { f(x, y, &mut len) };
	let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
	unsafe { core::mem::transmute(slice) }
}

/// Returns an slice of all sprites with collision rects
/// that intersect the `width` by `height` rect at `x`, `y`.
///
/// Equivalent to [`sys::ffi::playdate_sprite::querySpritesInRect`]
#[doc(alias = "sys::ffi::playdate_sprite::querySpritesInRect")]
pub fn query_sprites_in_rect(x: c_float, y: c_float, width: c_float, height: c_float) -> &'static [SpriteRef] {
	let mut len: c_int = 0;
	let f = api::Api::query_sprites_in_rect(&api::Default);
	let ptr = unsafe { f(x, y, width, height, &mut len) };
	let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
	unsafe { core::mem::transmute(slice) }
}

/// Returns an slice of all sprites with collision rects
/// that intersect the line connecting `x1`, `y1` and `x2`, `y2`.
///
/// Equivalent to [`sys::ffi::playdate_sprite::querySpritesAlongLine`]
#[doc(alias = "sys::ffi::playdate_sprite::querySpritesAlongLine")]
pub fn query_sprites_along_line(x1: c_float, y1: c_float, x2: c_float, y2: c_float) -> &'static [SpriteRef] {
	let mut len: c_int = 0;
	let f = api::Api::query_sprites_along_line(&api::Default);
	let ptr = unsafe { f(x1, y1, x2, y2, &mut len) };
	let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
	unsafe { core::mem::transmute(slice) }
}

/// Returns an slice of [`SpriteQueryInfo`]s for all sprites with collision rects
/// that intersect the line connecting `x1`, `y1` and `x2`, `y2`.
///
/// If you don’t need this information, use [`query_sprites_along_line`] as it will be faster.
///
/// Equivalent to [`sys::ffi::playdate_sprite::querySpriteInfoAlongLine`]
#[doc(alias = "sys::ffi::playdate_sprite::querySpriteInfoAlongLine")]
pub fn query_sprite_info_along_line(x1: c_float,
                                    y1: c_float,
                                    x2: c_float,
                                    y2: c_float)
                                    -> &'static [SpriteQueryInfo] {
	let mut len: c_int = 0;
	let f = api::Api::query_sprite_info_along_line(&api::Default);
	let ptr = unsafe { f(x1, y1, x2, y2, &mut len) };
	unsafe { core::slice::from_raw_parts(ptr, len as _) }
}


/// Returns an slice of all sprites that have collide rects that are currently overlapping.
///
/// Each consecutive pair of sprites is overlapping (eg. 0 & 1 overlap, 2 & 3 overlap, etc).
///
/// Equivalent to [`sys::ffi::playdate_sprite::allOverlappingSprites`]
#[doc(alias = "sys::ffi::playdate_sprite::allOverlappingSprites")]
pub fn all_overlapping_sprites() -> &'static [SpriteRef] {
	let f = api::Api::all_overlapping_sprites(&api::Default);
	let mut len: c_int = 0;
	let ptr = unsafe { f(&mut len) };
	let slice = unsafe { core::slice::from_raw_parts(ptr, len as _) };
	unsafe { core::mem::transmute(slice) }
}


pub trait AnySprite: AsRaw<Type = LCDSprite> + SpriteApi {}
impl<T: AnySprite> AnySprite for &'_ T {}


pub trait SpriteApi {
	/// Type of inner API access-point.
	type Api: api::Api;

	/// Get a copy of inner api access point.
	fn api(&self) -> Self::Api
		where Self::Api: Copy;

	/// Get a ref to inner api access point.
	fn api_ref(&self) -> &Self::Api;
}

impl<T: SpriteApi> SpriteApi for &'_ T {
	type Api = T::Api;

	fn api(&self) -> Self::Api
		where Self::Api: Copy {
		(*self).api()
	}

	fn api_ref(&self) -> &Self::Api { (*self).api_ref() }
}


/// Represents strictly typed sprite, includes associated user-data and free-on-drop flag.
pub trait TypedSprite: AsRaw<Type = LCDSprite> + SpriteApi {
	/// Associated user-data with sprite.
	type Userdata;
	/// Should be freed when sprite is dropped.
	const FREE_ON_DROP: bool;
}
