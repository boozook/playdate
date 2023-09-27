#![no_std]
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate sys;
extern crate playdate_fs as fs;

use core::ffi::*;
use core::ptr::null_mut;
use alloc::string::String;

use sys::ffi::*;
use fs::prelude::*;


fn list_bundle_dir() -> Result<(), fs::error::ApiError> {
	let fs = Fs::Default();
	let include_hidden = true;
	println!("Listing root dir...");
	fs.read_dir("/", |path| println!("  {path}"), include_hidden)?;
	Ok(())
}


const DIR: &Path = "temp";
const FILE: &Path = "temp/temp-file";


fn write_file() -> Result<(), fs::error::ApiError> {
	let fs = Fs::Cached();

	let exists = fs.metadata(FILE).is_ok();

	if exists {
		println!("file already exists, overwriting");
	} else {
		println!("file doesn't exists, creating new");

		// create dir:
		fs.create_dir(DIR)?;
	}

	let text = "Hello, World!";
	println!("writing '{text}' to '{FILE}'");

	let mut file = fs.open(FILE, FileOptions::new().write(true))?;
	let bytes_written = fs.write(&mut file, text.as_bytes())?;
	println!("written {bytes_written} bytes");

	Ok(())
}


fn read_file() -> Result<(), fs::error::ApiError> {
	let fs = Fs::Cached();

	println!("reading file metadata");
	let info = fs.metadata(&FILE)?;
	println!("info: {info:?}");


	// prepare prefilled buffer:
	println!("preparing buffer for {} bytes", info.size);
	let mut buf = vec![0_u8; info.size as usize];

	let mut file = fs.open(FILE, FileOptions::new().read(true).read_data(true))?;

	println!("reading '{FILE}'");
	let bytes_read = file.read(&mut buf, info.size)?;
	println!("read {bytes_read} bytes");

	let result = String::from_utf8(buf)?;
	println!("content:\n{result}");
	Ok(())
}


fn read_package_info() -> Result<(), fs::error::ApiError> {
	println!("reading pdxinfo");
	let text = fs::read_to_string("pdxinfo", false)?;
	println!("{text}");
	Ok(())
}


#[no_mangle]
/// Proxy event handler, calls `State::event`
pub extern "C" fn eventHandlerShim(api: *const PlaydateAPI, event: PDSystemEvent, _arg: u32) -> c_int {
	match event {
		PDSystemEvent::kEventInit => unsafe {
			// register the API entry point
			sys::API = api;

			// get `setUpdateCallback` fn
			let f = api!(system.setUpdateCallback);

			/// no-op update callback
			unsafe extern "C" fn on_update(_: *mut c_void) -> i32 { 0 }

			// register update callback with user-data = our state
			f(Some(on_update), null_mut());

			list_bundle_dir().expect("list_bundle_dir");
			write_file().expect("write_file");
			read_file().expect("read_file");
			read_package_info().expect("read_package_info");
		},
		_ => {},
	}
	0
}

// Needed for debug build
ll_symbols!();
