#![cfg(target_os = "windows")]

extern crate windows;
use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::pin::Pin;

use windows::core::PCSTR;
use windows::Win32::Storage::FileSystem::GetLogicalDrives;
use windows::Win32::Storage::FileSystem::GetVolumeInformationA;
use windows::Win32::Storage::FileSystem::IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS;
use windows::Win32::System::IO::DeviceIoControl;
use windows::Win32::Foundation::HANDLE;

use crate::error::Error;
use crate::usb::mode::{DeviceMode, Mode};
use crate::device::Device;


#[derive(Debug, Clone)]
pub struct Volume {
	letter: char,
	disk_number: u32,
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.letter.fmt(f) }
}

impl Volume {
	/// This volume's path.
	pub fn path(&self) -> Cow<'_, Path> { PathBuf::from(format!("{}:", self.letter)).into() }
}

mod unmount {
	use futures::FutureExt;
	use futures::TryFutureExt;

	use super::*;
	use crate::mount::Unmount;
	use crate::mount::UnmountAsync;


	impl Unmount for Volume {
		fn unmount_blocking(&self) -> Result<(), Error> {
			unmount_winapi(self.letter).or_else(|err| {
				                           if std::env::var_os("SHELL").is_some() {
					                           eject_sh(self.letter).status()
					                                                .map_err(Error::from)
					                                                .and_then(|res| res.exit_ok().map_err(Error::from))
					                                                .map_err(|err2| Error::chain(err, [err2]))
				                           } else {
					                           Err(err)
				                           }
			                           })
			                           .or_else(|err| {
				                           eject_pw(self.letter).status()
				                                                .map_err(Error::from)
				                                                .and_then(|res| res.exit_ok().map_err(Error::from))
				                                                .map_err(|err2| Error::chain(err, [err2]))
			                           })
		}
	}

	#[cfg(feature = "tokio")]
	impl UnmountAsync for Volume {
		async fn unmount(&self) -> Result<(), Error> {
			use tokio::process::Command;
			use futures_lite::future::ready;

			futures::future::lazy(|_| unmount_winapi(self.letter)).or_else(|err| {
				                                                      if std::env::var_os("SHELL").is_some() {
					                                                      Command::from(eject_sh(self.letter)).status()
																							.map_err(|err2| Error::chain(err, [err2]))
																							.and_then(|res| ready(res.exit_ok().map_err(Error::from)))
												                                 .left_future()
				                                                      } else {
					                                                      ready(Err(err)).right_future()
				                                                      }
			                                                      })
			                                                      .or_else(|err| {
				                                                      Command::from(eject_pw(self.letter)).status()
																						.map_err(|err2| Error::chain(err, [err2]))
																						.and_then(|res| ready(res.exit_ok().map_err(Error::from)))
			                                                      })
			                                                      .await
		}
	}


	fn unmount_winapi(letter: char) -> Result<(), Error> {
		use windows::Win32::System::Ioctl::FSCTL_DISMOUNT_VOLUME;

		let h = file_handle(letter)?;
		// let mut bytes_ret = 0;
		// let mut overlapped = windows::Win32::System::IO::OVERLAPPED::default();
		trace!("Unmounting {letter} {:?} is valid: {}", h.0, !h.0.is_invalid());
		unsafe {
			DeviceIoControl(
			                h.0,
			                FSCTL_DISMOUNT_VOLUME,
			                None,
			                0,
			                None,
			                0,
			                // Some(&mut bytes_ret),
			                None,
			                // Some((&mut overlapped) as *mut _),
			                None,
			)
		}.map(|_| drop(h))
		.map_err(Error::from)
		.or_else(|err| {
			use windows::Win32::Storage::FileSystem::DeleteVolumeMountPointA;

			debug!("{err}, trying fallback method...");
			let (string, s) = pcstr_short(letter);
			unsafe { DeleteVolumeMountPointA(s) }.map(|_| drop(string))
			                                     .map_err(|err2| Error::chain(err, [err2]))
		})
	}


	fn eject_sh(letter: char) -> std::process::Command {
		let (string, _) = pcstr_short(letter);
		let mut cmd = std::process::Command::new("eject");
		cmd.arg(string.to_string_lossy().as_ref());
		cmd
	}

	fn eject_pw(letter: char) -> std::process::Command {
		let arg = format!("(new-object -COM Shell.Application).NameSpace(17).ParseName('{letter}:').InvokeVerb('Eject') | Wait-Process");
		let mut cmd = std::process::Command::new("powershell");
		cmd.arg(arg);
		cmd
	}
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn volume_for(dev: &Device) -> Result<Volume, Error> {
	if !matches!(dev.info().mode(), Mode::Storage) {
		return Err(Error::not_found());
	}

	enumerate_volumes().find(|vol| dev.info().device_address() as u32 == vol.disk_number)
	                   .ok_or_else(|| Error::not_found())
}

#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn volumes_for(devs: &[Device]) -> impl Iterator<Item = (Volume, &Device)> {
	enumerate_volumes().filter_map(|vol| {
		                   devs.into_iter()
		                       .find(|dev| dev.info().device_address() as u32 == vol.disk_number)
		                       .map(|dev| (vol, dev))
	                   })
}


#[cfg_attr(feature = "tracing", tracing::instrument(skip(devs)))]
pub async fn volumes_for_map<I>(devs: I) -> Result<HashMap<Device, Option<Volume>>, Error>
	where I: IntoIterator<Item = Device> {
	let mut devs: Vec<_> = devs.into_iter().collect();
	let mut results = HashMap::with_capacity(devs.len());
	let vols = enumerate_volumes().filter_map(|vol| {
		                              let i = devs.iter()
		                                          .enumerate()
		                                          .find(|(_, dev)| dev.info().device_address() as u32 == vol.disk_number)
		                                          .map(|(i, _)| i);
		                              if let Some(i) = i {
			                              let dev = devs.remove(i);
			                              Some((dev, Some(vol)))
		                              } else {
			                              None
		                              }
	                              });
	results.extend(vols);
	results.extend(devs.into_iter().map(|dev| (dev, None)));
	Ok(results)
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
fn enumerate_volumes() -> impl Iterator<Item = Volume> {
	const LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
	let mask = unsafe { GetLogicalDrives() };
	let letters = (0..(std::mem::size_of_val(&mask) * 8)).into_iter()
	                                                     .filter(move |i| (1 << *i as u32) & mask != 0)
	                                                     .filter_map(|i| LETTERS.chars().nth(i));

	letters.filter_map(move |letter| {
		let (_string, s) = pcstr_short(letter);
		let mut name_buf = vec![0; 1000];

		let mut serial = 0_u32;
		unsafe { GetVolumeInformationA(s, Some(&mut name_buf), Some(&mut serial), None, None, None) }.map_err(|err| debug!("ERROR: {letter}: {err}"))
			                                                                                       .ok()?;

		let drive_name = std::str::from_utf8(&name_buf).map_err(|err| error!("{err}"))
		                                               .ok()?
		                                               .trim();

		debug!("found drive: {letter} {drive_name} ({serial})");
		drive_name.starts_with("PLAYDATE").then_some(letter)
	})
	.filter_map(|letter| {
		file_handle(letter).map(|h| (letter, h))
		                   .map_err(|err| error!("{err}"))
		                   .ok()
	})
	.filter_map(|(letter, handle)| {
		let mut bytes_ret = 0;
		let mut vde = windows::Win32::System::Ioctl::VOLUME_DISK_EXTENTS::default();
		let buf_size = std::mem::size_of_val(&vde);
		let res = unsafe {
			DeviceIoControl(
			                handle.0,
			                IOCTL_VOLUME_GET_VOLUME_DISK_EXTENTS,
			                None,
			                0,
			                Some((&mut vde) as *mut _ as _),
			                buf_size as _,
			                Some(&mut bytes_ret),
			                None,
			)
		};

		// close it anyway:
		drop(handle);

		// map with device address / DiskNumber:
		res.map(|_| {
			   if vde.NumberOfDiskExtents == 1 {
				   vde.Extents.first().map(|vol| {
					                      Volume { letter,
					                               disk_number: vol.DiskNumber }
				                      })
			   } else {
				   None
			   }
		   })
		   .map_err(|err| error!("{err}"))
		   .ok()
		   .flatten()
	})
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
fn file_handle(letter: char) -> Result<FileHandle, std::io::Error> {
	use windows::Win32::Storage::FileSystem::{FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING,
	                                      FILE_FLAGS_AND_ATTRIBUTES};
	use windows::Win32::Storage::FileSystem::CreateFileA;

	let (pinned, s) = pcstr_long(letter);
	let h = unsafe {
		CreateFileA(
		            s,
		            0,
		            FILE_SHARE_READ | FILE_SHARE_WRITE,
		            None,
		            OPEN_EXISTING,
		            FILE_FLAGS_AND_ATTRIBUTES(0),
		            HANDLE::default(),
		)
	}?;
	trace!("opened: {letter} handle, valid: {}", !h.is_invalid());
	Ok(FileHandle(h, pinned))
}


struct FileHandle(pub HANDLE, Pin<CString>);

impl Drop for FileHandle {
	fn drop(&mut self) {
		trace!("closing file handle {:?} by drop", self.0);
		unsafe { windows::Win32::Foundation::CloseHandle(self.0) }.map_err(|err| error!("{err}"))
		                                                          .ok();
	}
}


/// Format and produce `PCSTR` string pointing to the volume `letter` in short format,
/// e.g.: `D:`.
///
/// Returned first element is a pinned `CString` that __must not__ be freed before the returned `PCSTR` is used.
fn pcstr_short(letter: char) -> (Pin<CString>, PCSTR) { pcstr(format!(r"{letter}:")) }

/// Format and produce `PCSTR` string pointing to the volume `letter` in short format,
/// e.g.: `\\?\D:`.
///
/// Returned first element is a pinned `CString` that __must not__ be freed before the returned `PCSTR` is used.
fn pcstr_long(letter: char) -> (Pin<CString>, PCSTR) { pcstr(format!(r"\\.\{letter}:")) }

/// Format and produce `PCSTR` from the given string.
///
/// Returned first element is a pinned `CString` that __must not__ be freed before the returned `PCSTR` is used.
fn pcstr(s: String) -> (Pin<CString>, PCSTR) {
	let s = Pin::new(CString::new(s).unwrap());
	let ps = PCSTR::from_raw(s.as_ptr() as _);
	(s, ps)
}


#[cfg(test)]
mod tests {
	use windows::core::s;


	#[test]
	fn pcstr_short() {
		let (p, ps) = super::pcstr_short('A');
		let expected = s!("A:");
		let expected = unsafe { std::ffi::CStr::from_ptr(expected.0 as _) };
		let ps = unsafe { std::ffi::CStr::from_ptr(ps.0 as _) };
		assert_eq!(expected, ps);
		drop(p);
	}

	#[test]
	fn pcstr_long_() {
		let (p, ps) = super::pcstr_long('A');
		let expected = s!(r"\\.\A:");
		let expected = unsafe { std::ffi::CStr::from_ptr(expected.0 as _) };
		let ps = unsafe { std::ffi::CStr::from_ptr(ps.0 as _) };
		assert_eq!(expected, ps);
		drop(p);
	}
}
