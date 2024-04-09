use crate::retry::{IterTime, Retries};
use crate::usb::mode::Mode;
use crate::error::Error;

use super::Device;


type Result<T = (), E = Error> = std::result::Result<T, E>;


pub async fn wait_mode_storage<T>(dev: Device, retry: Retries<T>) -> Result<Device>
	where T: IterTime {
	wait_mode_change(dev, Mode::Storage, retry).await
}

pub async fn wait_mode_data<T>(dev: Device, retry: Retries<T>) -> Result<Device>
	where T: IterTime {
	wait_mode_change(dev, Mode::Data, retry).await
}


#[cfg_attr(feature = "tracing", tracing::instrument(skip(dev, retry), fields(dev = dev.info().serial_number())))]
pub async fn wait_mode_change(mut dev: Device, to: Mode, retry: Retries<impl IterTime>) -> Result<Device> {
	let total = &retry.total;
	let iter_ms = retry.iters.interval(total);
	let retries_num = total.as_millis() / iter_ms.as_millis();
	debug!("retries: {retries_num} * {iter_ms:?} ≈ {total:?}.");

	let mut counter = retries_num;
	#[cfg(all(feature = "tokio", not(feature = "async-std")))]
	let mut interval = tokio::time::interval(iter_ms);

	while {
		counter -= 1;
		counter
	} != 0
	{
		#[cfg(all(not(feature = "async-std"), feature = "tokio"))]
		interval.tick().await;
		#[cfg(feature = "async-std")]
		async_std::task::sleep(iter_ms).await;
		#[cfg(all(not(feature = "tokio"), not(feature = "async-std")))]
		std::thread::sleep(iter_ms);

		let mode = dev.mode_cached();
		trace!(
		       "{dev}: waiting mode {to}, current: {mode}, try: {}/{retries_num}",
		       retries_num - counter
		);

		if mode == to {
			trace!("{dev} is in {to} mode.");
			return Ok(dev);
		}

		if dev.refresh()? {
			if dev.mode_cached() == to {
				return Ok(dev);
			} else {
				trace!("refreshed to {mode} mode, waiting...")
			}
		}
	}

	Err(Error::usb_timeout(dev))
}
