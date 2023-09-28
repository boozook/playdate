use core::ffi::c_float;
use core::ffi::c_int;

use sys::ffi::SamplePlayer;
use sys::ffi::sndCallbackProc;

use crate::error::Error;
use super::Repeat;


mod api;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Player<Api: api::Api = api::Default>(*mut SamplePlayer, Api);


// ctor //

impl<Api> Player<Api> where Api: api::Api {
	/// Allocates and returns a new sample player.
	///
	/// Equivalent to [newPlayer](sys::ffi::playdate_sound_sampleplayer::newPlayer)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::newPlayer")]
	pub fn new() -> Result<Player<Api>, Error>
		where Api: Default {
		let api = Api::default();
		Self::new_with(api)
	}

	/// Allocates and returns a new sample player with given `api`.
	///
	/// Equivalent to [newPlayer](sys::ffi::playdate_sound_sampleplayer::newPlayer)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::newPlayer")]
	pub fn new_with(api: Api) -> Result<Player<Api>, Error> {
		let f = api.new_player();
		let player = unsafe { f() };
		if player.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Player(player, api))
		}
	}
}


impl<Api: api::Api> Drop for Player<Api> {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let f = self.api().free_player();
			unsafe { f(self.0) }
			self.0 = core::ptr::null_mut();
		}
	}
}


// utils //


impl<Api: api::Api> Player<Api> {
	#[inline(always)]
	pub fn api(&self) -> &Api { &self.1 }
}


// impl //

impl<Api> Player<Api> where Api: api::Api {
	/// Assigns `sample` to player.
	///
	/// Equivalent to [setSample](sys::ffi::playdate_sound_sampleplayer::setSample)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setSample")]
	pub fn set_sample(&self, sample: &crate::sample::Sample) {
		let f = self.api().set_sample();
		unsafe { f(self.0, sample.0) }
	}


	/// Returns `true` if player is playing a sample, `false` if not.
	///
	/// Equivalent to [isPlaying](sys::ffi::playdate_sound_sampleplayer::isPlaying)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::isPlaying")]
	pub fn is_playing(&self) -> bool {
		let f = self.api().is_playing();
		unsafe { f(self.0) == 1 }
	}


	/// Starts playing the sample in player.
	/// Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.
	///
	/// Equivalent to [play](sys::ffi::playdate_sound_sampleplayer::play)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::play")]
	pub fn play(&self, repeat: Repeat, rate: c_float) -> c_int {
		let f = self.api().play();
		unsafe { f(self.0, repeat.into(), rate) }
	}

	/// Stops playing the sample.
	///
	/// Equivalent to [stop](sys::ffi::playdate_sound_sampleplayer::stop)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::stop")]
	pub fn stop(&self) {
		let f = self.api().stop();
		unsafe { f(self.0) }
	}


	/// Gets the current left and right channel volume of the player.
	///
	/// Equivalent to [getVolume](sys::ffi::playdate_sound_sampleplayer::getVolume)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getVolume")]
	pub fn volume(&self) -> (c_float, c_float) {
		let (mut left, mut right) = (0.0, 0.0);
		let f = self.api().get_volume();
		unsafe { f(self.0, &mut left, &mut right) };
		(left, right)
	}

	/// Sets the playback volume for left and right channels.
	///
	/// Equivalent to [setVolume](sys::ffi::playdate_sound_sampleplayer::setVolume)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setVolume")]
	pub fn set_volume(&self, left: c_float, right: c_float) {
		let f = self.api().set_volume();
		unsafe { f(self.0, left, right) }
	}

	/// Returns the length, in seconds, of the sample assigned to player.
	///
	/// Equivalent to [getLength](sys::ffi::playdate_sound_sampleplayer::getLength)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getLength")]
	pub fn length(&self) -> c_float {
		let f = self.api().get_length();
		unsafe { f(self.0) }
	}

	/// Gets the current offset in seconds for player.
	///
	/// Equivalent to [getOffset](sys::ffi::playdate_sound_sampleplayer::getOffset)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getOffset")]
	pub fn offset(&self) -> c_float {
		let f = self.api().get_offset();
		unsafe { f(self.0) }
	}

	/// Sets the current offset of the player, in seconds.
	///
	/// Equivalent to [setOffset](sys::ffi::playdate_sound_sampleplayer::setOffset)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setOffset")]
	pub fn set_offset(&self, offset: c_float) {
		let f = self.api().set_offset();
		unsafe { f(self.0, offset) }
	}

	/// Gets the playback rate for player.
	///
	/// Equivalent to [getRate](sys::ffi::playdate_sound_sampleplayer::getRate)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getRate")]
	pub fn rate(&self) -> c_float {
		let f = self.api().get_rate();
		unsafe { f(self.0) }
	}

	/// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.
	///
	/// Equivalent to [setRate](sys::ffi::playdate_sound_sampleplayer::setRate)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setRate")]
	pub fn set_rate(&self, rate: c_float) {
		let f = self.api().set_rate();
		unsafe { f(self.0, rate) }
	}

	/// When used with a [`Repeat::PingPong`], does ping-pong looping, with a `start` and `end` position in frames.
	///
	/// Equivalent to [setPlayRange](sys::ffi::playdate_sound_sampleplayer::setPlayRange)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setPlayRange")]
	pub fn set_play_range(&self, start: c_int, end: c_int) {
		let f = self.api().set_play_range();
		unsafe { f(self.0, start, end) }
	}

	/// Pauses or resumes playback.
	///
	/// Equivalent to [setPaused](sys::ffi::playdate_sound_sampleplayer::setPaused)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setPaused")]
	pub fn set_paused(&self, value: bool) {
		let f = self.api().set_paused();
		unsafe { f(self.0, value as _) }
	}


	// callbacks //

	/// Sets a function to be called when playback has completed.
	///
	/// Equivalent to [setFinishCallback](sys::ffi::playdate_sound_sampleplayer::setFinishCallback)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setFinishCallback")]
	// TODO: Rustify this function, maybe impl like it done for sprites.
	// My idea is to store user-callback into self,
	// and here use proxy "extern C" function.
	// Maybe move self into the wrapper with user-callback?
	// We're need so store user-callback somewhere like StackedMap(stacked_type_map) or ErasedSet by type.
	// Type of `F: FnMut(*mut SoundSource)` is unique, so we can do it.
	// But with cost of memory - one static for each `F`*`Self`, so so much.
	pub fn set_finish_callback_raw(&self, callback: sndCallbackProc) -> Result<(), Error> {
		let f = self.api().set_finish_callback();
		Ok(unsafe { f(self.0, callback) })
	}

	/// Equivalent to [setLoopCallback](sys::ffi::playdate_sound_sampleplayer::setLoopCallback)
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setLoopCallback")]
	pub fn set_loop_callback_raw(&self, callback: sndCallbackProc) -> Result<(), Error> {
		let f = self.api().set_loop_callback();
		Ok(unsafe { f(self.0, callback) })
	}
}
