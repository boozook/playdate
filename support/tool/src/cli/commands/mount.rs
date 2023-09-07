use std::ffi::OsString;
use std::path::PathBuf;

use playdate::toolchain::sdk::Sdk;

use super::DeviceQuery;
use crate::Error;
use crate::cli::find_one_device;
use crate::model::Device;
use crate::mount::mount_with;
use crate::mount::mount_without_sdk;


/// Mount the device.
#[derive(Clone, Debug, clap::Parser)]
#[command(author, version, about, long_about = None, name = "install")]
pub struct Mount {
	#[cfg_attr(feature = "cli", command(flatten))]
	pub device: DeviceQuery,

	/// Expected mount point for serial device.
	#[cfg_attr(feature = "cli", arg(long, env = crate::DEVICE_MOUNT_POINT_ENV))]
	#[cfg_attr(feature = "cli", arg(default_value = crate::DEVICE_MOUNT_POINT_DEF))]
	pub mount: PathBuf,
}

impl Default for Mount {
	fn default() -> Self {
		Self { device: Default::default(),
		       mount:
			       std::env::var_os(crate::DEVICE_MOUNT_POINT_ENV).unwrap_or(OsString::from(crate::DEVICE_MOUNT_POINT_DEF))
			                                                      .into() }
	}
}


pub fn mount(cfg: &Mount) -> Result<crate::mount::MountHandle, Error> {
	let device = find_one_device(cfg.device.clone())?;
	trace!("mounting device: {device:?}");
	let handle = mount_device(&cfg, &device)?;

	info!(
	      "Successfully mounted, expected mount-point: {}",
	      handle.path().display()
	);
	Ok(handle)
}


fn mount_device(cfg: &Mount, device: &Device) -> Result<crate::mount::MountHandle, Error> {
	let mount_handle = if let Some(sdk) = Sdk::try_new().map_err(|err| error!("{err}")).ok() {
		mount_with(&sdk, &cfg.mount, device)
	} else {
		mount_without_sdk(&cfg.mount, device)
	}?;
	Ok(mount_handle)
}
