#![cfg(feature = "bindings-derive-cache")]

use core::ffi::c_void;
use core::ffi::c_float;
use core::ffi::c_int;

use sys::ffi::sndCallbackProc;
use sys::ffi::PlaydateSoundFileplayer;
pub use sys::ffi::PlaydateSoundFileplayerTry as FilePlayerApi;

use super::Repeat;
use super::Player;
use super::Endpoint;


/// Default cached file-player api.
///
/// See also [`sys::ffi::PlaydateSoundFileplayerCache`].
pub type CachedEndpoint = sys::cache::Ref<'static, Endpoint>;


// ctor //

impl<Api> Player<Api> where Api: PlaydateSoundFileplayer {
	pub fn new() -> Player<Api>
		where Api: From<&'static Endpoint> {
		let api = Api::from(sys::apifn!(sound.fileplayer));
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

impl<Api> Player<Api> where Api: PlaydateSoundFileplayer {
	/// Sets the buffer length of player to bufferLen seconds.
	pub fn set_buffer_length(&self, len: c_float) {
		let f = self.api().set_buffer_length();
		unsafe { f(self.0, len) }
	}


	/// Returns `true` if player is playing a sample, `false` if not.
	///
	/// See also [`isPlaying`](playdate_sys::ffi::playdate_sound_fileplayer::isPlaying).
	pub fn is_playing(&self) -> bool {
		let f = self.api().is_playing();
		unsafe { f(self.0) == 1 }
	}


	/// Starts playing the sample in player.
	/// Sets the playback rate for the sample. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc.
	///
	/// See also [`play`](playdate_sys::ffi::playdate_sound_fileplayer::play).
	pub fn play(&self, repeat: Repeat) -> c_int {
		let f = self.api().play();
		unsafe { f(self.0, repeat.into()) }
	}

	/// Stops playing the sample.
	///
	/// See also [`stop`](playdate_sys::ffi::playdate_sound_fileplayer::stop).
	pub fn stop(&self) {
		let f = self.api().stop();
		unsafe { f(self.0) }
	}


	/// Gets the current left and right channel volume of the player.
	///
	/// See also [`getVolume`](playdate_sys::ffi::playdate_sound_fileplayer::getVolume).
	pub fn get_volume(&self) -> (c_float, c_float) {
		let (mut left, mut right) = (0.0, 0.0);
		let f = self.api().get_volume();
		unsafe { f(self.0, &mut left, &mut right) };
		(left, right)
	}

	/// Sets the playback volume for left and right channels.
	///
	/// See also [`setVolume`](playdate_sys::ffi::playdate_sound_fileplayer::setVolume).
	pub fn set_volume(&self, left: c_float, right: c_float) {
		let f = self.api().set_volume();
		unsafe { f(self.0, left, right) }
	}

	/// Returns the length, in seconds, of the sample assigned to player.
	///
	/// See also [`getLength`](playdate_sys::ffi::playdate_sound_fileplayer::getLength).
	pub fn get_length(&self) -> c_float {
		let f = self.api().get_length();
		unsafe { f(self.0) }
	}

	/// Gets the current offset in seconds for player.
	///
	/// See also [`getOffset`](playdate_sys::ffi::playdate_sound_fileplayer::getOffset).
	pub fn get_offset(&self) -> c_float {
		let f = self.api().get_offset();
		unsafe { f(self.0) }
	}

	/// Sets the current offset of the player, in seconds.
	///
	/// See also [`setOffset`](playdate_sys::ffi::playdate_sound_fileplayer::setOffset).
	pub fn set_offset(&self, offset: c_float) {
		let f = self.api().set_offset();
		unsafe { f(self.0, offset) }
	}

	/// Gets the playback rate for player.
	///
	/// See also [`getRate`](playdate_sys::ffi::playdate_sound_fileplayer::getRate).
	pub fn get_rate(&self) -> c_float {
		let f = self.api().get_rate();
		unsafe { f(self.0) }
	}

	/// Sets the playback rate for the player. 1.0 is normal speed, 0.5 is down an octave, 2.0 is up an octave, etc. A negative rate produces backwards playback for PCM files, but does not work for ADPCM-encoded files.
	///
	/// See also [`setRate`](playdate_sys::ffi::playdate_sound_fileplayer::setRate).
	pub fn set_rate(&self, rate: c_float) {
		let f = self.api().set_rate();
		unsafe { f(self.0, rate) }
	}

	/// When used with a [`Repeat::PingPong`], does ping-pong looping, with a `start` and `end` position in frames.
	///
	/// See also [`setPlayRange`](playdate_sys::ffi::playdate_sound_fileplayer::setPlayRange).
	pub fn set_loop_range(&self, start: c_float, end: c_float) {
		let f = self.api().set_loop_range();
		unsafe { f(self.0, start, end) }
	}

	/// Returns `true` if player has underrun, `false` if not.
	pub fn did_underrun(&self) -> bool {
		let f = self.api().did_underrun();
		unsafe { f(self.0) == 1 }
	}

	/// If value is `true`, the player will restart playback (after an audible stutter) as soon as data is available.
	pub fn set_stop_on_underrun(&self, value: bool) {
		let f = self.api().set_stop_on_underrun();
		unsafe { f(self.0, value as _) }
	}


	// callbacks //

	/// Sets a function to be called when playback has completed. See sndCallbackProc.
	///
	/// See also [`setFinishCallback`](playdate_sys::ffi::playdate_sound_fileplayer::setFinishCallback).
	// TODO: rustify this function
	// pub fn set_finish_callback(&self, callback: Option<unsafe extern "C" fn(c: *mut SoundSource)>)
	pub fn set_finish_callback(&self, callback: sndCallbackProc) {
		let f = self.api().set_finish_callback();
		unsafe { f(self.0, callback) }
	}

	// TODO: rustify this function
	/// See also [`setLoopCallback`](playdate_sys::ffi::playdate_sound_fileplayer::setLoopCallback).
	pub fn set_loop_callback(&self, callback: sndCallbackProc) {
		let f = self.api().set_loop_callback();
		unsafe { f(self.0, callback) }
	}

	// TODO: rustify this function
	pub fn fade_volume(&self, left: c_float, right: c_float, len: i32, finish_callback: sndCallbackProc) {
		let f = self.api().fade_volume();
		unsafe { f(self.0, left, right, len, finish_callback) }
	}

	// TODO: rustify this function
	pub fn et_mp3_stream_source(&self,
	                            source: Option<unsafe extern "C" fn(data: *mut u8,
	                                                        bytes: c_int,
	                                                        userdata: *mut c_void)
	                                                        -> c_int>,
	                            userdata: *mut c_void,
	                            buffer_len: c_float) {
		let f = self.api().set_mp3_stream_source();
		unsafe { f(self.0, source, userdata, buffer_len) }
	}

	// TODO: try_set_mp3_stream_source
}
