#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]
#![feature(const_trait_impl)]

#[macro_use]
pub extern crate sys;
pub extern crate alloc;

pub mod error;
pub mod player;
pub mod sample;


pub mod prelude {
	pub use crate::error::ApiError as SndApiError;
	pub use crate::error::Error as SndError;

	pub use crate::player;
}
