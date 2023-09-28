use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_void;

use sys::ffi::CString;
use sys::ffi::FilePlayer;
use sys::ffi::sndCallbackProc;

use fs::Path;

use super::Repeat;
use crate::error::ApiError;
use crate::error::Error;

pub mod api;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Player<Api: api::Api = api::Default>(*mut FilePlayer, Api);


// ctor //

impl<Api> Player<Api> where Api: api::Api {
	pub fn new() -> Result<Player<Api>, Error>
		where Api: Default {
		let api = Api::default();
		Self::new_with(api)
	}

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
	/// Prepares player to stream the file at path.
	///
	/// Equivalent to [loadIntoPlayer](sys::ffi::playdate_sound_fileplayer::loadIntoPlayer)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::loadIntoPlayer")]
	pub fn load_into_player<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path_cs = CString::new(path.as_ref())?;
		let path_ptr = path_cs.as_ptr() as *mut c_char;

		let f = self.api().load_into_player();
		let code = unsafe { f(self.0, path_ptr) };
		if code == 1 {
			Ok(())
		} else {
			Err(Error::FileNotExist.into())
		}
	}

	/// Sets the buffer length of player to `len` seconds.
	///
	/// Equivalent to [setBufferLength](sys::ffi::playdate_sound_fileplayer::setBufferLength)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setBufferLength")]
	pub fn set_buffer_length(&self, len: c_float) {
		let f = self.api().set_buffer_length();
		unsafe { f(self.0, len) }
	}


	/// Returns `true` if player is playing.
	///
	/// Equivalent to [isPlaying](sys::ffi::playdate_sound_fileplayer::isPlaying)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::isPlaying")]
	pub fn is_playing(&self) -> bool {
		let f = self.api().is_playing();
		unsafe { f(self.0) == 1 }
	}


	/// Starts playing the file player.
	///
	/// Equivalent to [play](sys::ffi::playdate_sound_fileplayer::play)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::play")]
	pub fn play(&self, repeat: Repeat) -> c_int {
		let f = self.api().play();
		unsafe { f(self.0, repeat.into()) }
	}

	/// Stops playing the file.
	///
	/// Equivalent to [stop](sys::ffi::playdate_sound_fileplayer::stop)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::stop")]
	pub fn stop(&self) {
		let f = self.api().stop();
		unsafe { f(self.0) }
	}


	/// Gets the left and right channel playback volume for player.
	///
	/// Equivalent to [getVolume](sys::ffi::playdate_sound_fileplayer::getVolume)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getVolume")]
	pub fn volume(&self) -> (c_float, c_float) {
		let (mut left, mut right) = (0.0, 0.0);
		let f = self.api().get_volume();
		unsafe { f(self.0, &mut left, &mut right) };
		(left, right)
	}

	/// Sets the playback volume for left and right channels of player.
	///
	/// Equivalent to [setVolume](sys::ffi::playdate_sound_fileplayer::setVolume)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setVolume")]
	pub fn set_volume(&self, left: c_float, right: c_float) {
		let f = self.api().set_volume();
		unsafe { f(self.0, left, right) }
	}

	/// Returns the length, in seconds, of the file loaded into player.
	///
	/// Equivalent to [getLength](sys::ffi::playdate_sound_fileplayer::getLength)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getLength")]
	pub fn length(&self) -> c_float {
		let f = self.api().get_length();
		unsafe { f(self.0) }
	}

	/// Gets the current offset in seconds for player.
	///
	/// Equivalent to [getOffset](sys::ffi::playdate_sound_fileplayer::getOffset)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getOffset")]
	pub fn offset(&self) -> c_float {
		let f = self.api().get_offset();
		unsafe { f(self.0) }
	}

	/// Sets the current offset in seconds.
	///
	/// Equivalent to [setOffset](sys::ffi::playdate_sound_fileplayer::setOffset)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setOffset")]
	pub fn set_offset(&self, offset: c_float) {
		let f = self.api().set_offset();
		unsafe { f(self.0, offset) }
	}

	/// Gets the playback rate for player.
	///
	/// Equivalent to [getRate](sys::ffi::playdate_sound_fileplayer::getRate)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getRate")]
	pub fn rate(&self) -> c_float {
		let f = self.api().get_rate();
		unsafe { f(self.0) }
	}

	/// Sets the playback rate for the player.
	///
	/// `1.0` is normal speed, `0.5` is down an octave, `2.0` is up an octave, etc.
	///
	/// Unlike [`SamplePlayer`](crate::player::SamplePlayer),
	/// [`FilePlayer`](crate::player::FilePlayer)s can’t play in reverse (i.e., rate < 0).
	///
	/// Equivalent to [setRate](sys::ffi::playdate_sound_fileplayer::setRate)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setRate")]
	pub fn set_rate(&self, rate: c_float) {
		let f = self.api().set_rate();
		unsafe { f(self.0, rate) }
	}

	/// Sets the `start` and `end` of the loop region for playback, in seconds.
	///
	/// If end is omitted, the end of the file is used.
	///
	/// Equivalent to [setLoopRange](sys::ffi::playdate_sound_fileplayer::setLoopRange)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setLoopRange")]
	pub fn set_loop_range(&self, start: c_float, end: c_float) {
		let f = self.api().set_loop_range();
		unsafe { f(self.0, start, end) }
	}

	/// Returns `true` if player has underrun, `false` if not.
	///
	/// Equivalent to [didUnderrun](sys::ffi::playdate_sound_fileplayer::didUnderrun)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::didUnderrun")]
	pub fn did_underrun(&self) -> bool {
		let f = self.api().did_underrun();
		unsafe { f(self.0) == 1 }
	}

	/// If value is `true`, the player will restart playback (after an audible stutter) as soon as data is available.
	///
	/// Equivalent to [setStopOnUnderrun](sys::ffi::playdate_sound_fileplayer::setStopOnUnderrun)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setStopOnUnderrun")]
	pub fn set_stop_on_underrun(&self, value: bool) {
		let f = self.api().set_stop_on_underrun();
		unsafe { f(self.0, value as _) }
	}


	// callbacks //

	// TODO: rustify this functions

	/// Sets a function to be called when playback has completed.
	///
	/// This is an alias for [`sys::ffi::playdate_sound_source::setFinishCallback`].
	///
	/// Equivalent to [setFinishCallback](sys::ffi::playdate_sound_fileplayer::setFinishCallback)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setFinishCallback")]
	pub fn set_finish_callback(&self, callback: sndCallbackProc) {
		let f = self.api().set_finish_callback();
		unsafe { f(self.0, callback) }
	}

	/// Equivalent to [setLoopCallback](sys::ffi::playdate_sound_fileplayer::setLoopCallback)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setLoopCallback")]
	pub fn set_loop_callback(&self, callback: sndCallbackProc) {
		let f = self.api().set_loop_callback();
		unsafe { f(self.0, callback) }
	}

	/// Changes the volume of the [`Player`] to `left` and `right` over a length of `len` sample frames,
	/// then calls the provided `callback` (if set).
	///
	/// Equivalent to [fadeVolume](sys::ffi::playdate_sound_fileplayer::fadeVolume)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::fadeVolume")]
	// Probably here we can use just FnOnce, because it will dropped after call by proxy.
	pub fn fade_volume(&self, left: c_float, right: c_float, len: i32, finish_callback: sndCallbackProc) {
		let f = self.api().fade_volume();
		unsafe { f(self.0, left, right, len, finish_callback) }
	}

	/// Equivalent to [setMP3StreamSource](sys::ffi::playdate_sound_fileplayer::setMP3StreamSource)
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setMP3StreamSource")]
	pub fn set_mp3_stream_source(&self,
	                             source: Option<unsafe extern "C" fn(data: *mut u8,
	                                                         bytes: c_int,
	                                                         userdata: *mut c_void)
	                                                         -> c_int>,
	                             userdata: *mut c_void,
	                             buffer_len: c_float) {
		let f = self.api().set_mp3_stream_source();
		unsafe { f(self.0, source, userdata, buffer_len) }
	}
}
