use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ffi::c_void;
use core::ptr::NonNull;

use sys::ffi::FilePlayer;
use sys::ffi::sndCallbackProc;
use sys::ffi::playdate_sound_fileplayer;


/// Default file player api end-point, ZST.
///
/// All calls approximately costs ~4 derefs.
#[derive(Debug, Clone, Copy, core::default::Default)]
pub struct Default;
impl Api for Default {}


/// Cached file player api end-point.
///
/// Stores one reference, so size on stack is eq `usize`.
///
/// All calls approximately costs ~1 deref.
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Cache(&'static playdate_sound_fileplayer);

impl core::default::Default for Cache {
	fn default() -> Self { Self(sys::api!(sound.fileplayer)) }
}

impl From<*const playdate_sound_fileplayer> for Cache {
	#[inline(always)]
	fn from(ptr: *const playdate_sound_fileplayer) -> Self { Self(unsafe { ptr.as_ref() }.expect("sp")) }
}

impl From<&'static playdate_sound_fileplayer> for Cache {
	#[inline(always)]
	fn from(r: &'static playdate_sound_fileplayer) -> Self { Self(r) }
}

impl From<NonNull<playdate_sound_fileplayer>> for Cache {
	#[inline(always)]
	fn from(ptr: NonNull<playdate_sound_fileplayer>) -> Self { Self(unsafe { ptr.as_ref() }) }
}

impl From<&'_ NonNull<playdate_sound_fileplayer>> for Cache {
	#[inline(always)]
	fn from(ptr: &NonNull<playdate_sound_fileplayer>) -> Self { Self(unsafe { ptr.as_ref() }) }
}


impl Api for Cache {
	fn new_player(&self) -> FnNewPlayer { self.0.newPlayer.expect("newPlayer") }
	fn free_player(&self) -> FnFreePlayer { self.0.freePlayer.expect("freePlayer") }
	fn load_into_player(&self) -> FnLoadIntoPlayer { self.0.loadIntoPlayer.expect("loadIntoPlayer") }
	fn set_buffer_length(&self) -> FnSetBufferLength { self.0.setBufferLength.expect("setBufferLength") }
	fn play(&self) -> FnPlay { self.0.play.expect("play") }
	fn is_playing(&self) -> FnIsPlaying { self.0.isPlaying.expect("isPlaying") }
	fn stop(&self) -> FnStop { self.0.stop.expect("stop") }
	fn set_volume(&self) -> FnSetVolume { self.0.setVolume.expect("setVolume") }
	fn get_volume(&self) -> FnGetVolume { self.0.getVolume.expect("getVolume") }
	fn get_length(&self) -> FnGetLength { self.0.getLength.expect("getLength") }
	fn set_offset(&self) -> FnSetOffset { self.0.setOffset.expect("setOffset") }
	fn set_rate(&self) -> FnSetRate { self.0.setRate.expect("setRate") }
	fn set_loop_range(&self) -> FnSetLoopRange { self.0.setLoopRange.expect("setLoopRange") }
	fn did_underrun(&self) -> FnDidUnderrun { self.0.didUnderrun.expect("didUnderrun") }
	fn set_stop_on_underrun(&self) -> FnSetStopOnUnderrun { self.0.setStopOnUnderrun.expect("setStopOnUnderrun") }
	fn set_finish_callback(&self) -> FnSetFinishCallback { self.0.setFinishCallback.expect("setFinishCallback") }
	fn set_loop_callback(&self) -> FnSetLoopCallback { self.0.setLoopCallback.expect("setLoopCallback") }
	fn get_offset(&self) -> FnGetOffset { self.0.getOffset.expect("getOffset") }
	fn get_rate(&self) -> FnGetRate { self.0.getRate.expect("getRate") }
	fn fade_volume(&self) -> FnFadeVolume { self.0.fadeVolume.expect("fadeVolume") }
	fn set_mp3_stream_source(&self) -> FnSetMP3StreamSource {
		self.0.setMP3StreamSource.expect("setMP3StreamSource")
	}
}


pub trait Api {
	/// Returns [`sys::ffi::playdate_sound_fileplayer::newPlayer`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::newPlayer")]
	fn new_player(&self) -> FnNewPlayer { *sys::api!(sound.fileplayer.newPlayer) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::freePlayer`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::freePlayer")]
	fn free_player(&self) -> FnFreePlayer { *sys::api!(sound.fileplayer.freePlayer) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::loadIntoPlayer`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::loadIntoPlayer")]
	fn load_into_player(&self) -> FnLoadIntoPlayer { *sys::api!(sound.fileplayer.loadIntoPlayer) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setBufferLength`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setBufferLength")]
	fn set_buffer_length(&self) -> FnSetBufferLength { *sys::api!(sound.fileplayer.setBufferLength) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::play`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::play")]
	fn play(&self) -> FnPlay { *sys::api!(sound.fileplayer.play) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::isPlaying`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::isPlaying")]
	fn is_playing(&self) -> FnIsPlaying { *sys::api!(sound.fileplayer.isPlaying) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::stop`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::stop")]
	fn stop(&self) -> FnStop { *sys::api!(sound.fileplayer.stop) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setVolume")]
	fn set_volume(&self) -> FnSetVolume { *sys::api!(sound.fileplayer.setVolume) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::getVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getVolume")]
	fn get_volume(&self) -> FnGetVolume { *sys::api!(sound.fileplayer.getVolume) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::getLength`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getLength")]
	fn get_length(&self) -> FnGetLength { *sys::api!(sound.fileplayer.getLength) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setOffset`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setOffset")]
	fn set_offset(&self) -> FnSetOffset { *sys::api!(sound.fileplayer.setOffset) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setRate`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setRate")]
	fn set_rate(&self) -> FnSetRate { *sys::api!(sound.fileplayer.setRate) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setLoopRange`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setLoopRange")]
	fn set_loop_range(&self) -> FnSetLoopRange { *sys::api!(sound.fileplayer.setLoopRange) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::didUnderrun`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::didUnderrun")]
	fn did_underrun(&self) -> FnDidUnderrun { *sys::api!(sound.fileplayer.didUnderrun) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setStopOnUnderrun`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setStopOnUnderrun")]
	fn set_stop_on_underrun(&self) -> FnSetStopOnUnderrun { *sys::api!(sound.fileplayer.setStopOnUnderrun) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setFinishCallback`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setFinishCallback")]
	fn set_finish_callback(&self) -> FnSetFinishCallback { *sys::api!(sound.fileplayer.setFinishCallback) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setLoopCallback`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setLoopCallback")]
	fn set_loop_callback(&self) -> FnSetLoopCallback { *sys::api!(sound.fileplayer.setLoopCallback) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::getOffset`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getOffset")]
	fn get_offset(&self) -> FnGetOffset { *sys::api!(sound.fileplayer.getOffset) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::getRate`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::getRate")]
	fn get_rate(&self) -> FnGetRate { *sys::api!(sound.fileplayer.getRate) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::fadeVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::fadeVolume")]
	fn fade_volume(&self) -> FnFadeVolume { *sys::api!(sound.fileplayer.fadeVolume) }

	/// Returns [`sys::ffi::playdate_sound_fileplayer::setMP3StreamSource`]
	#[doc(alias = "sys::ffi::playdate_sound_fileplayer::setMP3StreamSource")]
	fn set_mp3_stream_source(&self) -> FnSetMP3StreamSource { *sys::api!(sound.fileplayer.setMP3StreamSource) }
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
