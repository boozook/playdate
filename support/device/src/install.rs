use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use futures::{FutureExt, Stream, StreamExt, TryFutureExt};

use crate::device::query::Query;
use crate::error::Error;
use crate::mount::MountedDevice;
use crate::mount;
use crate::retry::Retries;


type Result<T = (), E = Error> = std::result::Result<T, E>;


/// On-device path with owned mounted device.
pub struct MountedDevicePath {
	drive: MountedDevice,
	path: String,
}

/// On-device path with borrowed mounted device.
pub struct MountedDevicePathBorrowed<'dev> {
	drive: &'dev MountedDevice,
	path: String,
}

impl<'dev> MountedDevicePathBorrowed<'dev> {
	pub fn drive(&self) -> &MountedDevice { &self.drive }

	/// Local on-device path.
	pub fn path_local(&self) -> &str { &self.path }
	/// Absolute on-host path.
	pub fn path_abs(&self) -> PathBuf { self.drive.handle.path().join(&self.path) }

	pub fn into_path(self) -> String { self.path }
	pub fn into_parts(self) -> (&'dev MountedDevice, String) { (self.drive, self.path) }

	pub fn to_owned_replacing(self) -> impl FnOnce(MountedDevice) -> MountedDevicePath {
		let (_, path) = self.into_parts();
		move |drive| MountedDevicePath { drive, path }
	}
}

impl MountedDevicePath {
	pub fn drive(&self) -> &MountedDevice { &self.drive }
	pub fn drive_mut(&mut self) -> &mut MountedDevice { &mut self.drive }

	/// Local on-device path.
	pub fn path_local(&self) -> &str { &self.path }
	/// Absolute on-host path.
	pub fn path_abs(&self) -> PathBuf { self.drive.handle.path().join(&self.path) }

	pub fn into_path(self) -> String { self.path }
	pub fn into_parts(self) -> (MountedDevice, String) { (self.drive, self.path) }
}


/// Install package on the device.
///
/// `path` is a host filesystem path to pdx.
#[cfg_attr(feature = "tracing", tracing::instrument(skip(drive)))]
pub async fn install<'dev>(drive: &'dev MountedDevice,
                           path: &Path,
                           force: bool)
                           -> Result<MountedDevicePathBorrowed<'dev>> {
	#[cfg(all(feature = "tokio", not(feature = "async-std")))]
	use tokio::process::Command;
	#[cfg(feature = "async-std")]
	use async_std::process::Command;
	#[cfg(all(not(feature = "tokio"), not(feature = "async-std")))]
	use std::process::Command;


	let retry = Retries::new(Duration::from_millis(500), Duration::from_secs(60));
	mount::wait_fs_available(drive, retry).await?;
	validate_host_package(path).await?;

	trace!(
	       "Installing: {} -> {}",
	       path.display(),
	       drive.handle.path().display()
	);

	let games = drive.handle.path().join("Games");

	let cp = || {
		async {
			if cfg!(unix) {
				let mut cmd = Command::new("cp");
				cmd.arg("-r");

				if force {
					cmd.arg("-f");
				}

				cmd.arg(path);
				cmd.arg(&games);

				#[cfg(feature = "tokio")]
				cmd.status().await?.exit_ok()?;
				#[cfg(not(feature = "tokio"))]
				cmd.status()?.exit_ok()?;
			} else if cfg!(windows) {
				// xcopy c:\test c:\test2\test /S /E /H /I /Y
				let mut cmd = Command::new("xcopy");
				cmd.arg(path);
				cmd.arg(games.join(path.file_name().unwrap()));

				cmd.args(["/S", "/E", "/H", "/I"]);
				if force {
					cmd.arg("/Y");
				}

				#[cfg(feature = "tokio")]
				cmd.status().await?.exit_ok()?;
				#[cfg(not(feature = "tokio"))]
				cmd.status()?.exit_ok()?;
			} else {
				unreachable!("Unsupported OS")
			}
			Ok::<_, Error>(())
		}
	};

	if !path.is_dir() {
		#[cfg(feature = "tokio")]
		{
			tokio::fs::copy(path, games.join(path.file_name().unwrap())).map_ok(|bytes| trace!("copied {bytes}"))
			                                                            .inspect_err(|err| error!("{err}"))
			                                                            .or_else(|_| cp())
			                                                            .await?;
		};
		#[cfg(not(feature = "tokio"))]
		{
			std::fs::copy(path, games.join(path.file_name().unwrap())).map(|bytes| trace!("copied {bytes}"))
			                                                          .inspect_err(|err| error!("{err}"))
			                                                          .or_else(|_| {
				                                                          futures_lite::future::block_on(cp())
			                                                          })?;
		}
	} else {
		cp().await?;
	}

	// on-dev-path:
	let path = format!("/Games/{}", path.file_name().unwrap().to_string_lossy());
	Ok(MountedDevicePathBorrowed { drive, path })
}


/// 1. Mount if needed
/// 1. Wait for FS to become available
/// 1. Install package
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn mount_and_install(query: Query,
                               path: &Path,
                               force: bool)
                               -> Result<impl Stream<Item = Result<MountedDevicePath>> + '_> {
	validate_host_package(path).await?;

	// TODO: Check query is path and this is mounted volume.

	let fut = mount::mount_and(query, true).await?.flat_map(move |res| {
		                                              async move {
			                                              match res {
				                                              Ok(drive) => {
				                                                 let path = install(&drive, path, force).await?;
				                                                 Ok(path.to_owned_replacing()(drive))
			                                                 },
			                                                 Err(err) => Err(err),
			                                              }
		                                              }.into_stream()
	                                              });
	Ok(fut)
}


/// Validate path - pdz or pdx-dir.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn validate_host_package(path: &Path) -> Result<()> {
	use std::io::{Error, ErrorKind};

	if !path.try_exists()? {
		return Err(Error::new(ErrorKind::NotFound, "package not found").into());
	}

	(path.is_dir() ||
	     path.extension() == Some(OsStr::new("pdz")) ||
	     path.extension() == Some(OsStr::new("pdx")))
		  .then_some(())
		  .ok_or_else(|| Error::new(ErrorKind::InvalidData, "invalid package").into())
}
