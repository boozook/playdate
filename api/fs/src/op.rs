//! Oxidized api-functions which returns borrowed error with caller's needed lifetime.
use core::ffi::*;

use sys::ffi::FileStat;

use crate::error::Borrowed;
use crate::file::File;
use crate::options::OpenOptions;
use crate::seek::Whence;
use crate::Api;
use crate::Path;


pub fn open<'t, P: AsRef<Path>, Opts: OpenOptions>(api: Api,
                                                   path: P,
                                                   options: Opts)
                                                   -> Result<File, Borrowed<'t>> {
	let ptr = unsafe { (api.open)(path.as_ref().as_ptr(), options.into()) };
	if ptr.is_null() {
		Err(Borrowed::latest(api))
	} else {
		Ok(File(ptr.cast()))
	}
}

pub fn close<'t>(api: Api, mut file: File) -> Result<(), Borrowed<'t>> {
	let result = unsafe { (api.close)(file.0.cast()) };
	file.0 = core::ptr::null_mut();
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn seek<'t>(api: Api, file: &mut File, pos: c_int, whence: Whence) -> Result<(), Borrowed<'t>> {
	let result = unsafe { (api.seek)(file.0, pos, whence as _) };
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn tell<'t>(api: Api, file: &mut File) -> Result<c_uint, Borrowed<'t>> {
	let result = unsafe { (api.tell)(file.0) };
	Borrowed::from_code(result, api)
}

pub fn read<'t>(api: Api, file: &mut File, to: &mut [u8], len: c_uint) -> Result<c_uint, Borrowed<'t>> {
	debug_assert!(len as usize >= to.len());
	let result = unsafe { (api.read)(file.0, to.as_mut_ptr().cast(), len) };
	Borrowed::from_code(result, api)
}

pub fn write<'t>(api: Api, file: &mut File, from: &[u8]) -> Result<c_uint, Borrowed<'t>> {
	let result = unsafe { (api.write)(file.0, from.as_ptr().cast(), from.len() as _) };
	Borrowed::from_code(result, api)
}

pub fn flush<'t>(api: Api, file: &mut File) -> Result<c_uint, Borrowed<'t>> {
	let result = unsafe { (api.flush)(file.0) };
	Borrowed::from_code(result, api)
}

pub fn metadata<'t, P: AsRef<Path>>(api: Api, path: P, metadata: &mut FileStat) -> Result<(), Borrowed<'t>> {
	let path = path.as_ref();
	let result = unsafe { (api.stat)(path.as_ptr(), metadata) };
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn create_dir<'t, P: AsRef<Path>>(api: Api, path: P) -> Result<(), Borrowed<'t>> {
	let result = unsafe { (api.mkdir)(path.as_ref().as_ptr()) };
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn remove<'t, P: AsRef<Path>>(api: Api, path: P) -> Result<(), Borrowed<'t>> {
	let result = unsafe { (api.unlink)(path.as_ref().as_ptr(), 0) };
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn remove_dir_all<'t, P: AsRef<Path>>(api: Api, path: P) -> Result<(), Borrowed<'t>> {
	let result = unsafe { (api.unlink)(path.as_ref().as_ptr(), 1) };
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn rename<'t, P: AsRef<Path>, Q: AsRef<Path>>(api: Api, from: P, to: Q) -> Result<(), Borrowed<'t>> {
	let result = unsafe { (api.rename)(from.as_ref().as_ptr(), to.as_ref().as_ptr()) };
	Borrowed::from_code(result, api).map(|_| ())
}

pub fn read_dir<'t, P, Fn>(api: Api,
                           path: P,
                           mut callback: Fn,
                           include_hidden: bool)
                           -> Result<(), Borrowed<'t>>
	where P: AsRef<Path>,
	      Fn: FnMut(&Path)
{
	unsafe extern "C" fn read_dir<Fn: FnMut(&Path)>(filename: *const c_char, userdata: *mut c_void) {
		if let Some(callback) = (userdata as *mut _ as *mut Fn).as_mut() {
			callback(Path::from_ptr(filename as _));
		} else {
			panic!("callback missed");
		}
	}

	// that's safe because ref dies after internal listfiles() returns.
	let callback_ref = (&mut callback) as *mut Fn as *mut _;

	let result = unsafe {
		(api.listfiles)(
		                path.as_ref().as_ptr(),
		                Some(read_dir::<Fn>),
		                callback_ref,
		                include_hidden as _,
		)
	};
	Borrowed::from_code(result, api).map(|_| ())
}
