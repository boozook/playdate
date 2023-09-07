#![cfg(not(feature = "bindings-derive-cache"))]

use core::ffi::c_float;
use core::ffi::c_int;

use sys::ffi::SamplePlayer;
use sys::ffi::sndCallbackProc;
use sys::ffi::AudioSample;
use sys::error::NullPtrError;
use sys::error::OkOrNullFnErr;
use super::Endpoint;


/// Default cached sample-player api.
pub type CachedEndpoint = Ref<'static>;


pub trait SampleplayerApi {
	type Error: core::error::Error;

	fn try_new_player(&self) -> Result<&FnNewPlayer, Self::Error>;
	fn try_free_player(&self) -> Result<&FnFreePlayer, Self::Error>;
	fn try_set_sample(&self) -> Result<&FnSetSample, Self::Error>;
	fn try_play(&self) -> Result<&FnPlay, Self::Error>;
	fn try_is_playing(&self) -> Result<&FnIsPlaying, Self::Error>;
	fn try_stop(&self) -> Result<&FnStop, Self::Error>;
	fn try_set_volume(&self) -> Result<&FnSetVolume, Self::Error>;
	fn try_get_volume(&self) -> Result<&FnGetVolume, Self::Error>;
	fn try_get_length(&self) -> Result<&FnGetLength, Self::Error>;
	fn try_set_offset(&self) -> Result<&FnSetOffset, Self::Error>;
	fn try_set_rate(&self) -> Result<&FnSetRate, Self::Error>;
	fn try_set_play_range(&self) -> Result<&FnSetPlayRange, Self::Error>;
	fn try_set_finish_callback(&self) -> Result<&FnSetFinishCallback, Self::Error>;
	fn try_set_loop_callback(&self) -> Result<&FnSetLoopCallback, Self::Error>;
	fn try_get_offset(&self) -> Result<&FnGetOffset, Self::Error>;
	fn try_get_rate(&self) -> Result<&FnGetRate, Self::Error>;
	fn try_set_paused(&self) -> Result<&FnSetPaused, Self::Error>;
}


#[derive(Clone, Copy)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Ref<'t: 'static>(&'t Endpoint);


impl<'t> From<&'t Endpoint> for Ref<'t> {
	fn from(api: &'t Endpoint) -> Self { Self(api) }
}


impl<'t> SampleplayerApi for Ref<'t> {
	type Error = NullPtrError;

	fn try_new_player(&self) -> Result<&FnNewPlayer, NullPtrError> { self.0.newPlayer.as_ref().ok_or_null() }
	fn try_free_player(&self) -> Result<&FnFreePlayer, NullPtrError> { self.0.freePlayer.as_ref().ok_or_null() }
	fn try_set_sample(&self) -> Result<&FnSetSample, NullPtrError> { self.0.setSample.as_ref().ok_or_null() }
	fn try_play(&self) -> Result<&FnPlay, NullPtrError> { self.0.play.as_ref().ok_or_null() }
	fn try_is_playing(&self) -> Result<&FnIsPlaying, NullPtrError> { self.0.isPlaying.as_ref().ok_or_null() }
	fn try_stop(&self) -> Result<&FnStop, NullPtrError> { self.0.stop.as_ref().ok_or_null() }
	fn try_set_volume(&self) -> Result<&FnSetVolume, NullPtrError> { self.0.setVolume.as_ref().ok_or_null() }
	fn try_get_volume(&self) -> Result<&FnGetVolume, NullPtrError> { self.0.getVolume.as_ref().ok_or_null() }
	fn try_get_length(&self) -> Result<&FnGetLength, NullPtrError> { self.0.getLength.as_ref().ok_or_null() }
	fn try_set_offset(&self) -> Result<&FnSetOffset, NullPtrError> { self.0.setOffset.as_ref().ok_or_null() }
	fn try_set_rate(&self) -> Result<&FnSetRate, NullPtrError> { self.0.setRate.as_ref().ok_or_null() }
	fn try_set_play_range(&self) -> Result<&FnSetPlayRange, NullPtrError> {
		self.0.setPlayRange.as_ref().ok_or_null()
	}
	fn try_set_finish_callback(&self) -> Result<&FnSetFinishCallback, NullPtrError> {
		self.0.setFinishCallback.as_ref().ok_or_null()
	}
	fn try_set_loop_callback(&self) -> Result<&FnSetLoopCallback, NullPtrError> {
		self.0.setLoopCallback.as_ref().ok_or_null()
	}
	fn try_get_offset(&self) -> Result<&FnGetOffset, NullPtrError> { self.0.getOffset.as_ref().ok_or_null() }
	fn try_get_rate(&self) -> Result<&FnGetRate, NullPtrError> { self.0.getRate.as_ref().ok_or_null() }
	fn try_set_paused(&self) -> Result<&FnSetPaused, NullPtrError> { self.0.setPaused.as_ref().ok_or_null() }
}


type FnNewPlayer = unsafe extern "C" fn() -> *mut SamplePlayer;
type FnFreePlayer = unsafe extern "C" fn(player: *mut SamplePlayer);
type FnSetSample = unsafe extern "C" fn(player: *mut SamplePlayer, sample: *mut AudioSample);
type FnPlay = unsafe extern "C" fn(player: *mut SamplePlayer, repeat: c_int, rate: c_float) -> c_int;
type FnIsPlaying = unsafe extern "C" fn(player: *mut SamplePlayer) -> c_int;
type FnStop = unsafe extern "C" fn(player: *mut SamplePlayer);
type FnSetVolume = unsafe extern "C" fn(player: *mut SamplePlayer, left: c_float, right: c_float);
type FnGetVolume = unsafe extern "C" fn(player: *mut SamplePlayer, left: *mut c_float, right: *mut c_float);
type FnGetLength = unsafe extern "C" fn(player: *mut SamplePlayer) -> c_float;
type FnSetOffset = unsafe extern "C" fn(player: *mut SamplePlayer, offset: c_float);
type FnSetRate = unsafe extern "C" fn(player: *mut SamplePlayer, rate: c_float);
type FnSetPlayRange = unsafe extern "C" fn(player: *mut SamplePlayer, start: c_int, end: c_int);
type FnSetFinishCallback = unsafe extern "C" fn(player: *mut SamplePlayer, callback: sndCallbackProc);
type FnSetLoopCallback = unsafe extern "C" fn(player: *mut SamplePlayer, callback: sndCallbackProc);
type FnGetOffset = unsafe extern "C" fn(player: *mut SamplePlayer) -> c_float;
type FnGetRate = unsafe extern "C" fn(player: *mut SamplePlayer) -> c_float;
type FnSetPaused = unsafe extern "C" fn(player: *mut SamplePlayer, flag: c_int);
