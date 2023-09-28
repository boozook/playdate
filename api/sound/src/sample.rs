//! Playdate sound sample API

use alloc::boxed::Box;
use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use core::ops::Deref;
use sys::ffi::CString;
use sys::ffi::AudioSample;
use fs::Path;
use sys::ffi::SoundFormat;

use crate::error::ApiError;
use crate::error::Error;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Sample<Api: api::Api = api::Default>(pub(super) *mut AudioSample, Api);


impl<Api: api::Api> Drop for Sample<Api> {
	fn drop(&mut self) {
		if !self.0.is_null() {
			let f = self.1.free_sample();
			unsafe { f(self.0) };
			self.0 = core::ptr::null_mut();
		}
	}
}


impl<Api: Default + api::Api> Sample<Api> {
	/// Allocates and returns a new [`Sample`] with a buffer large enough to load a file of length `bytes`.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::newSampleBuffer`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::newSampleBuffer")]
	#[inline(always)]
	pub fn new_with_size(bytes: c_int) -> Result<Self, Error> {
		let api: Api = Default::default();
		Self::new_with_size_with(api, bytes)
	}

	/// Retrieves `size` of file and allocate with that size.
	///
	/// __Does not load a file.__
	///
	/// Uses [`sys::ffi::playdate_sound_sample::newSampleBuffer`]
	#[inline(always)]
	pub fn new_for_file<P: AsRef<Path>>(path: P) -> Result<Self, ApiError> {
		let api: Api = Default::default();
		Self::new_for_file_with(api, path).map_err(Into::into)
	}

	/// Loads the file into the self.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::load`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::load")]
	#[inline(always)]
	pub fn new_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ApiError> {
		let api: Api = Default::default();
		Self::new_from_file_with(api, path)
	}


	/// Returns a new [`Sample`] referencing the given audio data.
	///
	/// The sample keeps a reference to the `data` instead of copying it,
	/// so the data must remain valid while the sample is active.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::newSampleFromData`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::newSampleFromData")]
	pub fn new_from_data<'t>(data: &'t mut [u8],
	                         format: SoundFormat,
	                         sample_rate: u32)
	                         -> Result<SampleWithData<'t, Api>, Error> {
		let api: Api = Default::default();
		Self::new_from_data_with(api, data, format, sample_rate)
	}
}


impl<Api: api::Api> Sample<Api> {
	/// Allocates and returns a new [`Sample`] with a buffer large enough to load a file of length `bytes`.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::newSampleBuffer`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::newSampleBuffer")]
	pub fn new_with_size_with(api: Api, bytes: c_int) -> Result<Self, Error> {
		let f = api.new_sample_buffer();
		let ptr = unsafe { f(bytes) };
		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(Self(ptr, api))
		}
	}

	/// Retrieves `size` of file and allocate with that size.
	///
	/// __Does not load a file.__
	///
	/// Uses [`sys::ffi::playdate_sound_sample::newSampleBuffer`]
	pub fn new_for_file_with<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, ApiError> {
		let size = match fs::metadata(path) {
			Ok(stats) => stats.size,
			Err(err) => {
				return match err {
					fs::error::ApiError::Api(err) => Err(ApiError::Api(err.into())),
					fs::error::ApiError::Utf8(err) => Err(ApiError::Utf8(err)),
					fs::error::ApiError::FromUtf8(err) => Err(ApiError::FromUtf8(err)),
					fs::error::ApiError::CStr(err) => Err(ApiError::CStr(err)),
					fs::error::ApiError::NullPtr(_) => Err(ApiError::NullPtr(sys::error::NullPtrError)),
				}
			}, // Err(err) => return Err(ApiError::from_err(err)),
		};

		Self::new_with_size_with(api, size as _).map_err(Into::into)
	}

	/// Loads the file into the self.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::load`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::load")]
	pub fn new_from_file_with<P: AsRef<Path>>(api: Api, path: P) -> Result<Self, ApiError> {
		let path_cs = CString::new(path.as_ref())?;
		let path_ptr = path_cs.as_ptr() as *mut c_char;

		let f = api.load();

		let ptr = unsafe { f(path_ptr) };
		if ptr.is_null() {
			Err(crate::error::Error::Alloc.into())
		} else {
			Ok(Self(ptr, api))
		}
	}


	/// Returns a new [`Sample`] referencing the given audio data.
	///
	/// The sample keeps a reference to the `data` instead of copying it,
	/// so the `data` __must remain valid while the sample is active__.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::newSampleFromData`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::newSampleFromData")]
	pub fn new_from_data_with<'t>(api: Api,
	                              data: &'t mut [u8],
	                              format: SoundFormat,
	                              sample_rate: u32)
	                              -> Result<SampleWithData<'t, Api>, Error> {
		let f = api.new_sample_from_data();
		let ptr = unsafe { f(data.as_mut_ptr(), format, sample_rate, data.len() as _) };

		if ptr.is_null() {
			Err(Error::Alloc)
		} else {
			Ok(SampleWithData(Self(ptr, api), data))
		}
	}
}


impl<Api: api::Api> Sample<Api> {
	/// Returns the length, in seconds.
	///
	/// Equivalent to [`sys::ffi::playdate_sound_sample::getLength`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::getLength")]
	pub fn length(&self) -> c_float {
		if self.0.is_null() {
			0.0
		} else {
			let f = self.1.get_length();
			unsafe { f(self.0) }
		}
	}

	/// Equivalent to [`sys::ffi::playdate_sound_sample::getData`]
	#[doc(alias = "sys::ffi::playdate_sound_sample::getData")]
	pub fn get_data<'t>(&'t self) -> SampleData<'t> {
		let mut format: SoundFormat = SoundFormat::kSound8bitMono;
		let mut sample_rate: u32 = 0;
		let mut byte_length: u32 = 0;

		let mut boxed_data = Box::new(core::ptr::null_mut());
		let data = Box::into_raw(boxed_data);

		let f = self.1.get_data();
		unsafe { f(self.0, data, &mut format, &mut sample_rate, &mut byte_length) };

		boxed_data = unsafe { Box::from_raw(data) };
		let data = unsafe { core::slice::from_raw_parts_mut::<u8>(*boxed_data, byte_length as usize) };

		SampleData { data, sample_rate }
	}
}


/// Sample over borrowed audio data.
#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct SampleWithData<'t, Api: api::Api>(Sample<Api>, &'t mut [u8]);

impl<Api: api::Api> Deref for SampleWithData<'_, Api> {
	type Target = Sample<Api>;
	fn deref(&self) -> &Self::Target { &self.0 }
}

impl<Api: api::Api> AsRef<Sample<Api>> for SampleWithData<'_, Api> {
	fn as_ref(&self) -> &Sample<Api> { &self.0 }
}


pub struct SampleData<'t> {
	pub sample_rate: u32,
	pub data: &'t mut [u8],
}


pub mod api {
	use core::ffi::c_int;
	use core::ffi::c_char;
	use core::ffi::c_float;
	use core::ptr::NonNull;
	use sys::ffi::AudioSample;
	use sys::ffi::SoundFormat;
	use sys::ffi::playdate_sound_sample;


	/// Default sound sample api end-point, ZST.
	///
	/// All calls approximately costs ~4 derefs.
	#[derive(Debug, Clone, Copy, core::default::Default)]
	pub struct Default;
	impl Api for Default {}


	/// Cached sound sample api end-point.
	///
	/// Stores one reference, so size on stack is eq `usize`.
	///
	/// All calls approximately costs ~1 deref.
	#[derive(Clone, Copy)]
	#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
	pub struct Cache(&'static playdate_sound_sample);

	impl core::default::Default for Cache {
		fn default() -> Self { Self(sys::api!(sound.sample)) }
	}

	impl From<*const playdate_sound_sample> for Cache {
		#[inline(always)]
		fn from(ptr: *const playdate_sound_sample) -> Self { Self(unsafe { ptr.as_ref() }.expect("sample")) }
	}

	impl From<&'static playdate_sound_sample> for Cache {
		#[inline(always)]
		fn from(r: &'static playdate_sound_sample) -> Self { Self(r) }
	}

	impl From<NonNull<playdate_sound_sample>> for Cache {
		#[inline(always)]
		fn from(ptr: NonNull<playdate_sound_sample>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}

	impl From<&'_ NonNull<playdate_sound_sample>> for Cache {
		#[inline(always)]
		fn from(ptr: &NonNull<playdate_sound_sample>) -> Self { Self(unsafe { ptr.as_ref() }) }
	}


	impl Api for Cache {
		fn new_sample_buffer(&self) -> unsafe extern "C" fn(byteCount: c_int) -> *mut AudioSample {
			self.0.newSampleBuffer.expect("newSampleBuffer")
		}

		fn load_into_sample(&self) -> unsafe extern "C" fn(sample: *mut AudioSample, path: *const c_char) -> c_int {
			self.0.loadIntoSample.expect("loadIntoSample")
		}

		fn load(&self) -> unsafe extern "C" fn(path: *const c_char) -> *mut AudioSample {
			self.0.load.expect("load")
		}

		fn new_sample_from_data(
			&self)
			-> unsafe extern "C" fn(data: *mut u8,
			                        format: SoundFormat,
			                        sampleRate: u32,
			                        byteCount: c_int) -> *mut AudioSample {
			self.0.newSampleFromData.expect("newSampleFromData")
		}

		fn get_data(
			&self)
			-> unsafe extern "C" fn(sample: *mut AudioSample,
			                        data: *mut *mut u8,
			                        format: *mut SoundFormat,
			                        sampleRate: *mut u32,
			                        bytelength: *mut u32) {
			self.0.getData.expect("getData")
		}

		fn free_sample(&self) -> unsafe extern "C" fn(sample: *mut AudioSample) {
			self.0.freeSample.expect("freeSample")
		}

		fn get_length(&self) -> unsafe extern "C" fn(sample: *mut AudioSample) -> c_float {
			self.0.getLength.expect("getLength")
		}
	}


	pub trait Api {
		/// Returns [`sys::ffi::playdate_sound_sample::newSampleBuffer`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::newSampleBuffer")]
		fn new_sample_buffer(&self) -> unsafe extern "C" fn(byteCount: c_int) -> *mut AudioSample {
			*sys::api!(sound.sample.newSampleBuffer)
		}


		/// Returns [`sys::ffi::playdate_sound_sample::loadIntoSample`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::loadIntoSample")]
		fn load_into_sample(&self) -> unsafe extern "C" fn(sample: *mut AudioSample, path: *const c_char) -> c_int {
			*sys::api!(sound.sample.loadIntoSample)
		}


		/// Returns [`sys::ffi::playdate_sound_sample::load`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::load")]
		fn load(&self) -> unsafe extern "C" fn(path: *const c_char) -> *mut AudioSample {
			*sys::api!(sound.sample.load)
		}


		/// Returns [`sys::ffi::playdate_sound_sample::newSampleFromData`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::newSampleFromData")]
		fn new_sample_from_data(
			&self)
			-> unsafe extern "C" fn(data: *mut u8,
			                        format: SoundFormat,
			                        sampleRate: u32,
			                        byteCount: c_int) -> *mut AudioSample {
			*sys::api!(sound.sample.newSampleFromData)
		}

		/// Returns [`sys::ffi::playdate_sound_sample::getData`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::getData")]
		fn get_data(
			&self)
			-> unsafe extern "C" fn(sample: *mut AudioSample,
			                        data: *mut *mut u8,
			                        format: *mut SoundFormat,
			                        sampleRate: *mut u32,
			                        bytelength: *mut u32) {
			*sys::api!(sound.sample.getData)
		}

		/// Returns [`sys::ffi::playdate_sound_sample::freeSample`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::freeSample")]
		fn free_sample(&self) -> unsafe extern "C" fn(sample: *mut AudioSample) {
			*sys::api!(sound.sample.freeSample)
		}

		/// Returns [`sys::ffi::playdate_sound_sample::getLength`]
		#[doc(alias = "sys::ffi::playdate_sound_sample::getLength")]
		fn get_length(&self) -> unsafe extern "C" fn(sample: *mut AudioSample) -> c_float {
			*sys::api!(sound.sample.getLength)
		}
	}
}
