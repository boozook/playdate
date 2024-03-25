use std::borrow::Cow;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use std::path::PathBuf;

use futures::stream::FuturesUnordered;
use futures::{FutureExt, TryStreamExt};
use futures_lite::StreamExt;
use utils::toolchain::sdk::Sdk;

use crate::device::query::DeviceQuery as Query;
use crate::error::Error;
use crate::mount::UnmountAsync;
use crate::{install, device, usb, interface};


type Result<T = (), E = Error> = std::result::Result<T, E>;


pub async fn run_on_device(query: Query,
                           pdx: PathBuf,
                           no_install: bool,
                           no_read: bool,
                           force: bool)
                           -> Result<Vec<device::Device>> {
	let to_run = if !no_install {
		install::mount_and_install(query, &pdx, force).await?
		                                              .filter_map(|r| r.map_err(|e| error!("{e}")).ok())
		                                              .flat_map(|path| {
			                                              async {
				                                              let (mount, path) = path.into_parts();
				                                              mount.unmount().await?;
				                                              device::wait_mode_data(mount.device).await.map(|dev| {
					                                                                                        (dev, path.into())
				                                                                                        })
			                                              }.into_stream()
			                                              .filter_map(move |r| r.inspect_err(|e| error!("{e}")).ok())
		                                              })
		                                              .collect::<Vec<(device::Device, Cow<_>)>>()
		                                              .await
	} else {
		usb::discover::devices_data()?.map(|dev| (dev, pdx.to_string_lossy()))
		                              .collect()
	};


	let mut to_read = Vec::with_capacity(to_run.len());
	let readers = FuturesUnordered::new();

	for (mut device, path) in to_run {
		use interface::r#async::Out;

		device.open()?;
		{
			let interface = device.interface()?;
			interface.send_cmd(device::command::Command::Run { path: path.into_owned() })
			         .await?;
		}

		if !no_read {
			to_read.push(device);
		}
	}

	if !no_read {
		for device in to_read.iter_mut() {
			readers.push(usb::io::redirect_to_stdout(device));
		}
	}

	readers.inspect_err(|err| error!("{err}"))
	       .try_for_each_concurrent(8, |_| async { Ok(()) })
	       .await?;

	Ok(to_read)
}


pub async fn run_with_sim(pdx: PathBuf, sdk: Option<PathBuf>) -> Result<(), Error> {
	let sdk = sdk.map_or_else(|| Sdk::try_new(), Sdk::try_new_exact)?;

	let (pwd, sim) = if cfg!(target_os = "macos") {
		("Playdate Simulator.app/Contents/MacOs", "./Playdate Simulator")
	} else if cfg!(unix) {
		(".", "./PlaydateSimulator")
	} else if cfg!(windows) {
		(".", "PlaydateSimulator.exe")
	} else {
		return Err(IoError::new(IoErrorKind::Unsupported, "Unsupported platform").into());
	};


	#[cfg(feature = "tokio")]
	use tokio::process::Command;
	#[cfg(not(feature = "tokio"))]
	use std::process::Command;


	let mut cmd = Command::new(sim);
	cmd.current_dir(sdk.bin().join(pwd));
	cmd.arg(&pdx);

	debug!("Run: {cmd:?}");
	#[cfg(feature = "tokio")]
	cmd.status().await?.exit_ok()?;
	#[cfg(not(feature = "tokio"))]
	cmd.status()?.exit_ok()?;

	Ok(())
}
