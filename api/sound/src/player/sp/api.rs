use core::ffi::c_void;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ptr::NonNull;

use sys::ffi::AudioSample;
use sys::ffi::SamplePlayer;
use sys::ffi::sndCallbackProc;
use sys::ffi::playdate_sound_sampleplayer;


/// Default sample player api end-point, ZST.
///
/// All calls approximately costs ~4 derefs.
#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;
impl Api for Default {}


/// Cached sample player api end-point.
///
/// Stores one reference, so size on stack is eq `usize`.
///
/// All calls approximately costs ~1 deref.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Cache(&'static playdate_sound_sampleplayer);

impl core::default::Default for Cache {
	fn default() -> Self { Self(sys::api!(sound.sampleplayer)) }
}

impl From<*const playdate_sound_sampleplayer> for Cache {
	#[inline(always)]
	fn from(ptr: *const playdate_sound_sampleplayer) -> Self { Self(unsafe { ptr.as_ref() }.expect("sp")) }
}

impl From<&'static playdate_sound_sampleplayer> for Cache {
	#[inline(always)]
	fn from(r: &'static playdate_sound_sampleplayer) -> Self { Self(r) }
}

impl From<NonNull<playdate_sound_sampleplayer>> for Cache {
	#[inline(always)]
	fn from(ptr: NonNull<playdate_sound_sampleplayer>) -> Self { Self(unsafe { ptr.as_ref() }) }
}

impl From<&'_ NonNull<playdate_sound_sampleplayer>> for Cache {
	#[inline(always)]
	fn from(ptr: &NonNull<playdate_sound_sampleplayer>) -> Self { Self(unsafe { ptr.as_ref() }) }
}


impl Api for Cache {
	fn new_player(&self) -> FnNewPlayer { self.0.newPlayer.expect("newPlayer") }
	fn free_player(&self) -> FnFreePlayer { self.0.freePlayer.expect("freePlayer") }
	fn set_sample(&self) -> FnSetSample { self.0.setSample.expect("setSample") }
	fn play(&self) -> FnPlay { self.0.play.expect("play") }
	fn is_playing(&self) -> FnIsPlaying { self.0.isPlaying.expect("isPlaying") }
	fn stop(&self) -> FnStop { self.0.stop.expect("stop") }
	fn set_volume(&self) -> FnSetVolume { self.0.setVolume.expect("setVolume") }
	fn get_volume(&self) -> FnGetVolume { self.0.getVolume.expect("getVolume") }
	fn get_length(&self) -> FnGetLength { self.0.getLength.expect("getLength") }
	fn set_offset(&self) -> FnSetOffset { self.0.setOffset.expect("setOffset") }
	fn set_rate(&self) -> FnSetRate { self.0.setRate.expect("setRate") }
	fn set_play_range(&self) -> FnSetPlayRange { self.0.setPlayRange.expect("setPlayRange") }
	fn set_finish_callback(&self) -> FnSetFinishCallback { self.0.setFinishCallback.expect("setFinishCallback") }
	fn set_loop_callback(&self) -> FnSetLoopCallback { self.0.setLoopCallback.expect("setLoopCallback") }
	fn get_offset(&self) -> FnGetOffset { self.0.getOffset.expect("getOffset") }
	fn get_rate(&self) -> FnGetRate { self.0.getRate.expect("getRate") }
	fn set_paused(&self) -> FnSetPaused { self.0.setPaused.expect("setPaused") }
}


pub trait Api {
	/// Returns [`sys::ffi::playdate_sound_sampleplayer::newPlayer`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::newPlayer")]
	fn new_player(&self) -> FnNewPlayer { *sys::api!(sound.sampleplayer.newPlayer) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::freePlayer`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::freePlayer")]
	fn free_player(&self) -> FnFreePlayer { *sys::api!(sound.sampleplayer.freePlayer) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setSample`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setSample")]
	fn set_sample(&self) -> FnSetSample { *sys::api!(sound.sampleplayer.setSample) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::play`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::play")]
	fn play(&self) -> FnPlay { *sys::api!(sound.sampleplayer.play) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::isPlaying`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::isPlaying")]
	fn is_playing(&self) -> FnIsPlaying { *sys::api!(sound.sampleplayer.isPlaying) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::stop`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::stop")]
	fn stop(&self) -> FnStop { *sys::api!(sound.sampleplayer.stop) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setVolume")]
	fn set_volume(&self) -> FnSetVolume { *sys::api!(sound.sampleplayer.setVolume) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::getVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getVolume")]
	fn get_volume(&self) -> FnGetVolume { *sys::api!(sound.sampleplayer.getVolume) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::getLength`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getLength")]
	fn get_length(&self) -> FnGetLength { *sys::api!(sound.sampleplayer.getLength) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setOffset`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setOffset")]
	fn set_offset(&self) -> FnSetOffset { *sys::api!(sound.sampleplayer.setOffset) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setRate`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setRate")]
	fn set_rate(&self) -> FnSetRate { *sys::api!(sound.sampleplayer.setRate) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setPlayRange`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setPlayRange")]
	fn set_play_range(&self) -> FnSetPlayRange { *sys::api!(sound.sampleplayer.setPlayRange) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setFinishCallback`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setFinishCallback")]
	fn set_finish_callback(&self) -> FnSetFinishCallback { *sys::api!(sound.sampleplayer.setFinishCallback) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setLoopCallback`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setLoopCallback")]
	fn set_loop_callback(&self) -> FnSetLoopCallback { *sys::api!(sound.sampleplayer.setLoopCallback) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::getOffset`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getOffset")]
	fn get_offset(&self) -> FnGetOffset { *sys::api!(sound.sampleplayer.getOffset) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::getRate`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::getRate")]
	fn get_rate(&self) -> FnGetRate { *sys::api!(sound.sampleplayer.getRate) }

	/// Returns [`sys::ffi::playdate_sound_sampleplayer::setPaused`]
	#[doc(alias = "sys::ffi::playdate_sound_sampleplayer::setPaused")]
	fn set_paused(&self) -> FnSetPaused { *sys::api!(sound.sampleplayer.setPaused) }
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
type FnSetFinishCallback = unsafe extern "C" fn(player: *mut SamplePlayer, callback: sndCallbackProc, userdata: *mut c_void);
type FnSetLoopCallback = unsafe extern "C" fn(player: *mut SamplePlayer, callback: sndCallbackProc, userdata: *mut c_void);
type FnGetOffset = unsafe extern "C" fn(player: *mut SamplePlayer) -> c_float;
type FnGetRate = unsafe extern "C" fn(player: *mut SamplePlayer) -> c_float;
type FnSetPaused = unsafe extern "C" fn(player: *mut SamplePlayer, flag: c_int);
