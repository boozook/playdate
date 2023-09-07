#![cfg(feature = "bindings-derive-cache")]

use core::ffi::c_float;
use core::ffi::c_int;

use sys::ffi::PlaydateSoundSampleplayer;
use sys::ffi::sndCallbackProc;
pub use sys::ffi::PlaydateSoundSampleplayerTry as SampleplayerApi;

use super::Repeat;
use super::Player;
use super::Endpoint;


/// Default cached sample-player api.
///
/// See also [`sys::ffi::PlaydateSoundSampleplayerCache`].
pub type CachedEndpoint = sys::cache::Ref<'static, Endpoint>;


// ctor //

impl<Api> Player<Api> where Api: PlaydateSoundSampleplayer {
	pub fn new() -> Player<Api>
		where Api: From<&'static Endpoint> {
		let api = Api::from(sys::apifn!(sound.sampleplayer));
		Self::new_with(api)
	}


	pub fn new_with(api: Api) -> Player<Api> {
		let new_player = api.new_player();
		let player = unsafe { new_player() };
		if player.is_null() {
			panic!("new player is null");
		}
		Player(player, api)
	}
}


// default impl //

impl<Api> Player<Api> where Api: PlaydateSoundSampleplayer {
	/// Assigns sample to player.
	///
	/// See also [`playdate_sound_sampleplayer::setSample`](playdate_sys::ffi::playdate_sound_sampleplayer::setSample).
	pub fn set_sample(&self, sample: &crate::sample::Sample) {
		let f = self.api().set_sample();
		unsafe { f(self.0, sample.0) }
	}


	/// Returns `true` if player is playing a sample, `false` if not.
	///
	/// See also [`playdate_sound_sampleplayer::isPlaying`](playdate_sys::ffi::playdate_sound_sampleplayer::isPlaying).
	pub fn is_playing(&self) -> bool {
		let f = self.api().is_playing();
		unsafe { f(self.0) == 1 }
	}


	/// Starts playing the sample in player.
	/// Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.
	///
	/// See also [`playdate_sound_sampleplayer::play`](playdate_sys::ffi::playdate_sound_sampleplayer::play).
	pub fn play(&self, repeat: Repeat, rate: c_float) -> c_int {
		let f = self.api().play();
		unsafe { f(self.0, repeat.into(), rate) }
	}

	/// Stops playing the sample.
	///
	/// See also [`playdate_sound_sampleplayer::stop`](playdate_sys::ffi::playdate_sound_sampleplayer::stop).
	pub fn stop(&self) {
		let f = self.api().stop();
		unsafe { f(self.0) }
	}


	/// Gets the current left and right channel volume of the player.
	///
	/// See also [`playdate_sound_sampleplayer::getVolume`](playdate_sys::ffi::playdate_sound_sampleplayer::getVolume).
	pub fn get_volume(&self) -> (c_float, c_float) {
		let (mut left, mut right) = (0.0, 0.0);
		let f = self.api().get_volume();
		unsafe { f(self.0, &mut left, &mut right) };
		(left, right)
	}

	/// Sets the playback volume for left and right channels.
	///
	/// See also [`playdate_sound_sampleplayer::setVolume`](playdate_sys::ffi::playdate_sound_sampleplayer::setVolume).
	pub fn set_volume(&self, left: c_float, right: c_float) {
		let f = self.api().set_volume();
		unsafe { f(self.0, left, right) }
	}

	/// Returns the length, in seconds, of the sample assigned to player.
	///
	/// See also [`playdate_sound_sampleplayer::getLength`](playdate_sys::ffi::playdate_sound_sampleplayer::getLength).
	pub fn get_length(&self) -> c_float {
		let f = self.api().get_length();
		unsafe { f(self.0) }
	}

	/// Gets the current offset in seconds for player.
	///
	/// See also [`playdate_sound_sampleplayer::getOffset`](playdate_sys::ffi::playdate_sound_sampleplayer::getOffset).
	pub fn get_offset(&self) -> c_float {
		let f = self.api().get_offset();
		unsafe { f(self.0) }
	}

	/// Sets the current offset of the player, in seconds.
	///
	/// See also [`playdate_sound_sampleplayer::setOffset`](playdate_sys::ffi::playdate_sound_sampleplayer::setOffset).
	pub fn set_offset(&self, offset: c_float) {
		let f = self.api().set_offset();
		unsafe { f(self.0, offset) }
	}

	/// Gets the playback rate for player.
	///
	/// See also [`playdate_sound_sampleplayer::getRate`](playdate_sys::ffi::playdate_sound_sampleplayer::getRate).
	pub fn get_rate(&self) -> c_float {
		let f = self.api().get_rate();
		unsafe { f(self.0) }
	}

	/// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.
	///
	/// See also [`playdate_sound_sampleplayer::setRate`](playdate_sys::ffi::playdate_sound_sampleplayer::setRate).
	pub fn set_rate(&self, rate: c_float) {
		let f = self.api().set_rate();
		unsafe { f(self.0, rate) }
	}

	/// When used with a [`Repeat::PingPong`], does ping-pong looping, with a `start` and `end` position in frames.
	///
	/// See also [`playdate_sound_sampleplayer::setPlayRange`](playdate_sys::ffi::playdate_sound_sampleplayer::setPlayRange).
	pub fn set_play_range(&self, start: c_int, end: c_int) {
		let f = self.api().set_play_range();
		unsafe { f(self.0, start, end) }
	}

	/// Pauses or resumes playback.
	///
	/// See also [`playdate_sound_sampleplayer::setPaused`](playdate_sys::ffi::playdate_sound_sampleplayer::setPaused).
	pub fn set_paused(&self, value: bool) {
		let f = self.api().set_paused();
		unsafe { f(self.0, value as _) }
	}


	// callbacks //

	/// Sets a function to be called when playback has completed. See sndCallbackProc.
	///
	/// See also [`playdate_sound_sampleplayer::setFinishCallback`](playdate_sys::ffi::playdate_sound_sampleplayer::setFinishCallback).
	// TODO: rustify this function
	// pub fn set_finish_callback(&self, callback: Option<unsafe extern "C" fn(c: *mut SoundSource)>) {
	pub fn set_finish_callback(&self, callback: sndCallbackProc) {
		let f = self.api().set_finish_callback();
		unsafe { f(self.0, callback) }
	}

	// TODO: rustify this function
	/// See also [`playdate_sound_sampleplayer::setLoopCallback`](playdate_sys::ffi::playdate_sound_sampleplayer::setLoopCallback).
	pub fn set_loop_callback(&self, callback: sndCallbackProc) {
		let f = self.api().set_loop_callback();
		unsafe { f(self.0, callback) }
	}
}
