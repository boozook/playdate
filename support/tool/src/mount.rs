use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use ::build::toolchain::sdk::Sdk;

use crate::Error;
use crate::model::Device;
use crate::model::Mode;


#[cfg(target_os = "macos")]
pub const DEVICE_MOUNT_POINT_DEF: &str = "/Volumes/PLAYDATE";
#[cfg(all(unix, not(target_os = "macos")))]
pub const DEVICE_MOUNT_POINT_DEF: &str = "/run/media/${USER}/PLAYDATE";
#[cfg(not(unix))]
// FIXME: set the real expected path
pub const DEVICE_MOUNT_POINT_DEF: &str = "/TODO/PLAYDATE";


impl Device {
	pub fn mounted(&self) -> bool {
		self.mode == Mode::Storage &&
		self.volume
		    .as_deref()
		    .filter(|p| p.try_exists().ok().unwrap_or_default())
		    .is_some()
	}
}


pub fn mount_point(mount: &Path) -> Result<PathBuf, Error> {
	use ::build::assets::resolver::EnvResolver;

	let path = mount.to_str()
	                .ok_or_else(|| Error::Error(format!("Mount point is not invalid utf-8: {}", mount.display())))
	                .map(|s| EnvResolver::new().str_only(s))?;

	let path: &Path = Path::new(path.as_ref());

	if !path.try_exists()? {
		Ok(path.to_path_buf())
	} else {
		use std::io::{Error as IoError, ErrorKind};
		Err(IoError::new(
			ErrorKind::AlreadyExists,
			format!("Mount point is already exists: {}", mount.display()),
		).into())
	}
}


/// Mount `device` and return `MountHandle` with the given `mount`.
/// `mount` if __expected__ mount-point.
pub fn mount_device(device: &Device, mount: Option<PathBuf>) -> Result<MountHandle, Error> {
	let mount = if let Some(mount) = mount {
		mount
	} else {
		mount_point(Path::new(DEVICE_MOUNT_POINT_DEF))?
	};

	// mount the device:
	let mount_handle = if let Some(sdk) = Sdk::try_new().map_err(|err| error!("{err}")).ok() {
		mount_with(&sdk, &mount, &device)
	} else {
		mount_without_sdk(&mount, &device)
	}?;

	info!(
	      "{device} successfully mounted to {}",
	      mount_handle.path().display()
	);
	Ok(mount_handle)
}


/// Send command to `device` and return `MountHandle` with the given `mount`.
/// A `mount` is __expected__ mount-point.
pub fn mount_with(sdk: &Sdk, mount: &Path, device: &Device) -> Result<MountHandle, Error> {
	let mount = mount_point(mount)?;
	trace!("serial: {device:?}, mount to: {}", mount.display());

	ensure_device_mountable(device)?;
	let tty = device.tty.as_deref().ok_or(Error::unable_to_find_tty(device))?;
	let mut cmd = Command::new(sdk.pdutil());
	cmd.arg(tty).arg("datadisk");
	debug!("Run: {:?}", cmd);
	cmd.status()?.exit_ok()?;

	Ok(MountHandle::new(mount))
}

pub fn mount_by_device_tty_with(sdk: &Sdk, mount: &Path, device_tty: &Path) -> Result<MountHandle, Error> {
	let mount = mount_point(mount)?;
	trace!("tty: {}, mount to: {}", device_tty.display(), mount.display());

	let mut cmd = Command::new(sdk.pdutil());
	cmd.arg(device_tty).arg("datadisk");
	debug!("Run: {:?}", cmd);
	cmd.status()?.exit_ok()?;

	Ok(MountHandle::new(mount))
}


pub fn mount_without_sdk(mount: &Path, device: &Device) -> Result<MountHandle, Error> {
	ensure_device_mountable(device)?;
	send_storage_mode_to_device_without_sdk(mount, device).map_err(|err| {
		                                                      Error::Error(format!("Unable to send command to {device:?}: {err}."))
	                                                      })?;
	Ok(MountHandle::new(mount.to_path_buf()))
}

pub fn send_storage_mode_to_device_without_sdk(mount: &Path, device: &Device) -> Result<MountHandle, Error> {
	device.write("datadisk").map_err(|err| {
		                         error!("{err}");
		                         Error::Error(format!("Unable to send command to {device}."))
	                         })?;
	Ok(MountHandle::new(mount.to_path_buf()))
}


pub fn ensure_device_mountable(device: &Device) -> Result<(), Error> {
	if !device.mounted() {
		Ok(())
	} else {
		let volume = device.volume
		                   .as_deref()
		                   .map(|p| format!(" at {}", p.display()))
		                   .unwrap_or_default();
		Err(Error::Error(format!("{device} is already mounted{}", volume)))
	}
}


pub struct MountHandle {
	path: PathBuf,
	unmount_on_drop: bool,
}

impl MountHandle {
	pub fn new(path: PathBuf) -> Self {
		MountHandle { path,
		              unmount_on_drop: true }
	}

	pub fn unmount_on_drop(&mut self, value: bool) { self.unmount_on_drop = value; }
	pub fn path(&self) -> &Path { &self.path }

	pub(crate) fn set_mount_point(&mut self, path: PathBuf) { self.path = path; }
}

impl Drop for MountHandle {
	fn drop(&mut self) {
		if self.unmount_on_drop {
			debug!("Unmounting {}", self.path.display());
			let _ = unmount(&self.path).map_err(|err| {
				                           error!("{err}");
				                           info!("Please press 'A' on the Playdate to exit Data Disk mode.");
			                           })
			                           .ok();
		}
	}
}


#[cfg(target_os = "macos")]
pub fn unmount(path: &Path) -> Result<(), Error> {
	Command::new("diskutil").arg("eject")
	                        .arg(path)
	                        .status()?
	                        .exit_ok()?;
	Ok(())
}

#[cfg(target_os = "linux")]
pub fn unmount(path: &Path) -> Result<(), Error> {
	Command::new("eject").arg(path).status()?.exit_ok()?;
	Ok(())
}

#[cfg(target_os = "windows")]
pub fn unmount(path: &Path) -> Result<(), Error> {
	warn!("Unmounting not implemented for windows yet.");
	Command::new("eject").arg(path).status()?.exit_ok()?;
	Ok(())
}
