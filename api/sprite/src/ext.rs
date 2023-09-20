use core::ffi::c_float;

use sys::ffi::CollisionPoint;
use sys::ffi::CollisionVector;
use sys::ffi::PDRect;
use sys::ffi::SpriteCollisionInfo;
use sys::ffi::SpriteCollisionResponseType;
use sys::ffi::SpriteQueryInfo;

use crate::SpriteRef;


pub trait SpriteQueryInfoExt {
	/// The sprite being intersected by the segment
	fn sprite(&self) -> SpriteRef;

	/// Entry point
	///
	/// `ti1` and `ti2` are numbers between 0 and 1
	/// which indicate how far from the starting point of the line segment the collision happened
	fn ti1(&self) -> c_float;

	/// Exit point
	///
	/// `ti1` and `ti2` are numbers between 0 and 1
	/// which indicate how far from the starting point of the line segment the collision happened
	fn ti2(&self) -> c_float;

	/// The coordinates of the first intersection between sprite and the line segment
	fn entry_point(&self) -> &CollisionPoint;
	fn entry_point_mut(&mut self) -> &mut CollisionPoint;

	/// The coordinates of the second intersection between sprite and the line segment
	fn exit_point(&self) -> &CollisionPoint;
	fn exit_point_mut(&mut self) -> &mut CollisionPoint;
}

impl SpriteQueryInfoExt for SpriteQueryInfo {
	fn sprite(&self) -> SpriteRef { self.sprite.into() }

	fn ti1(&self) -> c_float { self.ti1 }

	fn ti2(&self) -> c_float { self.ti2 }

	fn entry_point(&self) -> &CollisionPoint { &self.entryPoint }
	fn entry_point_mut(&mut self) -> &mut CollisionPoint { &mut self.entryPoint }

	fn exit_point(&self) -> &CollisionPoint { &self.exitPoint }
	fn exit_point_mut(&mut self) -> &mut CollisionPoint { &mut self.exitPoint }
}


trait SpriteCollisionInfoExt {
	/// The sprite being moved
	fn sprite(&self) -> SpriteRef;

	/// The sprite colliding with the sprite being moved
	fn other(&self) -> SpriteRef;

	/// The result of collisionResponse
	fn response_type(&self) -> SpriteCollisionResponseType;

	/// True if the sprite was overlapping other when the collision started.
	///
	/// False if it didn’t overlap but tunneled through other
	fn overlaps(&self) -> bool;

	/// A number between `0` and `1` indicating how far along the movement to the goal the collision occurred
	fn ti(&self) -> c_float;

	/// The difference between the original coordinates and the actual ones when the collision happened
	fn diff(&self) -> &CollisionPoint;

	/// The collision normal;
	/// usually `-1`, `0`, or `1` in `x` and `y`.
	///
	/// Use this value to determine things like if your character is touching the ground
	fn normal(&self) -> &CollisionVector;

	/// The coordinates where the sprite started touching other
	fn touch(&self) -> &CollisionPoint;

	/// The rectangle the sprite occupied when the touch happened
	fn sprite_rect(&self) -> &PDRect;

	/// The rectangle the sprite being collided with occupied when the touch happened
	fn other_rect(&self) -> &PDRect;
}

impl SpriteCollisionInfoExt for SpriteCollisionInfo {
	fn sprite(&self) -> SpriteRef { self.sprite.into() }
	fn other(&self) -> SpriteRef { self.other.into() }
	fn response_type(&self) -> SpriteCollisionResponseType { self.responseType }
	fn overlaps(&self) -> bool { self.overlaps != 0 }
	fn ti(&self) -> c_float { self.ti }
	fn diff(&self) -> &CollisionPoint { &self.move_ }
	fn normal(&self) -> &CollisionVector { &self.normal }
	fn touch(&self) -> &CollisionPoint { &self.touch }
	fn sprite_rect(&self) -> &PDRect { &self.spriteRect }
	fn other_rect(&self) -> &PDRect { &self.otherRect }
}


pub trait SpriteCollisionResponseTypeExt {
	#![allow(non_upper_case_globals)]
	const Slide: SpriteCollisionResponseType = SpriteCollisionResponseType::kCollisionTypeSlide;
	const Freeze: SpriteCollisionResponseType = SpriteCollisionResponseType::kCollisionTypeFreeze;
	const Overlap: SpriteCollisionResponseType = SpriteCollisionResponseType::kCollisionTypeOverlap;
	const Bounce: SpriteCollisionResponseType = SpriteCollisionResponseType::kCollisionTypeBounce;
}

impl SpriteCollisionResponseTypeExt for SpriteCollisionResponseType {}
