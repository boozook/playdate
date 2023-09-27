// TODO: support cache api with try

use core::ffi::c_char;
use core::ffi::c_float;
use core::ffi::c_int;
use fs::Path;
use sys::ffi::CString;
use sys::ffi::AudioSample;


#[cfg_attr(feature = "bindings-derive-debug", derive(Debug))]
pub struct Sample(pub(super) *mut AudioSample);


impl Drop for Sample {
	fn drop(&mut self) {
		if !self.0.is_null() {
			// TODO: use inner api instead
			let f = sys::api!(sound.sample.freeSample);
			unsafe { f(self.0) };
			self.0 = core::ptr::null_mut();
		}
	}
}


impl Sample {
	pub fn new_with_size(bytes: c_int) -> Self {
		let f = sys::api!(sound.sample.newSampleBuffer);
		let sample = unsafe { f(bytes) };
		if sample.is_null() {
			panic!("failed sample allocation");
		}
		Self(sample)
	}

	/// Takes `stats.size` for file and allocate with that size.
	///
	/// __Does not loads a file.__
	pub fn new_for_file<P: AsRef<Path>>(path: P) -> Self {
		let size = {
			fs::metadata(path).expect("fs metadata").size
		};

		Self::new_with_size(size as _)
	}

	/// Loads the file into the self.
	pub fn new_from_file<P: AsRef<Path>>(path: P) -> Self {
		let path_cs = CString::new(path.as_ref()).unwrap();
		let path_ptr = path_cs.as_ptr() as *mut c_char;

		let f = sys::api!(sound.sample.load);

		let ptr = unsafe { f(path_ptr) };
		assert!(!ptr.is_null(), "failed sample allocation/loading");

		Self(ptr)
	}


	// TODO: newSampleFromData
	// TODO: getData


	pub fn get_length(&self) -> c_float {
		if self.0.is_null() {
			0.0
		} else {
			let f = sys::api!(sound.sample.getLength);
			unsafe { f(self.0) }
		}
	}
}
