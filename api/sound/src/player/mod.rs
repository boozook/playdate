use core::ffi::c_int;

pub mod fp;
pub mod sp;

pub use fp::Player as FilePlayer;
pub use sp::Player as SamplePlayer;


/// Repeat playback mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Repeat {
	/// Player loops the given number of times.
	Loops(c_int),
	/// Player loops endlessly until it is stopped with [`Player::stop()`] or [`Player::try_stop()`].
	LoopsEndlessly,
	/// Player does ping-pong looping.
	// XXX: Strange doc, "If negative one, it does ping-pong looping." - test and probably remove int and set to `-1`.
	PingPong,
}

impl Into<c_int> for Repeat {
	fn into(self) -> c_int {
		match self {
			Repeat::Loops(v) => v,
			Repeat::LoopsEndlessly => 0,
			Repeat::PingPong => -1,
		}
	}
}


// pub trait AnyPlayer {
// 	pub fn api(&self) -> &Api
// }
