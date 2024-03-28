use std::path::PathBuf;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use utils::toolchain::sdk::Sdk;


#[cfg_attr(feature = "tracing", tracing::instrument())]
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


pub use error::*;
mod error {
	// use std::backtrace::Backtrace;

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
