use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_void;

use sys::ffi::CString;
use sys::ffi::FilePlayer;
use sys::ffi::sndCallbackProc;
use sys::ffi::playdate_sound_fileplayer as Endpoint;

use fs::Path;

use super::Repeat;
use crate::error::Error;
use crate::error::ApiError;

mod cached;
mod api;

#[cfg(feature = "bindings-derive-cache")]
use cached as api;
use api::*;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Player<Api: FilePlayerApi = CachedEndpoint>(*mut FilePlayer, Api);


// ctor //

impl<Api> Player<Api>
	where Api: FilePlayerApi,
	      ApiError: From<<Api as FilePlayerApi>::Error>
{
	pub fn try_new() -> Result<Player<Api>, ApiError>
		where Api: TryFrom<&'static Endpoint>,
		      ApiError: From<<Api as TryFrom<&'static Endpoint>>::Error> {
		let api = Api::try_from(sys::api_ok!(sound.fileplayer)?)?;
		let new_player = api.try_new_player()?;
		let player = unsafe { new_player() };
		if player.is_null() {
			panic!("new player is null");
		}
		Ok(Player(player, api))
	}


	pub fn try_new_with(api: Api) -> Result<Player<Api>, ApiError> {
		let new_player = api.try_new_player()?;
		let player = unsafe { new_player() };
		if player.is_null() {
			panic!("new player is null");
		}
		Ok(Player(player, api))
	}
}


impl<Api: FilePlayerApi> Drop for Player<Api> {
	fn drop(&mut self) {
		if !self.0.is_null() {
			match self.api().try_free_player() {
				Ok(f) => {
					unsafe { f(self.0) }
					self.0 = core::ptr::null_mut();
				},
				Err(err) => println!("SP on drop: {err}"),
			}
		}
	}
}


// utils //

impl<Api: FilePlayerApi + Default> Default for Player<Api> {
	fn default() -> Self {
		let api = Api::default();
		let player = unsafe { api.try_new_player().expect("try_new_player")() };
		Self(player, api)
	}
}


impl<Api: FilePlayerApi> Player<Api> {
	#[inline(always)]
	pub fn api(&self) -> &Api { &self.1 }
}


// impl //

impl<Api> Player<Api>
	where Api: FilePlayerApi,
	      ApiError: From<<Api as FilePlayerApi>::Error>
{
	/// Prepares player to stream the file at path. Returns 1 if the file exists, otherwise 0.
	///
	/// See also [`loadIntoPlayer`](playdate_sys::ffi::playdate_sound_fileplayer::loadIntoPlayer).
	pub fn try_load_into_player<P: AsRef<Path>>(&self, path: P) -> Result<(), ApiError> {
		let path_cs = CString::new(path.as_ref())?;
		let path_ptr = path_cs.as_ptr() as *mut c_char;

		let f = self.api().try_load_into_player()?;
		let code = unsafe { f(self.0, path_ptr) };
		if code == 1 {
			Ok(())
		} else {
			Err(Error::FileNotExist.into())
		}
	}

	/// Sets the buffer length of player to bufferLen seconds.
	///
	/// See also [`setBufferLength`](playdate_sys::ffi::playdate_sound_fileplayer::setBufferLength).
	pub fn try_set_buffer_length(&self, len: c_float) -> Result<(), ApiError> {
		let f = self.api().try_set_buffer_length()?;
		Ok(unsafe { f(self.0, len) })
	}


	/// Returns `true` if player is playing a sample, `false` if not.
	///
	/// See also [`isPlaying`](playdate_sys::ffi::playdate_sound_fileplayer::isPlaying).
	pub fn try_is_playing(&self) -> Result<bool, ApiError> {
		let f = self.api().try_is_playing()?;
		Ok(unsafe { f(self.0) == 1 })
	}


	/// Starts playing the sample in player.
	/// Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.
	///
	/// See also [`play`](playdate_sys::ffi::playdate_sound_fileplayer::play).
	pub fn try_play(&self, repeat: Repeat) -> Result<c_int, ApiError> {
		let f = self.api().try_play()?;
		Ok(unsafe { f(self.0, repeat.into()) })
	}

	/// Stops playing the sample.
	///
	/// See also [`stop`](playdate_sys::ffi::playdate_sound_fileplayer::stop).
	pub fn try_stop(&self) -> Result<(), ApiError> {
		let f = self.api().try_stop()?;
		Ok(unsafe { f(self.0) })
	}


	/// Gets the current left and right channel volume of the player.
	///
	/// See also [`getVolume`](playdate_sys::ffi::playdate_sound_fileplayer::getVolume).
	pub fn try_get_volume(&self) -> Result<(c_float, c_float), ApiError> {
		let (mut left, mut right) = (0.0, 0.0);
		let f = self.api().try_get_volume()?;
		unsafe { f(self.0, &mut left, &mut right) };
		Ok((left, right))
	}

	/// Sets the playback volume for left and right channels.
	///
	/// See also [`setVolume`](playdate_sys::ffi::playdate_sound_fileplayer::setVolume).
	pub fn try_set_volume(&self, left: c_float, right: c_float) -> Result<(), ApiError> {
		let f = self.api().try_set_volume()?;
		Ok(unsafe { f(self.0, left, right) })
	}

	/// Returns the length, in seconds, of the sample assigned to player.
	///
	/// See also [`getLength`](playdate_sys::ffi::playdate_sound_fileplayer::getLength).
	pub fn try_get_length(&self) -> Result<c_float, ApiError> {
		let f = self.api().try_get_length()?;
		Ok(unsafe { f(self.0) })
	}

	/// Gets the current offset in seconds for player.
	///
	/// See also [`getOffset`](playdate_sys::ffi::playdate_sound_fileplayer::getOffset).
	pub fn try_get_offset(&self) -> Result<c_float, ApiError> {
		let f = self.api().try_get_offset()?;
		Ok(unsafe { f(self.0) })
	}

	/// Sets the current offset of the player, in seconds.
	///
	/// See also [`setOffset`](playdate_sys::ffi::playdate_sound_fileplayer::setOffset).
	pub fn try_set_offset(&self, offset: c_float) -> Result<(), ApiError> {
		let f = self.api().try_set_offset()?;
		Ok(unsafe { f(self.0, offset) })
	}

	/// Gets the playback rate for player.
	///
	/// See also [`getRate`](playdate_sys::ffi::playdate_sound_fileplayer::getRate).
	pub fn try_get_rate(&self) -> Result<c_float, ApiError> {
		let f = self.api().try_get_rate()?;
		Ok(unsafe { f(self.0) })
	}

	/// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.
	///
	/// See also [`setRate`](playdate_sys::ffi::playdate_sound_fileplayer::setRate).
	pub fn try_set_rate(&self, rate: c_float) -> Result<(), ApiError> {
		let f = self.api().try_set_rate()?;
		Ok(unsafe { f(self.0, rate) })
	}

	/// When used with a [`Repeat::PingPong`], does ping-pong looping, with a `start` and `end` position in frames.
	///
	/// See also [`setLoopRange`](playdate_sys::ffi::playdate_sound_fileplayer::setLoopRange).
	pub fn try_set_loop_range(&self, start: c_float, end: c_float) -> Result<(), ApiError> {
		let f = self.api().try_set_loop_range()?;
		Ok(unsafe { f(self.0, start, end) })
	}

	/// Returns `true` if player has underrun, `false` if not.
	pub fn try_did_underrun(&self) -> Result<bool, ApiError> {
		let f = self.api().try_did_underrun()?;
		Ok(unsafe { f(self.0) } == 1)
	}

	/// If value is `true`, the player will restart playback (after an audible stutter) as soon as data is available.
	pub fn try_set_stop_on_underrun(&self, value: bool) -> Result<(), ApiError> {
		let f = self.api().try_set_stop_on_underrun()?;
		Ok(unsafe { f(self.0, value as _) })
	}


	// callbacks //

	/// Sets a function to be called when playback has completed. See sndCallbackProc.
	///
	/// See also [`setFinishCallback`](playdate_sys::ffi::playdate_sound_fileplayer::setFinishCallback).
	// TODO: rustify this function
	pub fn try_set_finish_callback(&self, callback: sndCallbackProc) -> Result<(), ApiError> {
		let f = self.api().try_set_finish_callback()?;
		Ok(unsafe { f(self.0, callback) })
	}

	// TODO: rustify this function
	/// See also [`setLoopCallback`](playdate_sys::ffi::playdate_sound_fileplayer::setLoopCallback).
	pub fn try_set_loop_callback(&self, callback: sndCallbackProc) -> Result<(), ApiError> {
		let f = self.api().try_set_loop_callback()?;
		Ok(unsafe { f(self.0, callback) })
	}

	// TODO: rustify this function
	pub fn try_fade_volume(&self,
	                       left: c_float,
	                       right: c_float,
	                       len: i32,
	                       finish_callback: sndCallbackProc)
	                       -> Result<(), ApiError> {
		let f = self.api().try_fade_volume()?;
		Ok(unsafe { f(self.0, left, right, len, finish_callback) })
	}

	// TODO: rustify this function
	pub fn try_set_mp3_stream_source(&self,
	                                 source: Option<unsafe extern "C" fn(data: *mut u8,
	                                                             bytes: c_int,
	                                                             userdata: *mut c_void)
	                                                             -> c_int>,
	                                 userdata: *mut c_void,
	                                 buffer_len: c_float)
	                                 -> Result<(), ApiError> {
		let f = self.api().try_set_mp3_stream_source()?;
		Ok(unsafe { f(self.0, source, userdata, buffer_len) })
	}


	// TODO: try_set_mp3_stream_source
}
