#![cfg(not(feature = "bindings-derive-cache"))]

use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_void;

use sys::ffi::FilePlayer;
use sys::ffi::sndCallbackProc;
use sys::error::NullPtrError as Error;
use sys::error::OkOrNullFnErr;
use super::Endpoint;


/// Default cached sample-player api.
pub type CachedEndpoint = Ref<'static>;


pub trait FilePlayerApi {
	type Error: core::error::Error;

	fn try_new_player(&self) -> Result<&FnNewPlayer, Self::Error>;
	fn try_free_player(&self) -> Result<&FnFreePlayer, Self::Error>;
	fn try_load_into_player(&self) -> Result<&FnLoadIntoPlayer, Self::Error>;
	fn try_set_buffer_length(&self) -> Result<&FnSetBufferLength, Self::Error>;
	fn try_play(&self) -> Result<&FnPlay, Self::Error>;
	fn try_is_playing(&self) -> Result<&FnIsPlaying, Self::Error>;
	fn try_stop(&self) -> Result<&FnStop, Self::Error>;
	fn try_set_volume(&self) -> Result<&FnSetVolume, Self::Error>;
	fn try_get_volume(&self) -> Result<&FnGetVolume, Self::Error>;
	fn try_get_length(&self) -> Result<&FnGetLength, Self::Error>;
	fn try_set_offset(&self) -> Result<&FnSetOffset, Self::Error>;
	fn try_set_rate(&self) -> Result<&FnSetRate, Self::Error>;
	fn try_set_loop_range(&self) -> Result<&FnSetLoopRange, Self::Error>;
	fn try_did_underrun(&self) -> Result<&FnDidUnderrun, Self::Error>;
	fn try_set_stop_on_underrun(&self) -> Result<&FnSetStopOnUnderrun, Self::Error>;
	fn try_set_finish_callback(&self) -> Result<&FnSetFinishCallback, Self::Error>;
	fn try_set_loop_callback(&self) -> Result<&FnSetLoopCallback, Self::Error>;
	fn try_get_offset(&self) -> Result<&FnGetOffset, Self::Error>;
	fn try_get_rate(&self) -> Result<&FnGetRate, Self::Error>;
	fn try_fade_volume(&self) -> Result<&FnFadeVolume, Self::Error>;
	fn try_set_mp3_stream_source(&self) -> Result<&FnSetMP3StreamSource, Self::Error>;
}


#[derive(Clone, Copy)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Ref<'t: 'static>(&'t Endpoint);


impl<'t> From<&'t Endpoint> for Ref<'t> {
	fn from(api: &'t Endpoint) -> Self { Self(api) }
}


impl<'t> FilePlayerApi for Ref<'t> {
	type Error = self::Error;

	fn try_new_player(&self) -> Result<&FnNewPlayer, Error> { self.0.newPlayer.as_ref().ok_or_null() }
	fn try_free_player(&self) -> Result<&FnFreePlayer, Error> { self.0.freePlayer.as_ref().ok_or_null() }
	fn try_load_into_player(&self) -> Result<&FnLoadIntoPlayer, Error> {
		self.0.loadIntoPlayer.as_ref().ok_or_null()
	}
	fn try_set_buffer_length(&self) -> Result<&FnSetBufferLength, Error> {
		self.0.setBufferLength.as_ref().ok_or_null()
	}
	fn try_play(&self) -> Result<&FnPlay, Error> { self.0.play.as_ref().ok_or_null() }
	fn try_is_playing(&self) -> Result<&FnIsPlaying, Error> { self.0.isPlaying.as_ref().ok_or_null() }
	fn try_stop(&self) -> Result<&FnStop, Error> { self.0.stop.as_ref().ok_or_null() }
	fn try_set_volume(&self) -> Result<&FnSetVolume, Error> { self.0.setVolume.as_ref().ok_or_null() }
	fn try_get_volume(&self) -> Result<&FnGetVolume, Error> { self.0.getVolume.as_ref().ok_or_null() }
	fn try_get_length(&self) -> Result<&FnGetLength, Error> { self.0.getLength.as_ref().ok_or_null() }
	fn try_set_offset(&self) -> Result<&FnSetOffset, Error> { self.0.setOffset.as_ref().ok_or_null() }
	fn try_set_rate(&self) -> Result<&FnSetRate, Error> { self.0.setRate.as_ref().ok_or_null() }
	fn try_set_loop_range(&self) -> Result<&FnSetLoopRange, Error> { self.0.setLoopRange.as_ref().ok_or_null() }
	fn try_did_underrun(&self) -> Result<&FnDidUnderrun, Error> { self.0.didUnderrun.as_ref().ok_or_null() }
	fn try_set_stop_on_underrun(&self) -> Result<&FnSetStopOnUnderrun, Error> {
		self.0.setStopOnUnderrun.as_ref().ok_or_null()
	}
	fn try_set_finish_callback(&self) -> Result<&FnSetFinishCallback, Error> {
		self.0.setFinishCallback.as_ref().ok_or_null()
	}
	fn try_set_loop_callback(&self) -> Result<&FnSetLoopCallback, Error> {
		self.0.setLoopCallback.as_ref().ok_or_null()
	}
	fn try_get_offset(&self) -> Result<&FnGetOffset, Error> { self.0.getOffset.as_ref().ok_or_null() }
	fn try_get_rate(&self) -> Result<&FnGetRate, Error> { self.0.getRate.as_ref().ok_or_null() }
	fn try_fade_volume(&self) -> Result<&FnFadeVolume, Error> { self.0.fadeVolume.as_ref().ok_or_null() }
	fn try_set_mp3_stream_source(&self) -> Result<&FnSetMP3StreamSource, Error> {
		self.0.setMP3StreamSource.as_ref().ok_or_null()
	}
}


type FnNewPlayer = unsafe extern "C" fn() -> *mut FilePlayer;
type FnFreePlayer = unsafe extern "C" fn(player: *mut FilePlayer);

type FnLoadIntoPlayer = unsafe extern "C" fn(player: *mut FilePlayer, path: *const c_char) -> c_int;
type FnSetBufferLength = unsafe extern "C" fn(player: *mut FilePlayer, bufferLen: c_float);

type FnPlay = unsafe extern "C" fn(player: *mut FilePlayer, repeat: c_int) -> c_int;
type FnIsPlaying = unsafe extern "C" fn(player: *mut FilePlayer) -> c_int;
type FnStop = unsafe extern "C" fn(player: *mut FilePlayer);
type FnSetVolume = unsafe extern "C" fn(player: *mut FilePlayer, left: c_float, right: c_float);
type FnGetVolume = unsafe extern "C" fn(player: *mut FilePlayer, left: *mut c_float, right: *mut c_float);
type FnGetLength = unsafe extern "C" fn(player: *mut FilePlayer) -> c_float;
type FnSetOffset = unsafe extern "C" fn(player: *mut FilePlayer, offset: c_float);
type FnSetRate = unsafe extern "C" fn(player: *mut FilePlayer, rate: c_float);
type FnSetLoopRange = unsafe extern "C" fn(player: *mut FilePlayer, start: c_float, end: c_float);
type FnDidUnderrun = unsafe extern "C" fn(player: *mut FilePlayer) -> c_int;
type FnSetStopOnUnderrun = unsafe extern "C" fn(player: *mut FilePlayer, flag: c_int);
type FnSetFinishCallback = unsafe extern "C" fn(player: *mut FilePlayer, callback: sndCallbackProc);
type FnSetLoopCallback = unsafe extern "C" fn(player: *mut FilePlayer, callback: sndCallbackProc);
type FnGetOffset = unsafe extern "C" fn(player: *mut FilePlayer) -> c_float;
type FnGetRate = unsafe extern "C" fn(player: *mut FilePlayer) -> c_float;
type FnFadeVolume = unsafe extern "C" fn(player: *mut FilePlayer,
                                         left: c_float,
                                         right: c_float,
                                         len: i32,
                                         finishCallback: sndCallbackProc);
type FnSetMP3StreamSource = unsafe extern "C" fn(player: *mut FilePlayer,
                                                 dataSource: Option<FnDataSource>,
                                                 userdata: *mut c_void,
                                                 bufferLen: c_float);
type FnDataSource = unsafe extern "C" fn(data: *mut u8, bytes: c_int, userdata: *mut c_void) -> c_int;
