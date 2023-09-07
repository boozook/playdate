use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::time::Duration;

use playdate::toolchain::sdk::Sdk;

use crate::Error;
use crate::OnDevicePath;
use crate::wait_for_mut;
use super::find_one_device;
use super::install::Install;


/// Run a given pdx-package to the device or simulator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(feature = "cli", command(author, version, about, long_about = None, name = "run"))]
pub struct Run {
	#[cfg_attr(feature = "cli", clap(subcommand))]
	pub destination: Destination,
}


#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Subcommand))]
pub enum Destination {
	/// Install to the device.
	///
	/// Attention: <PACKAGE> parameter is a local path to pdx-package.
	/// But in case of '--no-install' given path will interpret as on-device relative to it's root path,
	/// e.g. "Games/my-game.pdx".
	#[cfg_attr(feature = "cli", clap(alias("dev")))]
	Device(DeviceDestination),

	/// Run with simulator.
	#[cfg_attr(feature = "cli", clap(alias("sim")))]
	Simulator(SimDestination),
}

impl std::fmt::Display for Destination {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let name = match self {
			Destination::Device(_) => "device",
			Destination::Simulator(_) => "simulator",
		};
		write!(f, "{name}")
	}
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
pub struct DeviceDestination {
	#[cfg_attr(feature = "cli", command(flatten))]
	pub install: Install,

	/// Do not install pdx to the device.
	/// If so, <PACKAGE> path will be interpreted as on-device path of already installed package,
	/// relative to the root of device's data partition.
	/// Add '--help' for more info.
	#[cfg_attr(feature = "cli", arg(long, name = "no-install", default_value_t = false))]
	pub no_install: bool,

	/// Do not wait connect & listen to the device after execution.
	/// Exits immediately after send 'run' command.
	#[cfg_attr(feature = "cli", arg(long, name = "no-wait", default_value_t = false))]
	pub no_wait: bool,
}


#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
pub struct SimDestination {
	/// Path to the PDX package.
	#[cfg_attr(feature = "cli", arg(value_name = "PACKAGE"))]
	pub pdx: PathBuf,
}


pub fn run(cfg: Run) -> Result<(), Error> {
	match cfg.destination {
		Destination::Device(cfg) => {
			run_on_device(cfg)?;
		},
		Destination::Simulator(cfg) => {
			run_on_sim(cfg)?;
		},
	}
	Ok(())
}


pub fn run_on_sim(cfg: SimDestination) -> Result<(), Error> {
	let sdk = Sdk::try_new()?;

	let (pwd, sim) = if cfg!(target_os = "macos") {
		("Playdate Simulator.app/Contents/MacOs", "./Playdate Simulator")
	} else if cfg!(unix) {
		(".", "./PlaydateSimulator")
	} else if cfg!(windows) {
		(".", "PlaydateSimulator.exe")
	} else {
		return Err(Error::Err("Unsupported platform"));
	};

	let mut cmd = Command::new(sim);
	cmd.current_dir(sdk.bin().join(pwd));
	cmd.arg(&cfg.pdx);

	debug!("Run: {cmd:?}");
	cmd.status()?.exit_ok()?;
	Ok(())
}


pub fn run_on_device(cfg: DeviceDestination) -> Result<OnDevicePath, Error> {
	let path = if let Some(sdk) = Sdk::try_new().map_err(|err| error!("{err}")).ok() {
		run_on_device_with(&sdk, &cfg)
	} else {
		run_on_device_without_sdk(&cfg)
	}?;

	#[cfg(any(not(windows), feature = "usb"))]
	if !cfg.no_wait {
		path.device.read_to_stdout(Some(false))?;
	}

	debug!("Run on {}: finished", &path.device);
	Ok(path)
}


fn find_ondevice_path(cfg: &DeviceDestination) -> Result<OnDevicePath, Error> {
	let device = find_one_device(cfg.install.mount.device.clone())?;
	let path = if cfg.install.pdx.try_exists().ok().unwrap_or_default() {
		           warn!("Sims to given path to pdx-package is a local path, but on-device path expected.");

		           // try convert to on-device path if possible:
		           let input_path = &cfg.install.pdx;
		           if input_path.components().count() > 2 && !input_path.starts_with("Games/") {
			           let new = Path::new("Games").join(input_path.file_name().expect("filename"));
			           debug!("Adapted path: {} <- {}", new.display(), input_path.display());
			           Cow::from(new)
		           } else {
			           cfg.install.pdx.as_path().into()
		           }
	           } else {
		           cfg.install.pdx.as_path().into()
	           }.into();
	Ok(OnDevicePath { path, device })
}


pub fn run_on_device_without_sdk(cfg: &DeviceDestination) -> Result<OnDevicePath, Error> {
	let path = if cfg.no_install {
		find_ondevice_path(&cfg)?
	} else {
		let mut installed = super::install::install_without_sdk(&cfg.install)?;

		// wait for unmount and connected again
		let err_msg = format!("Unable to connect, device {} not found.", &installed.device);

		let mut is_connected = {
			let installed = &mut installed;
			move || {
				installed.device.refresh().ok();
				installed.device.is_cu_ok()
			}
		};


		let duration = Duration::from_millis(50);
		let max = Duration::from_secs(10);
		// first try:
		let need_try = wait_for_mut(&mut is_connected, duration, max).map_err(|err| {
			                                                             debug!("{err:?}");
			                                                             warn!("{err_msg}");
		                                                             })
		                                                             .is_err();
		// last try:
		if need_try {
			wait_for_mut(&mut is_connected, duration, max).map_err(|err| {
				                                              debug!("{err:?}");
				                                              error!("{err_msg}");
				                                              err
			                                              })?;
		}

		debug!("{:?} was found", &installed.device);
		installed
	};

	// run:
	path.device
	    .write(format!("run {}\n", path.path.display()))
	    .map_err(|err| {
		    error!("{err}");
		    Error::Error(format!("Unable to send command to {}.", &path.device))
	    })?;

	Ok(path)
}


pub fn run_on_device_with(sdk: &Sdk, cfg: &DeviceDestination) -> Result<OnDevicePath, Error> {
	let path = if cfg.no_install {
		find_ondevice_path(&cfg)?
	} else {
		let mut installed = super::install::install_with(&sdk, &cfg.install)?;

		// wait for unmount and connected again
		let err_msg = format!("Unable to connect, device {} not found.", &installed.device);

		let mut is_connected = {
			let installed = &mut installed;
			move || {
				installed.device.refresh().ok();
				installed.device.is_cu_ok()
			}
		};


		let duration = Duration::from_millis(50);
		let max = Duration::from_secs(10);
		// first try:
		let need_try = wait_for_mut(&mut is_connected, duration, max).map_err(|err| {
			                                                             debug!("{err:?}");
			                                                             warn!("{err_msg}");
		                                                             })
		                                                             .is_err();
		// last try:
		if need_try {
			wait_for_mut(&mut is_connected, duration, max).map_err(|err| {
				                                              debug!("{err:?}");
				                                              error!("{err_msg}");
				                                              err
			                                              })?;
		}

		debug!("{:?} was found", &installed.device);
		installed
	};

	// run:

	if cfg!(feature = "usb") {
		path.device
		    .write(format!("run {}\n", path.path.display()))
		    .map_err(|err| {
			    error!("{err}");
			    Error::Error(format!("Unable to send command to {}.", &path.device))
		    })?;
		debug!("run cmd sent, waiting for boot finished");
		// this needed for case when we can catch a device immediately after reboot and OS did not loaded yet,
		// but we're writing `run` to buffer, so OS will read it after successful load and execute the command.
		// But if we write another command before OS read previous, it will overwritten and not read by OS.
		// And I don't know how to determine state of OS. (Perhaps just ask `ping`?)
		// So we must to wait.
		// TODO: implement write `ping` and read answer
		std::thread::sleep(Duration::from_secs(2));
		debug!("continuing");
	} else {
		let mut cmd = Command::new(sdk.pdutil());
		let tty = path.device
		              .tty
		              .as_deref()
		              .ok_or(Error::unable_to_find_tty(&path.device))?;
		cmd.arg(&tty).arg("run").arg(&path.path);
		cmd.stdout(Stdio::inherit());
		cmd.stderr(Stdio::inherit());
		debug!("Run: {cmd:?}");
		cmd.status()?.exit_ok()?;
	}

	Ok(path)
}
