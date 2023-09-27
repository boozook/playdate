#![cfg_attr(not(test), no_std)]
#![feature(error_in_core)]
#![feature(const_trait_impl)]

extern crate sys;
extern crate alloc;

pub mod error;
pub mod player;
pub mod sample;

// TODO: sound: channel, synth, sequence, effect, lfo, envelope, source!, etc..


pub mod prelude {
	pub use crate::error::ApiError as SndApiError;
	pub use crate::error::Error as SndError;

	pub use crate::player;
	pub use crate::sample;
}
