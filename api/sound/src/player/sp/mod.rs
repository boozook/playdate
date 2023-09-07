use core::ffi::c_float;
use core::ffi::c_int;

use sys::ffi::SamplePlayer;
use sys::ffi::sndCallbackProc;
use sys::ffi::playdate_sound_sampleplayer as Endpoint;

use crate::error::ApiError as Error;
use super::Repeat;


mod cached;
mod api;

#[cfg(feature = "bindings-derive-cache")]
use cached as api;
use api::*;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Player<Api: SampleplayerApi = CachedEndpoint>(*mut SamplePlayer, Api);


// ctor //

impl<Api> Player<Api>
	where Api: SampleplayerApi,
	      Error: From<<Api as SampleplayerApi>::Error>
{
	pub fn try_new() -> Result<Player<Api>, Error>
		where Api: TryFrom<&'static Endpoint>,
		      Error: From<<Api as TryFrom<&'static Endpoint>>::Error> {
		let api = Api::try_from(sys::api_ok!(sound.sampleplayer)?)?;
		let new_player = api.try_new_player()?;
		let player = unsafe { new_player() };
		if player.is_null() {
			Err(crate::error::Error::Alloc.into())
		} else {
			Ok(Player(player, api))
		}
	}


	pub fn try_new_with(api: Api) -> Result<Player<Api>, Error> {
		let new_player = api.try_new_player()?;
		let player = unsafe { new_player() };
		if player.is_null() {
			Err(crate::error::Error::Alloc.into())
		} else {
			Ok(Player(player, api))
		}
	}
}


impl<Api: SampleplayerApi> Drop for Player<Api> {
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

impl<Api: SampleplayerApi + Default> Default for Player<Api> {
	fn default() -> Self {
		let api = Api::default();
		let player = unsafe { api.try_new_player().expect("try_new_player")() };
		Self(player, api)
	}
}


impl<Api: SampleplayerApi> Player<Api> {
	#[inline(always)]
	pub fn api(&self) -> &Api { &self.1 }
}


// impl //

impl<Api> Player<Api>
	where Api: SampleplayerApi,
	      Error: From<<Api as SampleplayerApi>::Error>
{
	/// Assigns sample to player.
	///
	/// See also [`setSample`](playdate_sys::ffi::playdate_sound_sampleplayer::setSample).
	pub fn try_set_sample(&self, sample: &crate::sample::Sample) -> Result<(), Error> {
		let f = self.api().try_set_sample()?;
		Ok(unsafe { f(self.0, sample.0) })
	}


	/// Returns `true` if player is playing a sample, `false` if not.
	///
	/// See also [`isPlaying`](playdate_sys::ffi::playdate_sound_sampleplayer::isPlaying).
	pub fn try_is_playing(&self) -> Result<bool, Error> {
		let f = self.api().try_is_playing()?;
		Ok(unsafe { f(self.0) == 1 })
	}


	/// Starts playing the sample in player.
	/// Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.
	///
	/// See also [`play`](playdate_sys::ffi::playdate_sound_sampleplayer::play).
	pub fn try_play(&self, repeat: Repeat, rate: c_float) -> Result<c_int, Error> {
		let f = self.api().try_play()?;
		Ok(unsafe { f(self.0, repeat.into(), rate) })
	}

	/// Stops playing the sample.
	///
	/// See also [`stop`](playdate_sys::ffi::playdate_sound_sampleplayer::stop).
	pub fn try_stop(&self) -> Result<(), Error> {
		let f = self.api().try_stop()?;
		Ok(unsafe { f(self.0) })
	}


	/// Gets the current left and right channel volume of the player.
	///
	/// See also [`getVolume`](playdate_sys::ffi::playdate_sound_sampleplayer::getVolume).
	pub fn try_get_volume(&self) -> Result<(c_float, c_float), Error> {
		let (mut left, mut right) = (0.0, 0.0);
		let f = self.api().try_get_volume()?;
		unsafe { f(self.0, &mut left, &mut right) };
		Ok((left, right))
	}

	/// Sets the playback volume for left and right channels.
	///
	/// See also [`setVolume`](playdate_sys::ffi::playdate_sound_sampleplayer::setVolume).
	pub fn try_set_volume(&self, left: c_float, right: c_float) -> Result<(), Error> {
		let f = self.api().try_set_volume()?;
		Ok(unsafe { f(self.0, left, right) })
	}

	/// Returns the length, in seconds, of the sample assigned to player.
	///
	/// See also [`getLength`](playdate_sys::ffi::playdate_sound_sampleplayer::getLength).
	pub fn try_get_length(&self) -> Result<c_float, Error> {
		let f = self.api().try_get_length()?;
		Ok(unsafe { f(self.0) })
	}

	/// Gets the current offset in seconds for player.
	///
	/// See also [`getOffset`](playdate_sys::ffi::playdate_sound_sampleplayer::getOffset).
	pub fn try_get_offset(&self) -> Result<c_float, Error> {
		let f = self.api().try_get_offset()?;
		Ok(unsafe { f(self.0) })
	}

	/// Sets the current offset of the player, in seconds.
	///
	/// See also [`setOffset`](playdate_sys::ffi::playdate_sound_sampleplayer::setOffset).
	pub fn try_set_offset(&self, offset: c_float) -> Result<(), Error> {
		let f = self.api().try_set_offset()?;
		Ok(unsafe { f(self.0, offset) })
	}

	/// Gets the playback rate for player.
	///
	/// See also [`getRate`](playdate_sys::ffi::playdate_sound_sampleplayer::getRate).
	pub fn try_get_rate(&self) -> Result<c_float, Error> {
		let f = self.api().try_get_rate()?;
		Ok(unsafe { f(self.0) })
	}

	/// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.
	///
	/// See also [`setRate`](playdate_sys::ffi::playdate_sound_sampleplayer::setRate).
	pub fn try_set_rate(&self, rate: c_float) -> Result<(), Error> {
		let f = self.api().try_set_rate()?;
		Ok(unsafe { f(self.0, rate) })
	}

	/// When used with a [`Repeat::PingPong`], does ping-pong looping, with a `start` and `end` position in frames.
	///
	/// See also [`setPlayRange`](playdate_sys::ffi::playdate_sound_sampleplayer::setPlayRange).
	pub fn try_set_play_range(&self, start: c_int, end: c_int) -> Result<(), Error> {
		let f = self.api().try_set_play_range()?;
		Ok(unsafe { f(self.0, start, end) })
	}

	/// Pauses or resumes playback.
	///
	/// See also [`setPaused`](playdate_sys::ffi::playdate_sound_sampleplayer::setPaused).
	pub fn try_set_paused(&self, value: bool) -> Result<(), Error> {
		let f = self.api().try_set_paused()?;
		Ok(unsafe { f(self.0, value as _) })
	}


	// callbacks //

	/// Sets a function to be called when playback has completed. See sndCallbackProc.
	///
	/// See also [`setFinishCallback`](playdate_sys::ffi::playdate_sound_sampleplayer::setFinishCallback).
	// TODO: rustify this function
	// My idea is to store user-callback into self,
	// and here use proxy "extern C" function.
	// Maybe move self into the wrapper with user-callback?
	// We're need so store user-callback somewhere like StackedMap(stacked_type_map) or ErasedSet by type.
	// Type of `F: FnMut(*mut SoundSource)` is unique, so we can do it.
	// But with cost of memory - one static for each `F`*`Self`, so so much.
	pub fn try_set_finish_callback(&self, callback: sndCallbackProc) -> Result<(), Error> {
		let f = self.api().try_set_finish_callback()?;
		Ok(unsafe { f(self.0, callback) })
	}

	// TODO: rustify this function
	/// See also [`setLoopCallback`](playdate_sys::ffi::playdate_sound_sampleplayer::setLoopCallback).
	pub fn try_set_loop_callback(&self, callback: sndCallbackProc) -> Result<(), Error> {
		let f = self.api().try_set_loop_callback()?;
		Ok(unsafe { f(self.0, callback) })
	}
}
