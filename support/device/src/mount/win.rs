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

use crate::device::serial::SerialNumber;
use crate::error::Error;
use crate::usb::mode::{DeviceMode, Mode};
use crate::device::Device;


#[derive(Debug, Clone)]
pub struct Volume {
	letter: char,
	disk_number: Option<u32>,
	serial_number: Option<SerialNumber>,
}

impl Volume {
	pub fn new(letter: char) -> Self {
		Self { letter,
		       disk_number: None,
		       serial_number: None }
	}
}

impl std::fmt::Display for Volume {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.letter.fmt(f) }
}

impl Volume {
	/// This volume's path.
	pub fn path(&self) -> Cow<'_, Path> { PathBuf::from(format!("{}:", self.letter)).into() }
}

pub mod unmount {
	use futures::FutureExt;
	use futures::TryFutureExt;

	use super::*;
	use crate::mount::Unmount;
	use crate::mount::UnmountAsync;


	impl Unmount for Volume {
		#[cfg_attr(feature = "tracing", tracing::instrument())]
		fn unmount_blocking(&self) -> Result<(), Error> {
			#[cfg(feature = "eject")]
			let res = unmount_eject(&self).or_else(|err| {
				                     winapi::unmount(self.letter).map_err(|err2| Error::chain(err2, [err]))
			                     });
			#[cfg(not(feature = "eject"))]
			let res = winapi::unmount(self.letter);

			res.or_else(|err| {
				   if std::env::var_os("SHELL").is_some() {
					   eject_sh(self.letter).status()
					                        .map_err(Error::from)
					                        .and_then(|res| res.exit_ok().map_err(Error::from))
					                        .map_err(|err2| Error::chain(err2, [err]))
				   } else {
					   Err(err)
				   }
			   })
			   .or_else(|err| {
				   eject_pw(self.letter).status()
				                        .map_err(Error::from)
				                        .and_then(|res| res.exit_ok().map_err(Error::from))
				                        .map_err(|err2| Error::chain(err2, [err]))
			   })
		}
	}

	impl UnmountAsync for Volume {
		#[cfg_attr(feature = "tracing", tracing::instrument())]
		async fn unmount(&self) -> Result<(), Error> {
			use futures_lite::future::ready;
			use futures::future::lazy;
			#[cfg(all(feature = "tokio", not(feature = "async-std")))]
			use tokio::process::Command;
			#[cfg(feature = "async-std")]
			use async_std::process::Command;

			#[cfg(feature = "eject")]
			let fut = lazy(|_| unmount_eject(&self)).or_else(|err| {
				                               lazy(|_| {
					                               winapi::unmount(self.letter).map_err(|err2| {
						                                                           Error::chain(err2, [err])
					                                                           })
				                               })
			                               });
			#[cfg(not(feature = "eject"))]
			let fut = lazy(|_| winapi::unmount(self.letter));

			fut.or_else(|err| {
				   if std::env::var_os("SHELL").is_some() {
					   Command::from(eject_sh(self.letter)).status()
					                                       .map_err(|err2| Error::chain(err2, [err]))
					                                       .and_then(|res| ready(res.exit_ok().map_err(Error::from)))
					                                       .left_future()
				   } else {
					   ready(Err(err)).right_future()
				   }
			   })
			   .or_else(|err| {
				   Command::from(eject_pw(self.letter)).status()
				                                       .map_err(|err2| Error::chain(err2, [err]))
				                                       .and_then(|res| ready(res.exit_ok().map_err(Error::from)))
			   })
			   .await
		}
	}


	#[cfg(feature = "eject")]
	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn unmount_eject(vol: &Volume) -> Result<(), Error> {
		use eject::device::Device;

		let path = to_vol_path_short(vol.letter);
		let drive = Device::open(&path).map_err(std::io::Error::from)?;
		drive.eject().map_err(std::io::Error::from)?;
		trace!("Ejected {}", vol.letter);
		Ok(())
	}


	fn eject_sh(letter: char) -> std::process::Command {
		let string = to_vol_path_short(letter);
		let mut cmd = std::process::Command::new("eject");
		cmd.arg(string);
		cmd
	}

	fn eject_pw(letter: char) -> std::process::Command {
		// let arg = format!("(New-Object -comObject Shell.Application).NameSpace(17).ParseName('{letter}:').InvokeVerb('Eject') | Wait-Process");
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

	let sn = dev.info().serial_number().ok_or_else(|| {
		Error::DeviceSerial { source: crate::device::serial::error::DeviceSerialFormatError::from("missed sn") }
	})?;
	let sn = SerialNumber::try_from(sn)?;
	let dev_addr = dev.info().device_address() as u32;
	enumerate_volumes().find(move |vol| is_that_vol(&sn, dev_addr, &vol))
	                   .ok_or_else(|| Error::not_found())
}

#[cfg_attr(feature = "tracing", tracing::instrument())]
fn is_that_vol(dev_sn: &SerialNumber, device_address: u32, vol: &Volume) -> bool {
	if let Some(vol_sn) = vol.serial_number.as_ref() {
		vol_sn == dev_sn
	} else {
		vol.disk_number == Some(device_address)
	}
}

#[cfg_attr(feature = "tracing", tracing::instrument())]
pub fn volumes_for(devs: &[Device]) -> impl Iterator<Item = (Volume, &Device)> {
	enumerate_volumes().filter_map(|vol| {
		                   devs.into_iter()
		                       .find(|dev| {
			                       if let Some(sn) = dev.info().serial_number() {
				                       if let Ok(sn) = SerialNumber::try_from(sn).inspect_err(|err| error!("{err}")) {
					                       let dev_addr = dev.info().device_address() as u32;
					                       return is_that_vol(&sn, dev_addr, &vol);
				                       }
			                       }
			                       false
		                       })
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
		                                          .find(|(_, dev)| {
			                                          if let Some(sn) = dev.info().serial_number() {
				                                          if let Ok(sn) =
						                                  SerialNumber::try_from(sn).inspect_err(|err| error!("{err}"))
					                                  {
						                                  let dev_addr = dev.info().device_address() as u32;
						                                  return is_that_vol(&sn, dev_addr, &vol);
					                                  }
			                                          }
			                                          false
		                                          })
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


/// Enumerate all available mounted volumes,
/// filtered by:
/// - that is Playdate
/// - that are has SN __or__ DN.
#[cfg_attr(feature = "tracing", tracing::instrument())]
fn enumerate_volumes() -> impl Iterator<Item = Volume> {
	let letters = winapi::enumerate_volumes();

	letters.filter_map(|letter| {
		       // TODO: remove this check:
		       winapi::get_volume_name(letter).inspect_err(|err| debug!("{letter}: {err}"))
		                                      .ok()?
		                                      .starts_with("PLAYDATE")
		                                      .then_some(letter)
	       })
	       .map(|letter| {
		       let sn = ps::vol_sn(letter).inspect_err(|err| debug!("{letter}: {err}"))
		                                  .ok();
		       // Get DN if no SN:
		       let dn = sn.is_none()
		                  .then(|| {
			                  ps::vol_dn(letter).inspect_err(|err| debug!("{letter}: {err}"))
			                                    .ok()
		                  })
		                  .flatten();

		       trace!("Seems to {letter} is a Playdate: {dn:?}, {sn:?}");
		       Volume { letter,
		                disk_number: dn,
		                serial_number: sn }
	       })
	       .map(|mut vol| {
		       if vol.disk_number.is_none() {
			       let dn = winapi::get_disk_number(vol.letter).inspect_err(|err| debug!("{}: {err}", vol.letter))
			                                                   .ok();
			       if let Some(dn) = dn {
				       vol.disk_number = Some(dn);
			       }
		       }
		       vol
	       })
	       .filter(|vol| vol.disk_number.is_some() || vol.serial_number.is_some())
}


mod winapi {
	use super::*;

	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn enumerate_volumes() -> impl Iterator<Item = char> {
		const LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
		let mask = unsafe { GetLogicalDrives() };
		let letters = (0..(std::mem::size_of_val(&mask) * 8)).into_iter()
		                                                     .filter(move |i| (1 << *i as u32) & mask != 0)
		                                                     .filter_map(|i| LETTERS.chars().nth(i));
		letters
	}

	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn get_volume_name(letter: char) -> Result<String, Error> {
		let (_string, s) = pcstr_short(letter);
		let mut name_buf = vec![0; 1000];

		let mut serial = 0_u32;
		unsafe {
			GetVolumeInformationA(s, Some(&mut name_buf), Some(&mut serial), None, None, None)
		}
		.map_err(std::io::Error::from)?;

		let drive_name = std::str::from_utf8(&name_buf)?.trim();

		let temp_drive_name = (!drive_name.is_empty()).then_some(drive_name)
		                                              .unwrap_or("unnamed");
		trace!("found drive: {letter} '{temp_drive_name}' ({serial})");

		Ok(drive_name.to_string())
	}

	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn get_disk_number(letter: char) -> Result<u32, Error> {
		let handle = file_handle(letter)?;


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

		res?;

		if vde.NumberOfDiskExtents == 1 {
			debug!("found device volume {letter}.");
			vde.Extents
			   .first()
			   .map(|vol| vol.DiskNumber)
			   .ok_or(Error::not_found())
		} else {
			Err(Error::not_found())
		}
	}


	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn unmount(letter: char) -> Result<(), Error> {
		use windows::Win32::System::Ioctl::FSCTL_DISMOUNT_VOLUME;

		let h = winapi::file_handle(letter)?;
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
			let (string, s) = winapi::pcstr_short(letter);
			unsafe { DeleteVolumeMountPointA(s) }.map(|_| drop(string))
			                                     .map_err(|err2| Error::chain(err2, [err]))
		})
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
	#[inline]
	pub fn pcstr_short(letter: char) -> (Pin<CString>, PCSTR) { pcstr(to_vol_path_short(letter)) }

	/// Format and produce `PCSTR` string pointing to the volume `letter` in short format,
	/// e.g.: `\\?\D:`.
	///
	/// Returned first element is a pinned `CString` that __must not__ be freed before the returned `PCSTR` is used.
	#[inline]
	pub fn pcstr_long(letter: char) -> (Pin<CString>, PCSTR) { pcstr(to_vol_path_long(letter)) }

	/// Format and produce `PCSTR` from the given string.
	///
	/// Returned first element is a pinned `CString` that __must not__ be freed before the returned `PCSTR` is used.
	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn pcstr(s: String) -> (Pin<CString>, PCSTR) {
		let s = Pin::new(CString::new(s).unwrap());
		let ps = PCSTR::from_raw(s.as_ptr() as _);
		(s, ps)
	}


	#[cfg_attr(feature = "tracing", tracing::instrument)]
	pub fn get_vol_disk_number_winapi(letter: char) -> Result<u32, std::io::Error> {
		let handle = file_handle(letter)?;

		let mut bytes_ret = 0;
		let mut vde = windows::Win32::System::Ioctl::VOLUME_DISK_EXTENTS::default();
		let buf_size = std::mem::size_of_val(&vde);
		unsafe {
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
		}?;

		// map with device address / DiskNumber:
		let result = if vde.NumberOfDiskExtents == 1 {
			debug!("found device volume {letter}.");
			vde.Extents.first().map(|vol| vol.DiskNumber)
		} else {
			None
		};

		// close it anyway:
		drop(handle);

		result.ok_or_else(|| {
			      use std::io::{Error, ErrorKind};
			      let msg = format!("Cannot get volume {letter} disk number.");
			      Error::new(ErrorKind::NotFound, msg)
		      })
	}


	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn file_handle(letter: char) -> Result<FileHandle, std::io::Error> {
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
}


fn to_vol_path_short(letter: char) -> String { format!(r"{letter}:") }
fn to_vol_path_long(letter: char) -> String { format!(r"\\.\{letter}:") }


mod ps {
	//! Powershell-related functions.
	use crate::device::serial::SerialNumber;
	// TODO: should be ExecutionError:
	use crate::error::Error;


	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn vol_dn(letter: char) -> Result<u32, Error> {
		let mut cmd = vol_dn_cmd_1(letter);
		let stdout = cmd.output().and_then(validate_output).or_else(|err| {
			                                                    trace!("method #1: {err}");
			                                                    vol_dn_cmd_2(letter).output()
			                                                                        .and_then(validate_output)
		                                                    })?;
		std::str::from_utf8(&stdout)?.trim().parse().map_err(|err| {
			                                            use std::io::{Error, ErrorKind as Kind};
			                                            Error::new(Kind::InvalidData, err).into()
		                                            })
	}

	#[cfg_attr(feature = "tracing", tracing::instrument())]
	pub fn vol_sn(letter: char) -> Result<SerialNumber, Error> {
		let mut cmd = vol_sn_cmd_1(letter);
		let stdout = cmd.output().and_then(validate_output).or_else(|err| {
			                                                    trace!("method #1: {err}");
			                                                    vol_sn_cmd_2(letter).output()
			                                                                        .and_then(validate_output)
		                                                    })?;
		std::str::from_utf8(&stdout)?.trim().parse().map_err(Into::into)
	}


	/// Return stdout of the process output if not empty and no error occurred.
	#[cfg_attr(feature = "tracing", tracing::instrument())]
	fn validate_output(output: std::process::Output) -> Result<Vec<u8>, std::io::Error> {
		use std::io::{Error, ErrorKind as Kind};

		match output.status.exit_ok() {
			Ok(_) => {
				let stdout = std::str::from_utf8(&output.stdout).inspect_err(|err| error!("{err}"))
				                                                .ok()
				                                                .map(|s| s.trim())
				                                                .filter(|s| !s.is_empty())
				                                                .is_some();
				if stdout {
					Ok(output.stdout)
				} else {
					Err(Error::new(Kind::InvalidData, "Empty output."))
				}
			},
			Err(err) => {
				let stderr = String::from_utf8_lossy(&output.stderr);
				let stderr = stderr.trim();
				if !stderr.is_empty() {
					Err(Error::new(Kind::Other, stderr))
				} else {
					Err(Error::new(Kind::Other, err))
				}
			},
		}
	}


	/// Retrieve __disk-number__ for the volume `letter`.
	/// Perhaps not work on Win7 and earlier.
	/// Returns exit code.
	/// Returns number in the stdout, should be trimmed.
	fn vol_dn_cmd_1(letter: char) -> std::process::Command {
		let arg =
			format!("Get-Partition -DriveLetter {letter} | Get-Disk | select-object -ExpandProperty DiskNumber");
		let mut cmd = std::process::Command::new("powershell");
		cmd.arg(arg);
		cmd
	}

	/// Retrieve __serial-number__ for the volume `letter`.
	/// Perhaps not work on Win7 and earlier.
	/// Returns exit code.
	/// Returns sn-string in the stdout, should be trimmed.
	fn vol_sn_cmd_1(letter: char) -> std::process::Command {
		let arg =
			format!("Get-Partition -DriveLetter {letter} | Get-Disk | select-object -ExpandProperty SerialNumber");
		let mut cmd = std::process::Command::new("powershell");
		cmd.arg(arg);
		cmd
	}

	/// Retrieve __disk-number__ for the volume `letter`.
	/// Returns exit code always = `0`.
	/// Returns sn-string in the stdout, should be trimmed.
	/// If no result is found, returns empty stdout (after trim).
	fn vol_dn_cmd_2(letter: char) -> std::process::Command {
		let arg = format!("Get-CimInstance -ClassName Win32_DiskDrive | Get-CimAssociatedInstance -Association Win32_DiskDriveToDiskPartition | Get-CimAssociatedInstance -Association Win32_LogicalDiskToPartition | Where-Object DeviceId -eq '{letter}:' | Get-CimAssociatedInstance -Association Win32_LogicalDiskToPartition | Select-Object -ExpandProperty DiskIndex");
		let mut cmd = std::process::Command::new("powershell");
		cmd.arg(arg);
		cmd
	}

	/// Retrieve __serial-number__ for the volume `letter`.
	/// Returns exit code always = `0`.
	/// Returns sn-string in the stdout, should be trimmed.
	/// If no result is found, returns empty stdout (after trim).
	fn vol_sn_cmd_2(letter: char) -> std::process::Command {
		let arg = format!("Get-CimInstance -ClassName Win32_DiskDrive | Get-CimAssociatedInstance -Association Win32_DiskDriveToDiskPartition | Get-CimAssociatedInstance -Association Win32_LogicalDiskToPartition | Where-Object DeviceId -eq '{letter}:' | Get-CimAssociatedInstance -Association Win32_LogicalDiskToPartition | Get-CimAssociatedInstance -Association Win32_DiskDriveToDiskPartition | Select-Object -ExpandProperty SerialNumber");
		let mut cmd = std::process::Command::new("powershell");
		cmd.arg(arg);
		cmd
	}
}
