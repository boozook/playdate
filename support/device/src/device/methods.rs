#![cfg(feature = "tokio")]

use crate::error::Error;
use crate::usb::mode::Mode;

use super::Device;


type Result<T = (), E = Error> = std::result::Result<T, E>;


pub async fn wait_mode_storage(dev: Device) -> Result<Device> { wait_mode_change(dev, Mode::Storage).await }

pub async fn wait_mode_data(dev: Device) -> Result<Device> { wait_mode_change(dev, Mode::Data).await }


// TODO: make timeout configurable
pub async fn wait_mode_change(mut dev: Device, to: Mode) -> Result<Device> {
	const ITER: u64 = 40; // ms
	const RETRIES: u8 = 130; // ≈5 sec
	let mut counter = RETRIES;
	let every = std::time::Duration::from_millis(ITER);
	let mut interval = tokio::time::interval(every);

	while {
		counter -= 1;
		counter
	} != 0
	{
		interval.tick().await;
		trace!("try: {}/{RETRIES}", RETRIES - counter);

		let mode = dev.mode_cached();
		dev.info()
		   .serial_number()
		   .map(|s| trace!("waiting mode {to} of {s}, current: {mode}"));

		if mode == to {
			dev.info().serial_number().map(|s| trace!("{s} is in {to} mode."));
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
