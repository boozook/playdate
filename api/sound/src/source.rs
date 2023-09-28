//! Playdate sound-source API

use core::ffi::c_float;

use sys::ffi::sndCallbackProc;
use sys::ffi::SoundSource as OpaqueSoundSource;
use sys::traits::AsRaw;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct SoundSource<Api = api::Default>(*mut OpaqueSoundSource, Api);

impl<Api> AsRaw for SoundSource<Api> {
	type Type = OpaqueSoundSource;
	unsafe fn as_raw(&self) -> *mut Self::Type { self.0 }
}

impl<Api: Default> From<*mut OpaqueSoundSource> for SoundSource<Api> {
	fn from(ptr: *mut OpaqueSoundSource) -> Self { Self(ptr, Default::default()) }
}

impl<Api: api::Api> SoundSource<Api> {
	pub fn from_with(api: Api, ptr: *mut OpaqueSoundSource) -> Self { Self(ptr, api) }

	/// Returns `true` if the source is currently playing.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_source::isPlaying`]
	#[doc(alias = "sys::ffi::playdate_sound_source::isPlaying")]
	pub fn is_playing(&self) -> bool {
		let f = self.1.is_playing();
		unsafe { f(self.0) == 1 }
	}

	/// Gets the playback volume (`0.0` - `1.0`) for `left` and `right` channels of the source.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_source::getVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_source::getVolume")]
	pub fn get_volume(&self) -> (c_float, c_float) {
		let mut l = 0.;
		let mut r = 0.;
		let f = self.1.get_volume();
		unsafe { f(self.0, &mut l, &mut r) };
		(l, r)
	}

	/// Sets the playback volume (`0.0` - `1.0`) for `left` and `right` channels of the source.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_source::setVolume`]
	#[doc(alias = "sys::ffi::playdate_sound_source::setVolume")]
	pub fn set_volume(&self, left: c_float, right: c_float) {
		let f = self.1.set_volume();
		unsafe { f(self.0, left, right) }
	}


	/// Equivalent to [`sys::ffi::playdate_sound_source::setFinishCallback`]
	#[doc(alias = "sys::ffi::playdate_sound_source::setFinishCallback")]
	pub fn set_finish_callback_raw(&self, callback: sndCallbackProc) {
		let f = self.1.set_finish_callback();
		unsafe { f(self.0, callback) }
	}
}


pub mod api {
	use core::ffi::c_float;
	use core::ptr::NonNull;

	use sys::ffi::SoundSource;
	use sys::ffi::sndCallbackProc;
	use sys::ffi::playdate_sound_source;

	/// Default sound source api end-point, ZST.
	///
	/// All calls approximately costs ~4 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached sound source api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_sound_source);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(sound.source)) }
	}

	impl From<*const playdate_sound_source> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_sound_source) -> Self { Self(unsafe { ptr.as_ref() }.expect("snd.src")) }
	}

	impl From<&'static playdate_sound_source> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_sound_source) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_sound_source>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_sound_source>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_sound_source>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_sound_source>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}


	impl Api for Cache {
		#[inline(always)]
		fn set_volume(&self) -> unsafe extern "C" fn(c: *mut SoundSource, lvol: c_float, rvol: c_float) {
			self.0.setVolume.expect("setVolume")
		}

		#[inline(always)]
		fn get_volume(&self) -> unsafe extern "C" fn(c: *mut SoundSource, outl: *mut c_float, outr: *mut c_float) {
			self.0.getVolume.expect("getVolume")
		}

		#[inline(always)]
		fn is_playing(&self) -> unsafe extern "C" fn(c: *mut SoundSource) -> core::ffi::c_int {
			self.0.isPlaying.expect("isPlaying")
		}

		#[inline(always)]
		fn set_finish_callback(&self) -> unsafe extern "C" fn(c: *mut SoundSource, callback: sndCallbackProc) {
			self.0.setFinishCallback.expect("setFinishCallback")
		}
	}


	pub trait Api {
		/// Returns [`sys::ffi::playdate_sound_source::setVolume`]
		#[doc(alias = "sys::ffi::playdate_sound_source::setVolume")]
		#[inline(always)]
		fn set_volume(&self) -> unsafe extern "C" fn(c: *mut SoundSource, lvol: c_float, rvol: c_float) {
			*sys::api!(sound.source.setVolume)
		}

		/// Returns [`sys::ffi::playdate_sound_source::getVolume`]
		#[doc(alias = "sys::ffi::playdate_sound_source::getVolume")]
		#[inline(always)]
		fn get_volume(&self) -> unsafe extern "C" fn(c: *mut SoundSource, outl: *mut c_float, outr: *mut c_float) {
			*sys::api!(sound.source.getVolume)
		}

		/// Returns [`sys::ffi::playdate_sound_source::isPlaying`]
		#[doc(alias = "sys::ffi::playdate_sound_source::isPlaying")]
		#[inline(always)]
		fn is_playing(&self) -> unsafe extern "C" fn(c: *mut SoundSource) -> core::ffi::c_int {
			*sys::api!(sound.source.isPlaying)
		}

		/// Returns [`sys::ffi::playdate_sound_source::setFinishCallback`]
		#[doc(alias = "sys::ffi::playdate_sound_source::setFinishCallback")]
		#[inline(always)]
		fn set_finish_callback(&self) -> unsafe extern "C" fn(c: *mut SoundSource, callback: sndCallbackProc) {
			*sys::api!(sound.source.setFinishCallback)
		}
	}
}
