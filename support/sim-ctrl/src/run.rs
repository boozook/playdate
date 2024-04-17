use std::path::Path;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use utils::toolchain::sdk::Sdk;


#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn run(pdx: &Path, sdk: Option<&Path>) -> Result<(), Error> {
	#[cfg(all(feature = "tokio", not(feature = "async-std")))]
	use tokio::process::Command;
	#[cfg(feature = "async-std")]
	use async_std::process::Command;

	#[allow(unused_mut)]
	let mut cmd = command(pdx, sdk)?;

	#[cfg(any(feature = "tokio", feature = "async-std"))]
	let mut cmd = Command::from(cmd);

	trace!("executing: {cmd:?}");

	#[cfg(any(feature = "tokio", feature = "async-std"))]
	cmd.status().await?.exit_ok()?;
	#[cfg(all(not(feature = "tokio"), not(feature = "async-std")))]
	cmd.status()?.exit_ok()?;

	Ok(())
}


#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn command(pdx: &Path, sdk: Option<&Path>) -> Result<std::process::Command, Error> {
	let pdx = pdx.canonicalize()?;
	let sdk = sdk.map_or_else(Sdk::try_new, Sdk::try_new_exact)?;

	let (pwd, sim) = if cfg!(target_os = "macos") {
		("Playdate Simulator.app/Contents/MacOs", "./Playdate Simulator")
	} else if cfg!(unix) {
		(".", "./PlaydateSimulator")
	} else if cfg!(windows) {
		(".", "PlaydateSimulator.exe")
	} else {
		return Err(IoError::new(IoErrorKind::Unsupported, "Unsupported platform").into());
	};

	let mut cmd = std::process::Command::new(sim);
	cmd.current_dir(sdk.bin().join(pwd));
	cmd.arg(pdx);

	Ok(cmd)
}


pub use error::*;
mod error {
	#[derive(thiserror::Error, Debug)]
	pub enum Error {
		#[error(transparent)]
		Io {
			#[backtrace]
			#[from]
			source: std::io::Error,
		},
		#[error(transparent)]
		Exec {
			#[backtrace]
			#[from]
			source: std::process::ExitStatusError,
		},
	}
}
