use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_void;

use sys::ffi::LCDBitmap;
use sys::ffi::LCDBitmapDrawMode;
use sys::ffi::LCDBitmapFlip;
use sys::ffi::LCDRect;
use sys::ffi::LCDSprite;
use sys::ffi::LCDSpriteCollisionFilterProc;
use sys::ffi::LCDSpriteDrawFunction;
use sys::ffi::LCDSpriteUpdateFunction;
use sys::ffi::PDRect;
use sys::ffi::SpriteCollisionInfo;
use sys::ffi::SpriteQueryInfo;


#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;

impl Api for Default {}


pub trait Api {
	/// Returns [`sys::ffi::playdate_sprite::setAlwaysRedraw`].
	#[doc(alias = "sys::ffi::playdate_sprite::setAlwaysRedraw")]
	#[inline(always)]
	fn set_always_redraw(&self) -> unsafe extern "C" fn(flag: c_int) { *sys::api!(sprite.setAlwaysRedraw) }


	/// Returns [`sys::ffi::playdate_sprite::addDirtyRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::addDirtyRect")]
	#[inline(always)]
	fn add_dirty_rect(&self) -> unsafe extern "C" fn(dirtyRect: LCDRect) { *sys::api!(sprite.addDirtyRect) }


	/// Returns [`sys::ffi::playdate_sprite::drawSprites`].
	#[doc(alias = "sys::ffi::playdate_sprite::drawSprites")]
	#[inline(always)]
	fn draw_sprites(&self) -> unsafe extern "C" fn() { *sys::api!(sprite.drawSprites) }


	/// Returns [`sys::ffi::playdate_sprite::updateAndDrawSprites`].
	#[doc(alias = "sys::ffi::playdate_sprite::updateAndDrawSprites")]
	#[inline(always)]
	fn update_and_draw_sprites(&self) -> unsafe extern "C" fn() { *sys::api!(sprite.updateAndDrawSprites) }


	/// Returns [`sys::ffi::playdate_sprite::newSprite`].
	#[doc(alias = "sys::ffi::playdate_sprite::newSprite")]
	#[inline(always)]
	fn new_sprite(&self) -> unsafe extern "C" fn() -> *mut LCDSprite { *sys::api!(sprite.newSprite) }


	/// Returns [`sys::ffi::playdate_sprite::freeSprite`].
	#[doc(alias = "sys::ffi::playdate_sprite::freeSprite")]
	#[inline(always)]
	fn free_sprite(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) { *sys::api!(sprite.freeSprite) }


	/// Returns [`sys::ffi::playdate_sprite::copy`].
	#[doc(alias = "sys::ffi::playdate_sprite::copy")]
	#[inline(always)]
	fn copy(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut LCDSprite { *sys::api!(sprite.copy) }


	/// Returns [`sys::ffi::playdate_sprite::addSprite`].
	#[doc(alias = "sys::ffi::playdate_sprite::addSprite")]
	#[inline(always)]
	fn add_sprite(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) { *sys::api!(sprite.addSprite) }


	/// Returns [`sys::ffi::playdate_sprite::removeSprite`].
	#[doc(alias = "sys::ffi::playdate_sprite::removeSprite")]
	#[inline(always)]
	fn remove_sprite(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) { *sys::api!(sprite.removeSprite) }


	/// Returns [`sys::ffi::playdate_sprite::removeSprites`].
	#[doc(alias = "sys::ffi::playdate_sprite::removeSprites")]
	#[inline(always)]
	fn remove_sprites(&self) -> unsafe extern "C" fn(sprites: *mut *mut LCDSprite, count: c_int) {
		*sys::api!(sprite.removeSprites)
	}


	/// Returns [`sys::ffi::playdate_sprite::removeAllSprites`].
	#[doc(alias = "sys::ffi::playdate_sprite::removeAllSprites")]
	#[inline(always)]
	fn remove_all_sprites(&self) -> unsafe extern "C" fn() { *sys::api!(sprite.removeAllSprites) }


	/// Returns [`sys::ffi::playdate_sprite::getSpriteCount`].
	#[doc(alias = "sys::ffi::playdate_sprite::getSpriteCount")]
	#[inline(always)]
	fn get_sprite_count(&self) -> unsafe extern "C" fn() -> c_int { *sys::api!(sprite.getSpriteCount) }


	/// Returns [`sys::ffi::playdate_sprite::setBounds`].
	#[doc(alias = "sys::ffi::playdate_sprite::setBounds")]
	#[inline(always)]
	fn set_bounds(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, bounds: PDRect) {
		*sys::api!(sprite.setBounds)
	}


	/// Returns [`sys::ffi::playdate_sprite::getBounds`].
	#[doc(alias = "sys::ffi::playdate_sprite::getBounds")]
	#[inline(always)]
	fn get_bounds(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> PDRect { *sys::api!(sprite.getBounds) }


	/// Returns [`sys::ffi::playdate_sprite::moveTo`].
	#[doc(alias = "sys::ffi::playdate_sprite::moveTo")]
	#[inline(always)]
	fn move_to(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, x: c_float, y: c_float) {
		*sys::api!(sprite.moveTo)
	}


	/// Returns [`sys::ffi::playdate_sprite::moveBy`].
	#[doc(alias = "sys::ffi::playdate_sprite::moveBy")]
	#[inline(always)]
	fn move_by(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, dx: c_float, dy: c_float) {
		*sys::api!(sprite.moveBy)
	}


	/// Returns [`sys::ffi::playdate_sprite::setImage`].
	#[doc(alias = "sys::ffi::playdate_sprite::setImage")]
	#[inline(always)]
	fn set_image(&self)
	             -> unsafe extern "C" fn(sprite: *mut LCDSprite, image: *mut LCDBitmap, flip: LCDBitmapFlip) {
		*sys::api!(sprite.setImage)
	}


	/// Returns [`sys::ffi::playdate_sprite::getImage`].
	#[doc(alias = "sys::ffi::playdate_sprite::getImage")]
	#[inline(always)]
	fn get_image(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut LCDBitmap {
		*sys::api!(sprite.getImage)
	}


	/// Returns [`sys::ffi::playdate_sprite::setSize`].
	#[doc(alias = "sys::ffi::playdate_sprite::setSize")]
	#[inline(always)]
	fn set_size(&self) -> unsafe extern "C" fn(s: *mut LCDSprite, width: c_float, height: c_float) {
		*sys::api!(sprite.setSize)
	}


	/// Returns [`sys::ffi::playdate_sprite::setZIndex`].
	#[doc(alias = "sys::ffi::playdate_sprite::setZIndex")]
	#[inline(always)]
	fn set_z_index(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, zIndex: i16) {
		*sys::api!(sprite.setZIndex)
	}


	/// Returns [`sys::ffi::playdate_sprite::getZIndex`].
	#[doc(alias = "sys::ffi::playdate_sprite::getZIndex")]
	#[inline(always)]
	fn get_z_index(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> i16 { *sys::api!(sprite.getZIndex) }


	/// Returns [`sys::ffi::playdate_sprite::setDrawMode`].
	#[doc(alias = "sys::ffi::playdate_sprite::setDrawMode")]
	#[inline(always)]
	fn set_draw_mode(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, mode: LCDBitmapDrawMode) {
		*sys::api!(sprite.setDrawMode)
	}


	/// Returns [`sys::ffi::playdate_sprite::setImageFlip`].
	#[doc(alias = "sys::ffi::playdate_sprite::setImageFlip")]
	#[inline(always)]
	fn set_image_flip(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, flip: LCDBitmapFlip) {
		*sys::api!(sprite.setImageFlip)
	}


	/// Returns [`sys::ffi::playdate_sprite::getImageFlip`].
	#[doc(alias = "sys::ffi::playdate_sprite::getImageFlip")]
	#[inline(always)]
	fn get_image_flip(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> LCDBitmapFlip {
		*sys::api!(sprite.getImageFlip)
	}


	/// Returns [`sys::ffi::playdate_sprite::setStencil`].
	#[doc(alias = "sys::ffi::playdate_sprite::setStencil")]
	#[inline(always)]
	fn set_stencil(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, stencil: *mut LCDBitmap) {
		*sys::api!(sprite.setStencil)
	}


	/// Returns [`sys::ffi::playdate_sprite::setClipRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::setClipRect")]
	#[inline(always)]
	fn set_clip_rect(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, clipRect: LCDRect) {
		*sys::api!(sprite.setClipRect)
	}


	/// Returns [`sys::ffi::playdate_sprite::clearClipRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::clearClipRect")]
	#[inline(always)]
	fn clear_clip_rect(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) { *sys::api!(sprite.clearClipRect) }


	/// Returns [`sys::ffi::playdate_sprite::setClipRectsInRange`].
	#[doc(alias = "sys::ffi::playdate_sprite::setClipRectsInRange")]
	#[inline(always)]
	fn set_clip_rects_in_range(&self) -> unsafe extern "C" fn(clipRect: LCDRect, startZ: c_int, endZ: c_int) {
		*sys::api!(sprite.setClipRectsInRange)
	}


	/// Returns [`sys::ffi::playdate_sprite::clearClipRectsInRange`].
	#[doc(alias = "sys::ffi::playdate_sprite::clearClipRectsInRange")]
	#[inline(always)]
	fn clear_clip_rects_in_range(&self) -> unsafe extern "C" fn(startZ: c_int, endZ: c_int) {
		*sys::api!(sprite.clearClipRectsInRange)
	}


	/// Returns [`sys::ffi::playdate_sprite::setUpdatesEnabled`].
	#[doc(alias = "sys::ffi::playdate_sprite::setUpdatesEnabled")]
	#[inline(always)]
	fn set_updates_enabled(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, flag: c_int) {
		*sys::api!(sprite.setUpdatesEnabled)
	}


	/// Returns [`sys::ffi::playdate_sprite::updatesEnabled`].
	#[doc(alias = "sys::ffi::playdate_sprite::updatesEnabled")]
	#[inline(always)]
	fn updates_enabled(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> c_int {
		*sys::api!(sprite.updatesEnabled)
	}


	/// Returns [`sys::ffi::playdate_sprite::setCollisionsEnabled`].
	#[doc(alias = "sys::ffi::playdate_sprite::setCollisionsEnabled")]
	#[inline(always)]
	fn set_collisions_enabled(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, flag: c_int) {
		*sys::api!(sprite.setCollisionsEnabled)
	}


	/// Returns [`sys::ffi::playdate_sprite::collisionsEnabled`].
	#[doc(alias = "sys::ffi::playdate_sprite::collisionsEnabled")]
	#[inline(always)]
	fn collisions_enabled(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> c_int {
		*sys::api!(sprite.collisionsEnabled)
	}


	/// Returns [`sys::ffi::playdate_sprite::setVisible`].
	#[doc(alias = "sys::ffi::playdate_sprite::setVisible")]
	#[inline(always)]
	fn set_visible(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, flag: c_int) {
		*sys::api!(sprite.setVisible)
	}


	/// Returns [`sys::ffi::playdate_sprite::isVisible`].
	#[doc(alias = "sys::ffi::playdate_sprite::isVisible")]
	#[inline(always)]
	fn is_visible(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> c_int { *sys::api!(sprite.isVisible) }


	/// Returns [`sys::ffi::playdate_sprite::setOpaque`].
	#[doc(alias = "sys::ffi::playdate_sprite::setOpaque")]
	#[inline(always)]
	fn set_opaque(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, flag: c_int) {
		*sys::api!(sprite.setOpaque)
	}


	/// Returns [`sys::ffi::playdate_sprite::markDirty`].
	#[doc(alias = "sys::ffi::playdate_sprite::markDirty")]
	#[inline(always)]
	fn mark_dirty(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) { *sys::api!(sprite.markDirty) }


	/// Returns [`sys::ffi::playdate_sprite::setTag`].
	#[doc(alias = "sys::ffi::playdate_sprite::setTag")]
	#[inline(always)]
	fn set_tag(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, tag: u8) { *sys::api!(sprite.setTag) }


	/// Returns [`sys::ffi::playdate_sprite::getTag`].
	#[doc(alias = "sys::ffi::playdate_sprite::getTag")]
	#[inline(always)]
	fn get_tag(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> u8 { *sys::api!(sprite.getTag) }


	/// Returns [`sys::ffi::playdate_sprite::setIgnoresDrawOffset`].
	#[doc(alias = "sys::ffi::playdate_sprite::setIgnoresDrawOffset")]
	#[inline(always)]
	fn set_ignores_draw_offset(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, flag: c_int) {
		*sys::api!(sprite.setIgnoresDrawOffset)
	}


	/// Returns [`sys::ffi::playdate_sprite::setUpdateFunction`].
	#[doc(alias = "sys::ffi::playdate_sprite::setUpdateFunction")]
	#[inline(always)]
	fn set_update_function(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteUpdateFunction) {
		*sys::api!(sprite.setUpdateFunction)
	}


	/// Returns [`sys::ffi::playdate_sprite::setDrawFunction`].
	#[doc(alias = "sys::ffi::playdate_sprite::setDrawFunction")]
	#[inline(always)]
	fn set_draw_function(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteDrawFunction) {
		*sys::api!(sprite.setDrawFunction)
	}


	/// Returns [`sys::ffi::playdate_sprite::getPosition`].
	#[doc(alias = "sys::ffi::playdate_sprite::getPosition")]
	#[inline(always)]
	fn get_position(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, x: *mut c_float, y: *mut c_float) {
		*sys::api!(sprite.getPosition)
	}


	/// Returns [`sys::ffi::playdate_sprite::resetCollisionWorld`].
	#[doc(alias = "sys::ffi::playdate_sprite::resetCollisionWorld")]
	#[inline(always)]
	fn reset_collision_world(&self) -> unsafe extern "C" fn() { *sys::api!(sprite.resetCollisionWorld) }


	/// Returns [`sys::ffi::playdate_sprite::setCollideRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::setCollideRect")]
	#[inline(always)]
	fn set_collide_rect(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, collideRect: PDRect) {
		*sys::api!(sprite.setCollideRect)
	}


	/// Returns [`sys::ffi::playdate_sprite::getCollideRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::getCollideRect")]
	#[inline(always)]
	fn get_collide_rect(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> PDRect {
		*sys::api!(sprite.getCollideRect)
	}


	/// Returns [`sys::ffi::playdate_sprite::clearCollideRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::clearCollideRect")]
	#[inline(always)]
	fn clear_collide_rect(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) {
		*sys::api!(sprite.clearCollideRect)
	}


	/// Returns [`sys::ffi::playdate_sprite::setCollisionResponseFunction`].
	#[doc(alias = "sys::ffi::playdate_sprite::setCollisionResponseFunction")]
	#[inline(always)]
	fn set_collision_response_function(
		&self)
		-> unsafe extern "C" fn(sprite: *mut LCDSprite, func: LCDSpriteCollisionFilterProc) {
		*sys::api!(sprite.setCollisionResponseFunction)
	}


	/// Returns [`sys::ffi::playdate_sprite::checkCollisions`].
	#[doc(alias = "sys::ffi::playdate_sprite::checkCollisions")]
	#[inline(always)]
	fn check_collisions(
		&self)
		-> unsafe extern "C" fn(sprite: *mut LCDSprite,
		                        goalX: c_float,
		                        goalY: c_float,
		                        actualX: *mut c_float,
		                        actualY: *mut c_float,
		                        len: *mut c_int) -> *mut SpriteCollisionInfo {
		*sys::api!(sprite.checkCollisions)
	}


	/// Returns [`sys::ffi::playdate_sprite::moveWithCollisions`].
	#[doc(alias = "sys::ffi::playdate_sprite::moveWithCollisions")]
	#[inline(always)]
	fn move_with_collisions(
		&self)
		-> unsafe extern "C" fn(sprite: *mut LCDSprite,
		                        goalX: c_float,
		                        goalY: c_float,
		                        actualX: *mut c_float,
		                        actualY: *mut c_float,
		                        len: *mut c_int) -> *mut SpriteCollisionInfo {
		*sys::api!(sprite.moveWithCollisions)
	}


	/// Returns [`sys::ffi::playdate_sprite::querySpritesAtPoint`].
	#[doc(alias = "sys::ffi::playdate_sprite::querySpritesAtPoint")]
	#[inline(always)]
	fn query_sprites_at_point(
		&self)
		-> unsafe extern "C" fn(x: c_float, y: c_float, len: *mut c_int) -> *mut *mut LCDSprite {
		*sys::api!(sprite.querySpritesAtPoint)
	}


	/// Returns [`sys::ffi::playdate_sprite::querySpritesInRect`].
	#[doc(alias = "sys::ffi::playdate_sprite::querySpritesInRect")]
	#[inline(always)]
	fn query_sprites_in_rect(
		&self)
		-> unsafe extern "C" fn(x: c_float,
		                        y: c_float,
		                        width: c_float,
		                        height: c_float,
		                        len: *mut c_int) -> *mut *mut LCDSprite {
		*sys::api!(sprite.querySpritesInRect)
	}


	/// Returns [`sys::ffi::playdate_sprite::querySpritesAlongLine`].
	#[doc(alias = "sys::ffi::playdate_sprite::querySpritesAlongLine")]
	#[inline(always)]
	fn query_sprites_along_line(
		&self)
		-> unsafe extern "C" fn(x1: c_float,
		                        y1: c_float,
		                        x2: c_float,
		                        y2: c_float,
		                        len: *mut c_int) -> *mut *mut LCDSprite {
		*sys::api!(sprite.querySpritesAlongLine)
	}


	/// Returns [`sys::ffi::playdate_sprite::querySpriteInfoAlongLine`].
	#[doc(alias = "sys::ffi::playdate_sprite::querySpriteInfoAlongLine")]
	#[inline(always)]
	fn query_sprite_info_along_line(
		&self)
		-> unsafe extern "C" fn(x1: c_float,
		                        y1: c_float,
		                        x2: c_float,
		                        y2: c_float,
		                        len: *mut c_int) -> *mut SpriteQueryInfo {
		*sys::api!(sprite.querySpriteInfoAlongLine)
	}


	/// Returns [`sys::ffi::playdate_sprite::overlappingSprites`].
	#[doc(alias = "sys::ffi::playdate_sprite::overlappingSprites")]
	#[inline(always)]
	fn overlapping_sprites(
		&self)
		-> unsafe extern "C" fn(sprite: *mut LCDSprite, len: *mut c_int) -> *mut *mut LCDSprite {
		*sys::api!(sprite.overlappingSprites)
	}


	/// Returns [`sys::ffi::playdate_sprite::allOverlappingSprites`].
	#[doc(alias = "sys::ffi::playdate_sprite::allOverlappingSprites")]
	#[inline(always)]
	fn all_overlapping_sprites(&self) -> unsafe extern "C" fn(len: *mut c_int) -> *mut *mut LCDSprite {
		*sys::api!(sprite.allOverlappingSprites)
	}


	/// Returns [`sys::ffi::playdate_sprite::setStencilPattern`].
	#[doc(alias = "sys::ffi::playdate_sprite::setStencilPattern")]
	#[inline(always)]
	fn set_stencil_pattern(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, pattern: *mut [u8; 8]) {
		*sys::api!(sprite.setStencilPattern)
	}


	/// Returns [`sys::ffi::playdate_sprite::clearStencil`].
	#[doc(alias = "sys::ffi::playdate_sprite::clearStencil")]
	#[inline(always)]
	fn clear_stencil(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) { *sys::api!(sprite.clearStencil) }


	/// Returns [`sys::ffi::playdate_sprite::setUserdata`].
	#[doc(alias = "sys::ffi::playdate_sprite::setUserdata")]
	#[inline(always)]
	fn set_userdata(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite, userdata: *mut c_void) {
		*sys::api!(sprite.setUserdata)
	}


	/// Returns [`sys::ffi::playdate_sprite::getUserdata`].
	#[doc(alias = "sys::ffi::playdate_sprite::getUserdata")]
	#[inline(always)]
	fn get_userdata(&self) -> unsafe extern "C" fn(sprite: *mut LCDSprite) -> *mut c_void {
		*sys::api!(sprite.getUserdata)
	}


	/// Returns [`sys::ffi::playdate_sprite::setStencilImage`].
	#[doc(alias = "sys::ffi::playdate_sprite::setStencilImage")]
	#[inline(always)]
	fn set_stencil_image(&self)
	                     -> unsafe extern "C" fn(sprite: *mut LCDSprite, stencil: *mut LCDBitmap, tile: c_int) {
		*sys::api!(sprite.setStencilImage)
	}
}
