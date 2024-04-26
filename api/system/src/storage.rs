use core::ffi::c_char;
use core::ffi::CStr;
use alloc::string::String;
use erased_set::ErasedSendSet;

pub static mut STORE: Option<ErasedSendSet> = None;

pub fn init_store() {
	if unsafe { STORE.is_none() } {
		unsafe { STORE = Some(ErasedSendSet::new()) }
	}
}

pub fn clean_store() {
	if let Some(true) = unsafe { STORE.as_mut() }.map(|store| store.is_empty()) {
		unsafe { STORE = None }
	}
}

pub unsafe extern "C" fn proxy_serial_message_callback<F: 'static + Send + FnMut(String)>(data: *const c_char) {
	let data = CStr::from_ptr(data as _).to_string_lossy().into_owned();
	let f = unsafe { STORE.as_mut() }.map(|store| store.remove::<F>())
	                                 .flatten();
	f.map(|mut f| f(data)).or_else(|| panic!("missed callback"));
	clean_store();
}
