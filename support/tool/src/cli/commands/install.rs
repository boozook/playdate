use std::path::PathBuf;
use std::time::Duration;
use std::process::Command;

use ::build::toolchain::sdk::Sdk;
use ::build::compile::PDX_PKG_EXT;

use crate::Error;
use crate::OnDevicePath;
use crate::wait_for;
use crate::wait_for_mut;
use super::mount::Mount;
use crate::mount::MountHandle;
use crate::cli::find_one_device;


/// Installs a given pdx-package to the device.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(feature = "cli", command(author, version, about, long_about = None, name = "install"))]
pub struct Install {
	/// Path to the PDX package.
	#[cfg_attr(feature = "cli", arg(value_name = "PACKAGE"))]
	pub pdx: PathBuf,

	#[cfg_attr(feature = "cli", command(flatten))]
	pub mount: Mount,
}


pub fn install(cfg: &Install) -> Result<OnDevicePath, Error> {
	if let Some(sdk) = Sdk::try_new().map_err(|err| error!("{err}")).ok() {
		install_with(&sdk, cfg)
	} else {
		install_without_sdk(cfg)
	}
}

//
//
// TODO: before try mount check the device already mounted.
//
//

pub fn install_without_sdk(_cfg: &Install) -> Result<OnDevicePath, Error> {
	//
	todo!()
}


#[cfg(unix)]
pub fn install_with(sdk: &Sdk, cfg: &Install) -> Result<OnDevicePath, Error> {
	if !cfg.pdx.try_exists()? {
		return Err(Error::Error(format!("Input file not exists: {}", cfg.pdx.display())));
	}
	if !cfg.pdx.is_dir() ||
	   cfg.pdx
	      .extension()
	      .map(|s| s.to_string_lossy().to_lowercase())
	      .as_deref() !=
	   Some(PDX_PKG_EXT)
	{
		return Err(Error::Error(format!(
			"Sims to input directory is not PDX package: {}",
			cfg.pdx.display()
		)));
	}

	let mut device = find_one_device(cfg.mount.device.clone())?;

	if device.mounted() {
		device.refresh()?;
	}

	let mut mount_handle = if device.mounted() {
		debug!("{device} is already mounted");
		if let Some(vol) = device.volume.as_deref() {
			let mut handle = MountHandle::new(vol.to_owned());
			handle.unmount_on_drop(false);
			handle
		} else {
			unreachable!()
		}
	} else {
		crate::mount::mount_with(&sdk, &cfg.mount.mount, &device)?
	};

	// mount the device:
	// let mount_handle = crate::mount::mount_with(&sdk, &cfg.mount.mount, &device)?;
	let mount = mount_handle.path();

	// wait mounting:
	let duration = Duration::from_millis(100);
	let max = Duration::from_secs(6);
	// let is_mounted = || mount.try_exists().ok().unwrap_or_default();
	let mut is_mounted = {
		let device = &mut device;
		move || mount.try_exists().ok().unwrap_or_default() || (device.refresh().is_ok() && device.mounted())
	};
	// first try:
	let need_try = wait_for_mut(&mut is_mounted, duration, max).or_else(|err| {
		               error!("{err}");
		               warn!("If your OS does not automatically mount your Playdate, please do so now.");
		               //   wait_for(&mut is_mounted, duration, Duration::from_secs(60 * 2))
		               Ok::<_, Error>(())
	               })
	               .is_err();
	// last try:
	if need_try {
		wait_for_mut(&mut is_mounted, duration, Duration::from_secs(60 * 2))?;
	}

	// update mount-point if needed:
	if let Some(vol) = device.volume.as_deref() {
		if vol != mount_handle.path() {
			debug!("Mount point changed to {}", vol.display());
			mount_handle.set_mount_point(vol.to_owned());
		}
	}


	info!("Device {device} mounted successfully");
	let _ = std::fs::read_dir(mount_handle.path()).map(|entries| entries.count())
	                                              .ok();


	let games = mount_handle.path().join("Games");
	// This prevents issues that occur when the PLAYDATE volume is mounted
	// but not all of the inner folders are available yet.
	let is_fs_ok = || games.try_exists().ok().unwrap_or_default();
	let max = Duration::from_secs(10);
	debug!("Waiting fs availability for {device}...");
	wait_for(is_fs_ok, duration, max).map_err(|err| {
		                                 error!("Device {device} directory '{}' not found, {err}", games.display());
		                                 err
	                                 })?;


	// copying the game:
	info!("Copying PDX to {device}, do not eject.");
	let pdx_filename = cfg.pdx.file_name().expect("filename");
	let target = games.join(&pdx_filename);
	debug!("Copying PDX to '{}'", target.display());
	std::fs::copy(&cfg.pdx, &target).map_err(Error::from)
	                                .or_else(|_| {
		                                // -f for force
		                                #[cfg(unix)]
		                                Command::new("cp").arg("-r")
		                                                  .arg(&cfg.pdx)
		                                                  .arg(&games)
		                                                  .status()?
		                                                  .exit_ok()?;
		                                Ok::<_, Error>(0)
	                                })?;
	info!("Copied: {}", pdx_filename.to_string_lossy());

	Ok(OnDevicePath { path: PathBuf::from(games.file_name().unwrap()).join(&pdx_filename),
	                  device })
}


#[cfg(windows)]
pub fn install_with(sdk: &Sdk, cfg: &Install) -> Result<OnDevicePath, Error> {
	if !cfg.pdx.try_exists()? {
		return Err(Error::Error(format!("Input file not exists: {}", cfg.pdx.display())));
	}
	if cfg.pdx
	      .extension()
	      .map(|s| s.to_string_lossy().to_lowercase())
	      .as_deref() !=
	   Some(PDX_PKG_EXT)
	{
		return Err(Error::Error(format!("Sims to input file is not PDX: {}", cfg.pdx.display())));
	}

	Command::new(sdk.pdutil()).arg("install")
	                          .arg(&cfg.pdx)
	                          .status()?
	                          .exit_ok()?;

	Ok(OnDevicePath { path: PathBuf::from("Games").join(&cfg.pdx.file_name().expect("filename")),
	                  device: todo!() })
}
