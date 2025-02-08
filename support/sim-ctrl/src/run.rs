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
	use std::process::Command;

	let pdx = pdx.canonicalize()?;
	let sdk = sdk.map_or_else(Sdk::try_new, Sdk::try_new_exact)?;

	let bin = sdk.bin();
	let mut cmd = if cfg!(target_os = "macos") {
		let mut cmd = Command::new("./Playdate Simulator");
		cmd.current_dir(bin.join("Playdate Simulator.app/Contents/MacOs"));
		cmd
	} else if cfg!(unix) {
		let mut cmd = Command::new("./PlaydateSimulator");
		cmd.current_dir(bin);
		cmd
	} else if cfg!(windows) {
		let mut cmd = Command::new(bin.join("PlaydateSimulator.exe"));
		cmd.current_dir(bin);
		cmd
	} else {
		return Err(IoError::new(IoErrorKind::Unsupported, "Unsupported platform").into());
	};

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
