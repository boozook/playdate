use std::borrow::Cow;
use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;

use crate::device::Device;
use crate::error::Error;


#[cfg(target_os = "macos")]
#[path = "mac.rs"]
pub mod volume;
#[cfg(target_os = "windows")]
#[path = "win.rs"]
pub mod volume;
#[cfg(target_os = "linux")]
#[path = "linux.rs"]
pub mod volume;

mod methods;
pub use methods::*;


// TODO: If unmount fails, do warn!("Please press 'A' on the Playdate to exit Data Disk mode.")


// TODO: MountError for this module


pub trait Volume {
	/// This volume's path.
	fn path(&self) -> Cow<'_, Path>;
}

pub trait Unmount {
	/// Unmount this volume. Blocking.
	fn unmount_blocking(&self) -> Result<(), Error>;
}

pub trait UnmountAsync {
	/// Unmount this volume.
	fn unmount(&self) -> impl std::future::Future<Output = Result<(), Error>>;
}

pub trait Mount {
	/// Mount this volume. Blocking.
	fn mount_blocking(&self) -> Result<(), Error>;
}

pub trait MountAsync {
	fn mount(&self) -> impl std::future::Future<Output = Result<(), Error>>;
}


impl Mount for Device {
	fn mount_blocking(&self) -> Result<(), Error> {
		use crate::interface::blocking::Out;
		use crate::device::command::Command;

		self.interface()?.send_cmd(Command::Datadisk)?;
		Ok(())
	}
}

impl MountAsync for Device {
	async fn mount(&self) -> Result<(), Error> { self.interface()?.mount().await }
}


impl<T> MountAsync for T where T: crate::interface::r#async::Out {
	async fn mount(&self) -> Result<(), Error> {
		self.send_cmd(crate::device::command::Command::Datadisk).await?;
		Ok(())
	}
}

impl<T> Mount for T where T: crate::interface::blocking::Out {
	fn mount_blocking(&self) -> Result<(), Error> {
		self.send_cmd(crate::device::command::Command::Datadisk)?;
		Ok(())
	}
}


impl<T> UnmountAsync for T where T: crate::interface::r#async::Out {
	async fn unmount(&self) -> Result<(), Error> {
		self.send_cmd(crate::device::command::Command::Datadisk).await?;
		Ok(())
	}
}

impl<T> Unmount for T where T: crate::interface::blocking::Out {
	fn unmount_blocking(&self) -> Result<(), Error> {
		self.send_cmd(crate::device::command::Command::Datadisk)?;
		Ok(())
	}
}


pub struct MountedDevice {
	pub device: Device,
	pub handle: MountHandle,
}

impl Unmount for MountedDevice {
	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(dev = self.info().serial_number(), mount = self.handle.volume().path().as_ref().display().to_string())))]
	fn unmount_blocking(&self) -> Result<(), Error> {
		<volume::Volume as Unmount>::unmount_blocking(&self.handle.volume)
	}
}

impl UnmountAsync for MountedDevice where volume::Volume: UnmountAsync {
	#[cfg_attr(feature = "tracing", tracing::instrument(skip(self), fields(dev = self.info().serial_number(), mount = self.handle.volume().path().as_ref().display().to_string())))]
	fn unmount(&self) -> impl std::future::Future<Output = Result<(), Error>> {
		<volume::Volume as UnmountAsync>::unmount(&self.handle.volume)
	}
}

impl MountedDevice {
	pub fn new(device: Device, handle: MountHandle) -> Self { Self { device, handle } }
}

impl Deref for MountedDevice {
	type Target = Device;
	fn deref(&self) -> &Self::Target { &self.device }
}

impl DerefMut for MountedDevice {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.device }
}


pub struct MountHandle {
	volume: volume::Volume,
	pub unmount_on_drop: bool,
}

impl MountHandle {
	pub fn new(volume: volume::Volume, unmount_on_drop: bool) -> Self {
		Self { volume,
		       unmount_on_drop }
	}

	pub fn path(&self) -> Cow<'_, Path> { self.volume.path() }
	pub fn volume(&self) -> &volume::Volume { &self.volume }

	pub fn unmount(mut self) {
		self.unmount_on_drop = true;
		drop(self)
	}
}

impl Drop for MountHandle {
	fn drop(&mut self) {
		if self.unmount_on_drop {
			trace!("Unmounting {} by drop", self.volume);
			let _ = self.volume
			            .unmount_blocking()
			            .map_err(|err| {
				            error!("{err}");
				            info!("Please press 'A' on the Playdate to exit Data Disk mode.");
			            })
			            .ok();
		}
	}
}
