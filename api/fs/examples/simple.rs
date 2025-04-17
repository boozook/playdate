#![no_std]
#![no_main]
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate sys;
extern crate playdate_fs as fs;

use alloc::string::String;
use core::ptr::null_mut;

use sys::ffi::*;
use sys::ctrl::EventLoopCtrl;
use fs::error::ReadUtf8Error;
use fs::prelude::*;


fn list_bundle_dir() -> Result<(), FsError> {
	let fs = Fs::default();
	let include_hidden = true;
	println!("Listing root dir...");
	let mut num_files = 0;
	fs.read_dir(
	            c"/",
	            |path| {
		            num_files += 1;
		            println!("  {path:?}");
	            },
	            include_hidden,
	)?;
	println!("{num_files} files root dir.");
	Ok(())
}

const DIR: &Path = c"temp";
const FILE: &Path = c"temp/temp-file";


fn write_file() -> Result<(), FsError> {
	let fs = Fs::default();

	let exists = fs.metadata(FILE).is_ok();

	if exists {
		println!("file already exists, overwriting");
	} else {
		println!("file doesn't exists, creating new");

		// Create dir:
		fs.create_dir(DIR)?;
	}

	let text = "Hello, World!";
	println!("writing {text:?} to {FILE:?}");

	let mut file = fs.open(FILE, FileOptions::new().write(true))?;
	fs.write(&mut file, text.as_bytes())
	  .inspect(|bytes_written| println!("written {bytes_written} bytes"))
	  .map(|_| ())?;
	Ok(())
}


fn read_file() -> Result<(), ReadUtf8Error> {
	let fs = Fs::default();

	println!("reading file metadata");
	let info = fs.metadata(FILE)?;
	println!("info: {info:#?}");

	// Prepare prefilled buffer:
	println!("preparing buffer for {} bytes", info.size);
	let mut buf = vec![0_u8; info.size as usize];

	let mut file = fs.open(FILE, FileOptions::new().read(true).read_data(true))?;

	println!("reading {FILE:?}");
	let bytes_read = file.read(&mut buf, info.size)?;
	println!("read {bytes_read} bytes");

	let result = String::from_utf8(buf)?;
	println!("content: {result:?}");

	Ok(())
}


fn read_package_info() -> Result<(), ReadUtf8Error> {
	println!("reading pdxinfo:");
	let text = fs::read_to_string(c"pdxinfo", false)?;
	println!("{text}");
	Ok(())
}


/// Entry point / event handler
#[no_mangle]
fn event_handler(api: &'static Playdate, event: SystemEvent, _: u32) -> EventLoopCtrl {
	// Ignore any other events, just for this minimalistic example
	if matches!(event, SystemEvent::Init) {
		list_bundle_dir().expect("list_bundle_dir");
		write_file().expect("write_file");
		read_file().expect("read_file");
		read_package_info().expect("read_package_info");

		// Set no-op update callback
		unsafe { (api.system.setUpdateCallback)(None, null_mut()) };
	}

	EventLoopCtrl::Stop
}


#[cfg(miri)]
#[no_mangle]
fn miri_start(_argc: isize, _argv: *const *const u8) -> isize { sys::mock::executor::minimal() }


// Needed for device target when building with arm-gcc and linking with its stdlib.
// ll_symbols!();
