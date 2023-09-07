use std::time::Duration;
use rusb::{Device, DeviceDescriptor, DeviceHandle, Direction, Result, TransferType, UsbContext};

#[derive(Debug)]
struct Endpoint {
	config: u8,
	iface: u8,
	setting: u8,
	address: u8,
}

// TODO: impl real IO mode for ping for example:
// - buffered read, write
// - within one non-global context


pub fn read_device<T: UsbContext>(device: &mut Device<T>,
                                  device_desc: &DeviceDescriptor,
                                  handle: &mut DeviceHandle<T>)
                                  -> Result<()> {
	debug!("USB: Start reading device...");
	handle.reset()?;

	match find_readable_endpoint(device, device_desc, TransferType::Interrupt) {
		Some(endpoint) => read_endpoint(handle, endpoint, TransferType::Interrupt),
		None => debug!("No readable interrupt endpoint"),
	}

	match find_readable_endpoint(device, device_desc, TransferType::Bulk) {
		Some(endpoint) => read_endpoint(handle, endpoint, TransferType::Bulk),
		None => debug!("No readable bulk endpoint"),
	}

	Ok(())
}

pub fn write_device<T: UsbContext, S: AsRef<str>>(device: &mut Device<T>,
                                                  device_desc: &DeviceDescriptor,
                                                  handle: &mut DeviceHandle<T>,
                                                  command: S)
                                                  -> Result<()> {
	debug!("USB: Start writing to device '{}'...", command.as_ref());
	handle.reset()?;

	match find_writable_endpoint(device, device_desc, TransferType::Interrupt) {
		Some(endpoint) => {
			write_endpoint(
			               handle,
			               endpoint,
			               TransferType::Interrupt,
			               command.as_ref().as_bytes(),
			)
		},
		None => debug!("No writable interrupt endpoint"),
	}

	match find_writable_endpoint(device, device_desc, TransferType::Bulk) {
		Some(endpoint) => write_endpoint(handle, endpoint, TransferType::Bulk, command.as_ref().as_bytes()),
		None => debug!("No writable bulk endpoint"),
	}

	Ok(())
}

fn find_readable_endpoint<T: UsbContext>(device: &mut Device<T>,
                                         device_desc: &DeviceDescriptor,
                                         transfer_type: TransferType)
                                         -> Option<Endpoint> {
	for n in 0..device_desc.num_configurations() {
		let config_desc = match device.config_descriptor(n) {
			Ok(c) => c,
			Err(_) => continue,
		};

		for interface in config_desc.interfaces() {
			for interface_desc in interface.descriptors() {
				for endpoint_desc in interface_desc.endpoint_descriptors() {
					if endpoint_desc.direction() == Direction::In && endpoint_desc.transfer_type() == transfer_type {
						return Some(Endpoint { config: config_desc.number(),
						                       iface: interface_desc.interface_number(),
						                       setting: interface_desc.setting_number(),
						                       address: endpoint_desc.address() });
					}
				}
			}
		}
	}

	None
}

fn find_writable_endpoint<T: UsbContext>(device: &mut Device<T>,
                                         device_desc: &DeviceDescriptor,
                                         transfer_type: TransferType)
                                         -> Option<Endpoint> {
	for n in 0..device_desc.num_configurations() {
		let config_desc = match device.config_descriptor(n) {
			Ok(c) => c,
			Err(_) => continue,
		};

		for interface in config_desc.interfaces() {
			for interface_desc in interface.descriptors() {
				for endpoint_desc in interface_desc.endpoint_descriptors() {
					if endpoint_desc.direction() == Direction::Out && endpoint_desc.transfer_type() == transfer_type {
						return Some(Endpoint { config: config_desc.number(),
						                       iface: interface_desc.interface_number(),
						                       setting: interface_desc.setting_number(),
						                       address: endpoint_desc.address() });
					}
				}
			}
		}
	}

	None
}

fn read_endpoint<T: UsbContext>(handle: &mut DeviceHandle<T>, endpoint: Endpoint, transfer_type: TransferType) {
	debug!("Reading from endpoint: {:?}", endpoint);

	let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
		Ok(true) => {
			handle.detach_kernel_driver(endpoint.iface).ok();
			true
		},
		_ => false,
	};
	debug!("  kernel driver: {}", has_kernel_driver);

	match configure_endpoint(handle, &endpoint) {
		Ok(_) => {
			let mut buf = [0; 256];
			let timeout = Duration::from_secs(60);

			debug!("  read while...");
			let mut ok = true;
			while ok {
				let res = match transfer_type {
					TransferType::Interrupt => handle.read_interrupt(endpoint.address, &mut buf, timeout),
					TransferType::Bulk => handle.read_bulk(endpoint.address, &mut buf, timeout),
					_ => unreachable!(),
				};
				ok = res.is_ok();
				match res {
					Ok(len) => {
						use std::io::prelude::*;
						std::io::stdout().write_all(&buf[..len]).ok();
					},
					Err(err) => error!("could not read from endpoint: {}", err),
				}
			}
		},
		Err(err) => error!("could not configure endpoint: {}", err),
	}

	if has_kernel_driver {
		handle.attach_kernel_driver(endpoint.iface).ok();
	}
}


fn write_endpoint<T: UsbContext>(handle: &mut DeviceHandle<T>,
                                 endpoint: Endpoint,
                                 transfer_type: TransferType,
                                 buf: &[u8]) {
	debug!("Writing from endpoint: {:?}", endpoint);

	let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
		Ok(true) => {
			handle.detach_kernel_driver(endpoint.iface).ok();
			true
		},
		_ => false,
	};
	debug!("  kernel driver: {}", has_kernel_driver);

	match configure_endpoint(handle, &endpoint) {
		Ok(_) => {
			let timeout = Duration::from_secs(60 * 4);

			debug!("  write {buf:?}");
			let res = match transfer_type {
				TransferType::Interrupt => handle.write_interrupt(endpoint.address, &buf, timeout),
				TransferType::Bulk => handle.write_bulk(endpoint.address, &buf, timeout),
				_ => unreachable!(),
			};
			match res {
				Ok(len) => debug!("  wrote {len} bytes"),
				Err(err) => error!("could not read from endpoint: {}", err),
			}
		},
		Err(err) => error!("could not configure endpoint: {}", err),
	}

	if has_kernel_driver {
		handle.attach_kernel_driver(endpoint.iface).ok();
	}
}

fn configure_endpoint<T: UsbContext>(handle: &mut DeviceHandle<T>, endpoint: &Endpoint) -> Result<()> {
	handle.set_active_configuration(endpoint.config)?;
	handle.claim_interface(endpoint.iface)?;
	handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;
	Ok(())
}
