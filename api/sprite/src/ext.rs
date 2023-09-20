use core::ffi::c_float;

use sys::ffi::CollisionPoint;
use sys::ffi::SpriteQueryInfo;

use crate::SpriteRef;


pub trait SpriteQueryInfoExt {
	/// The sprite being intersected by the segment
	fn sprite(&self) -> SpriteRef;

	/// Entry point
	///
	/// `ti1` and `ti2` are numbers between 0 and 1 which indicate how far from the starting point of the line segment the collision happened
	fn ti1(&self) -> c_float;

	/// Exit point
	///
	/// `ti1` and `ti2` are numbers between 0 and 1 which indicate how far from the starting point of the line segment the collision happened
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
