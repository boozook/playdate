#![cfg_attr(not(test), no_std)]
#![feature(const_trait_impl)]

use core::ffi::c_int;
use core::ffi::c_void;

use sys::ffi::AudioSourceFunction;
use sys::traits::AsRaw;

extern crate sys;
extern crate alloc;

pub mod error;
pub mod player;
pub mod sample;
pub mod source;

// TODO: Sound api: channel, synth, sequence, effect, lfo, envelope, callbacks, etc..


pub mod prelude {
	pub use crate::error::ApiError as SndApiError;
	pub use crate::error::Error as SndError;

	pub use crate::player;
	pub use crate::sample;
}


#[derive(Debug, Clone, Copy)]
pub struct Sound<Api = api::Default>(Api);

impl Sound<api::Default> {
	/// Creates default [`Sound`] without type parameter requirement.
	///
	/// Uses ZST [`api::Default`].
	#[allow(non_snake_case)]
	pub fn Default() -> Self { Self(Default::default()) }
}

impl Sound<api::Cache> {
	/// Creates [`Sound`] without type parameter requirement.
	///
	/// Uses [`api::Cache`].
	#[allow(non_snake_case)]
	pub fn Cached() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Default for Sound<Api> {
	fn default() -> Self { Self(Default::default()) }
}

impl<Api: Default + api::Api> Sound<Api> {
	pub fn new() -> Self { Self(Default::default()) }
}

impl<Api: api::Api> Sound<Api> {
	pub fn new_with(api: Api) -> Self { Self(api) }
}

#[gen_api_shorthands::gen_shorthands]
impl<Api: api::Api> Sound<Api> {
	/// Returns the sound engine’s current time value, in units of sample frames, `44100` per second.
	///
	/// Equivalent to [`sys::ffi::playdate_sound::getCurrentTime`]
	#[doc(alias = "sys::ffi::playdate_sound::getCurrentTime")]
	pub fn current_time(&self) -> u32 {
		let f = self.0.get_current_time();
		unsafe { f() }
	}


	/// If `headphone` is `Some`, the value is set to 1 if headphones are currently plugged in.
	///
	/// Likewise, `mic` is set if the headphones include a microphone.
	///
	/// Example:
	/// ```no_run
	/// let mut headphone = Some(0);
	/// let mut mic = Some(0);
	/// sound.headphone_state(headphone.as_mut(), mic.as_mut());
	/// println!( "{}/{}", current_frame.unwrap(), mic.unwrap());
	/// ```
	/// See also [`Sound::set_headphone_state_change_callback`].
	#[doc(alias = "sys::ffi::playdate_sound::getHeadphoneState")]
	#[inline(always)]
	pub fn headphone_state(&self, headphone: Option<&mut c_int>, mic: Option<&mut c_int>) {
		self.set_headphone_state_change_callback(headphone, mic, None)
	}

	/// If `headphone` is `Some`, the value is set to 1 if headphones are currently plugged in.
	///
	/// Likewise, `mic` is set if the headphones include a microphone.
	///
	/// If `change_callback` is provided, it will be called when the headset or mic status changes,
	/// and audio output will not automatically switch from speaker to headphones when headphones are plugged in (and vice versa).
	///
	/// In this case, the callback should use [`Sound::set_outputs_active`] to change the output if needed.
	///
	/// Equivalent to [`sys::ffi::playdate_sound::getHeadphoneState`]
	#[doc(alias = "sys::ffi::playdate_sound::getHeadphoneState")]
	pub fn set_headphone_state_change_callback(&self,
	                                           headphone: Option<&mut c_int>,
	                                           mic: Option<&mut c_int>,
	                                           change_callback: Option<unsafe extern "C" fn(headphone: c_int,
	                                                                       mic: c_int)>) {
		use core::ptr::null_mut;

		let f = self.0.get_headphone_state();
		unsafe {
			f(
			  headphone.map_or(null_mut() as _, |v| v as *mut _),
			  mic.map_or(null_mut() as _, |v| v as *mut _),
			  change_callback,
			)
		}
	}

	/// Force audio output to the given outputs, regardless of headphone status.
	///
	/// Equivalent to [`sys::ffi::playdate_sound::setOutputsActive`]
	#[doc(alias = "sys::ffi::playdate_sound::setOutputsActive")]
	pub fn set_outputs_active(&self, headphone: bool, speaker: bool) {
		let f = self.0.set_outputs_active();
		unsafe { f(headphone.into(), speaker.into()) }
	}

	/// The callback function you pass in will be called every audio render cycle.
	///
	/// ```no_run
	/// // AudioSourceFunction:
	/// unsafe extern "C" fn(context: *mut c_void, left: *mut i16, right: *mut i16, len: c_int) -> c_int
	/// ```
	/// This function should fill the passed-in `left` buffer (and `right` if it’s a stereo source)
	/// with `len` samples each and return 1,
	/// or return 0 if the source is silent through the cycle.
	///
	/// Equivalent to [`sys::ffi::playdate_sound::addSource`]
	#[doc(alias = "sys::ffi::playdate_sound::addSource")]
	pub fn add_source_raw(&self,
	                      callback: AudioSourceFunction,
	                      context: *mut c_void,
	                      stereo: bool)
	                      -> source::SoundSource {
		let f = self.0.add_source();
		unsafe { f(callback, context, stereo.into()) }.into()
	}

	/// Removes the given [`SoundSource`](source::SoundSource) object from its channel,
	/// whether it’s in the default channel or a channel created with [`Sound::add_channel`].
	///
	/// Returns `true` if a source was removed, `false` if the source wasn’t in a channel.
	///
	/// Equivalent to [`sys::ffi::playdate_sound::removeSource`]
	#[doc(alias = "sys::ffi::playdate_sound::removeSource")]
	pub fn remove_source(&self, source: &source::SoundSource) -> bool {
		let f = self.0.remove_source();
		unsafe { f(source.as_raw()) == 1 }
	}


	// /// Returns the default channel, where sound sources play
	// /// if they haven’t been explicitly assigned to a different channel.
	// ///
	// /// Equivalent to [`sys::ffi::playdate_sound::getDefaultChannel`]
	// #[doc(alias = "sys::ffi::playdate_sound::getDefaultChannel")]
	// pub fn default_channel(&self) -> *mut SoundChannel {
	// 	let f = self.0.get_default_channel();
	// 	unsafe { f() }
	// }

	// /// Adds the given channel to the sound engine.
	// ///
	// /// Returns 1 if the channel was added, 0 if it was already in the engine.
	// ///
	// /// Equivalent to [`sys::ffi::playdate_sound::addChannel`]
	// #[doc(alias = "sys::ffi::playdate_sound::addChannel")]
	// pub fn add_channel(&self, channel: *mut SoundChannel) -> c_int {
	// 	let f = self.0.add_channel();
	// 	unsafe { f(channel) }
	// }

	// /// Removes the given channel from the sound engine.
	// ///
	// /// Returns 1 if the channel was successfully removed, 0 if the channel is the default channel or hadn’t been previously added.
	// ///
	// /// Equivalent to [`sys::ffi::playdate_sound::removeChannel`]
	// #[doc(alias = "sys::ffi::playdate_sound::removeChannel")]
	// pub fn remove_channel(&self, channel: *mut SoundChannel) -> c_int {
	// 	let f = self.0.remove_channel();
	// 	unsafe { f(channel) }
	// }

	// /// The callback you pass in will be called every audio cycle.
	// ///
	// /// Equivalent to [`sys::ffi::playdate_sound::setMicCallback`]
	// #[doc(alias = "sys::ffi::playdate_sound::setMicCallback")]
	// pub fn set_mic_callback(&self, callback: RecordCallback, context: *mut c_void, force_internal: c_int) {
	// 	let f = self.0.set_mic_callback();
	// 	unsafe { f(callback, context, force_internal) }
	// }
}


pub mod api {
	use core::ffi::c_int;
	use core::ffi::c_void;
	use core::ptr::NonNull;
	use sys::ffi::*;


	/// Default sound api end-point, ZST.
	///
	/// All calls approximately costs ~3 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached sound api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_sound);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(sound)) }
	}

	impl From<*const playdate_sound> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_sound) -> Self { Self(unsafe { ptr.as_ref() }.expect("sp")) }
	}

	impl From<&'static playdate_sound> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_sound) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_sound>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_sound>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_sound>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_sound>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}


	impl Api for Cache {
		#[inline(always)]
		fn channel(&self) -> &'static playdate_sound_channel {
			unsafe { self.0.channel.as_ref() }.expect("channel")
		}
		#[inline(always)]
		fn fileplayer(&self) -> &'static playdate_sound_fileplayer {
			unsafe { self.0.fileplayer.as_ref() }.expect("fileplayer")
		}
		#[inline(always)]
		fn sample(&self) -> &'static playdate_sound_sample { unsafe { self.0.sample.as_ref() }.expect("sample") }
		#[inline(always)]
		fn sampleplayer(&self) -> &'static playdate_sound_sampleplayer {
			unsafe { self.0.sampleplayer.as_ref() }.expect("sampleplayer")
		}
		#[inline(always)]
		fn synth(&self) -> &'static playdate_sound_synth { unsafe { self.0.synth.as_ref() }.expect("synth") }
		#[inline(always)]
		fn sequence(&self) -> &'static playdate_sound_sequence {
			unsafe { self.0.sequence.as_ref() }.expect("sequence")
		}
		#[inline(always)]
		fn effect(&self) -> &'static playdate_sound_effect { unsafe { self.0.effect.as_ref() }.expect("effect") }
		#[inline(always)]
		fn lfo(&self) -> &'static playdate_sound_lfo { unsafe { self.0.lfo.as_ref() }.expect("lfo") }
		#[inline(always)]
		fn envelope(&self) -> &'static playdate_sound_envelope {
			unsafe { self.0.envelope.as_ref() }.expect("envelope")
		}
		#[inline(always)]
		fn source(&self) -> &'static playdate_sound_source { unsafe { self.0.source.as_ref() }.expect("source") }
		#[inline(always)]
		fn control_signal(&self) -> &'static playdate_control_signal {
			unsafe { self.0.controlsignal.as_ref() }.expect("controlsignal")
		}
		#[inline(always)]
		fn track(&self) -> &'static playdate_sound_track { unsafe { self.0.track.as_ref() }.expect("track") }
		#[inline(always)]
		fn instrument(&self) -> &'static playdate_sound_instrument {
			unsafe { self.0.instrument.as_ref() }.expect("instrument")
		}
		#[inline(always)]
		fn signal(&self) -> &'static playdate_sound_signal { unsafe { self.0.signal.as_ref() }.expect("signal") }


		fn get_current_time(&self) -> unsafe extern "C" fn() -> u32 {
			self.0.getCurrentTime.expect("getCurrentTime")
		}

		fn add_source(
			&self)
			-> unsafe extern "C" fn(callback: AudioSourceFunction,
			                        context: *mut c_void,
			                        stereo: c_int) -> *mut SoundSource {
			self.0.addSource.expect("addSource")
		}

		fn get_default_channel(&self) -> unsafe extern "C" fn() -> *mut SoundChannel {
			self.0.getDefaultChannel.expect("getDefaultChannel")
		}

		fn add_channel(&self) -> unsafe extern "C" fn(channel: *mut SoundChannel) -> c_int {
			self.0.addChannel.expect("addChannel")
		}

		fn remove_channel(&self) -> unsafe extern "C" fn(channel: *mut SoundChannel) -> c_int {
			self.0.removeChannel.expect("removeChannel")
		}

		fn set_mic_callback(
			&self)
			-> unsafe extern "C" fn(callback: RecordCallback, context: *mut c_void, source: MicSource) -> c_int {
			self.0.setMicCallback.expect("setMicCallback")
		}

		fn get_headphone_state(
			&self)
			-> unsafe extern "C" fn(headphone: *mut c_int,
			                        headsetmic: *mut c_int,
			                        changeCallback: Option<unsafe extern "C" fn(headphone: c_int, mic: c_int)>) {
			self.0.getHeadphoneState.expect("getHeadphoneState")
		}

		fn set_outputs_active(&self) -> unsafe extern "C" fn(headphone: c_int, speaker: c_int) {
			self.0.setOutputsActive.expect("setOutputsActive")
		}

		fn remove_source(&self) -> unsafe extern "C" fn(source: *mut SoundSource) -> c_int {
			self.0.removeSource.expect("removeSource")
		}
	}


	pub trait Api {
		fn channel(&self) -> &'static playdate_sound_channel { sys::api!(sound.channel) }
		fn fileplayer(&self) -> &'static playdate_sound_fileplayer { sys::api!(sound.fileplayer) }
		fn sample(&self) -> &'static playdate_sound_sample { sys::api!(sound.sample) }
		fn sampleplayer(&self) -> &'static playdate_sound_sampleplayer { sys::api!(sound.sampleplayer) }
		fn synth(&self) -> &'static playdate_sound_synth { sys::api!(sound.synth) }
		fn sequence(&self) -> &'static playdate_sound_sequence { sys::api!(sound.sequence) }
		fn effect(&self) -> &'static playdate_sound_effect { sys::api!(sound.effect) }
		fn lfo(&self) -> &'static playdate_sound_lfo { sys::api!(sound.lfo) }
		fn envelope(&self) -> &'static playdate_sound_envelope { sys::api!(sound.envelope) }
		fn source(&self) -> &'static playdate_sound_source { sys::api!(sound.source) }
		fn control_signal(&self) -> &'static playdate_control_signal { sys::api!(sound.controlsignal) }
		fn track(&self) -> &'static playdate_sound_track { sys::api!(sound.track) }
		fn instrument(&self) -> &'static playdate_sound_instrument { sys::api!(sound.instrument) }
		fn signal(&self) -> &'static playdate_sound_signal { sys::api!(sound.signal) }

		/// Returns [`sys::ffi::playdate_sound::getCurrentTime`]
		#[doc(alias = "sys::ffi::playdate_sound::getCurrentTime")]
		fn get_current_time(&self) -> unsafe extern "C" fn() -> u32 { *sys::api!(sound.getCurrentTime) }

		/// Returns [`sys::ffi::playdate_sound::addSource`]
		#[doc(alias = "sys::ffi::playdate_sound::addSource")]
		fn add_source(
			&self)
			-> unsafe extern "C" fn(callback: AudioSourceFunction,
			                        context: *mut c_void,
			                        stereo: c_int) -> *mut SoundSource {
			*sys::api!(sound.addSource)
		}

		/// Returns [`sys::ffi::playdate_sound::getDefaultChannel`]
		#[doc(alias = "sys::ffi::playdate_sound::getDefaultChannel")]
		fn get_default_channel(&self) -> unsafe extern "C" fn() -> *mut SoundChannel {
			*sys::api!(sound.getDefaultChannel)
		}

		/// Returns [`sys::ffi::playdate_sound::addChannel`]
		#[doc(alias = "sys::ffi::playdate_sound::addChannel")]
		fn add_channel(&self) -> unsafe extern "C" fn(channel: *mut SoundChannel) -> c_int {
			*sys::api!(sound.addChannel)
		}

		/// Returns [`sys::ffi::playdate_sound::removeChannel`]
		#[doc(alias = "sys::ffi::playdate_sound::removeChannel")]
		fn remove_channel(&self) -> unsafe extern "C" fn(channel: *mut SoundChannel) -> c_int {
			*sys::api!(sound.removeChannel)
		}

		/// Returns [`sys::ffi::playdate_sound::setMicCallback`]
		#[doc(alias = "sys::ffi::playdate_sound::setMicCallback")]
		fn set_mic_callback(
			&self)
			-> unsafe extern "C" fn(callback: RecordCallback, context: *mut c_void, source: MicSource) -> c_int {
			*sys::api!(sound.setMicCallback)
		}

		/// Returns [`sys::ffi::playdate_sound::getHeadphoneState`]
		#[doc(alias = "sys::ffi::playdate_sound::getHeadphoneState")]
		fn get_headphone_state(
			&self)
			-> unsafe extern "C" fn(headphone: *mut c_int,
			                        headsetmic: *mut c_int,
			                        changeCallback: Option<unsafe extern "C" fn(headphone: c_int, mic: c_int)>) {
			*sys::api!(sound.getHeadphoneState)
		}

		/// Returns [`sys::ffi::playdate_sound::setOutputsActive`]
		#[doc(alias = "sys::ffi::playdate_sound::setOutputsActive")]
		fn set_outputs_active(&self) -> unsafe extern "C" fn(headphone: c_int, speaker: c_int) {
			*sys::api!(sound.setOutputsActive)
		}

		/// Returns [`sys::ffi::playdate_sound::removeSource`]
		#[doc(alias = "sys::ffi::playdate_sound::removeSource")]
		fn remove_source(&self) -> unsafe extern "C" fn(source: *mut SoundSource) -> c_int {
			*sys::api!(sound.removeSource)
		}
	}
}
