#![cfg(feature = "usb")]
use std::borrow::Cow;

use crate::Error;
use crate::model::Device;
use crate::model::Mode;
use crate::model::SerialNumber;

mod io;


pub fn list_usb_devices() -> Result<Vec<Device>, Error> {
	let mut found = Vec::new();

	#[cfg(debug_assertions)]
	debug::list_devices()?;

	for device in rusb::devices()?.iter().into_iter() {
		match usb_to_pd(&device) {
			Ok(Some(pd)) => found.push(pd),
			Err(err) => debug!("{err}"),
			Ok(None) => continue,
		}
	}
	Ok(found)
}


fn usb_to_pd(device: &rusb::Device<rusb::GlobalContext>) -> Result<Option<Device>, Error> {
	use rusb::constants::{LIBUSB_CLASS_COMM, LIBUSB_CLASS_DATA, LIBUSB_CLASS_MASS_STORAGE};

	let descriptor = device.device_descriptor()?;
	if descriptor.vendor_id() != 0x1331 {
		return Ok(None);
	}

	// Probably not need to filter product id because revB, etc..
	// if descriptor.product_id() != 0x5740 { continue }

	let handle = device.open()?;
	let serial = handle.read_serial_number_string_ascii(&descriptor)?;
	let serial = SerialNumber::try_from(serial)?;
	// let man = handle.read_manufacturer_string_ascii(&descriptor)?;
	// if !man.contains("Panic") { continue; }

	// get pd device mode:
	let cfg_id = handle.active_configuration()?;
	let config_descriptor = device.config_descriptor(cfg_id - 1)?;
	let modes =
		config_descriptor.interfaces()
		                 .flat_map(|interface| {
			                 interface.descriptors().filter_map(|d| {
				                                        let class = d.class_code();
				                                        match class {
					                                        LIBUSB_CLASS_MASS_STORAGE => Some(Mode::Storage),
				                                           LIBUSB_CLASS_DATA | LIBUSB_CLASS_COMM => Some(Mode::Data),
				                                           _ => None,
				                                        }
			                                        })
		                 })
		                 .collect::<Vec<_>>();
	let mode = if modes.contains(&Mode::Storage) {
		Mode::Storage
	} else if modes.contains(&Mode::Data) {
		Mode::Data
	} else {
		return Err(Error::Err("Unknown device mode, required interfaces not found"));
	};


	Ok(Some(Device { serial,
	                 mode,
	                 volume: None,
	                 tty: None }))
}


pub fn read_output(device: &Device, echo: Option<bool>) -> Result<(), Error> {
	let (mut handle, _) = open(&device)?;
	let mut dev = handle.device();
	let descr = dev.device_descriptor()?;

	if let Some(echo) = echo {
		let v = if echo { "on" } else { "off" };
		io::write_device(&mut dev, &descr, &mut handle, format!("echo {v}"))?;
	}
	io::read_device(&mut dev, &descr, &mut handle)?;

	Ok(())
}

pub fn write<S: AsRef<str>>(device: &Device, command: S) -> Result<(), Error> {
	let (mut handle, _) = open(&device)?;
	let mut dev = handle.device();
	let descr = dev.device_descriptor()?;

	let command: Cow<str> = if command.as_ref().ends_with("\n") {
		command.as_ref().into()
	} else {
		format!("{}\n", command.as_ref()).into()
	};

	debug!("USB: write: '{}'", command.as_ref());

	io::write_device(&mut dev, &descr, &mut handle, command)?;

	Ok(())
}


pub fn open(device: &Device) -> Result<(rusb::DeviceHandle<rusb::GlobalContext>, Device), Error> {
	let (handle, pd) = if let Some(handle) = rusb::open_device_with_vid_pid(0x1331, 0x5740) {
		let pd = usb_to_pd(&handle.device()).ok()
		                                    .flatten()
		                                    .filter(|pd| pd.serial == device.serial && pd.mode == Mode::Data);
		if let Some(pd) = pd {
			Ok((handle, pd))
		} else if let Some((usb, dev)) = find(&device).ok().filter(|(_, d)| d.mode == Mode::Data) {
			usb.open().map(|handle| (handle, dev)).map_err(Error::from)
		} else {
			Err(Error::named_device_not_found(device.serial.to_string()))
		}
	} else if let Some((usb, dev)) = find(&device).ok().filter(|(_, d)| d.mode == Mode::Data) {
		usb.open().map(|handle| (handle, dev)).map_err(Error::from)
	} else {
		Err(Error::named_device_not_found(device.serial.to_string()))
	}?;
	Ok((handle, pd))
}


// XXX: This is absolutely ineffective, optimize it.
/// Search and return usb device looks like a playdate and a our simple interface struct `Device`.
pub fn find(device: &Device) -> Result<(rusb::Device<rusb::GlobalContext>, Device), Error> {
	let dev = rusb::devices()?.iter().into_iter().find_map(|dev| {
		                                             if let Some(pd) = usb_to_pd(&dev).ok().flatten() {
			                                             if pd.serial == device.serial {
				                                             Some((dev, pd))
			                                             } else {
				                                             None
			                                             }
		                                             } else {
			                                             None
		                                             }
	                                             });
	dev.ok_or(Error::named_device_not_found(device.serial.to_string()))
}


pub mod debug {
	use rusb::{ConfigDescriptor, DeviceDescriptor, DeviceHandle, DeviceList, EndpointDescriptor, InterfaceDescriptor,
	       Language, Result, Speed, UsbContext};
	use std::time::Duration;
	use usb_ids::{self, FromId};

	struct UsbDevice<T: UsbContext> {
		handle: DeviceHandle<T>,
		language: Language,
		timeout: Duration,
	}


	pub fn list_devices() -> Result<()> {
		let timeout = Duration::from_secs(1);

		for device in DeviceList::new()?.iter() {
			let device_desc = match device.device_descriptor() {
				Ok(d) => d,
				Err(_) => continue,
			};

			let mut usb_device = {
				match device.open() {
					Ok(h) => {
						match h.read_languages(timeout) {
							Ok(l) => {
								if !l.is_empty() {
									Some(UsbDevice { handle: h,
									                 language: l[0],
									                 timeout, })
								} else {
									None
								}
							},
							Err(_) => None,
						}
					},
					Err(_) => None,
				}
			};

			println!(
			         "Bus {:03} Device {:03} ID {:04x}:{:04x} {}",
			         device.bus_number(),
			         device.address(),
			         device_desc.vendor_id(),
			         device_desc.product_id(),
			         get_speed(device.speed())
			);
			print_device(&device_desc, &mut usb_device);

			for n in 0..device_desc.num_configurations() {
				let config_desc = match device.config_descriptor(n) {
					Ok(c) => c,
					Err(_) => continue,
				};

				print_config(&config_desc, &mut usb_device);

				for interface in config_desc.interfaces() {
					for interface_desc in interface.descriptors() {
						print_interface(&interface_desc, &mut usb_device);

						for endpoint_desc in interface_desc.endpoint_descriptors() {
							print_endpoint(&endpoint_desc);
						}
					}
				}
			}
		}

		Ok(())
	}

	fn print_device<T: UsbContext>(device_desc: &DeviceDescriptor, handle: &mut Option<UsbDevice<T>>) {
		let vid = device_desc.vendor_id();
		let pid = device_desc.product_id();

		let vendor_name = match usb_ids::Vendor::from_id(device_desc.vendor_id()) {
			Some(vendor) => vendor.name(),
			None => "Unknown vendor",
		};

		let product_name = match usb_ids::Device::from_vid_pid(device_desc.vendor_id(), device_desc.product_id()) {
			Some(product) => product.name(),
			None => "Unknown product",
		};

		println!("Device Descriptor:");
		println!(
		         "  bcdUSB             {:2}.{}{}",
		         device_desc.usb_version().major(),
		         device_desc.usb_version().minor(),
		         device_desc.usb_version().sub_minor()
		);
		println!("  bDeviceClass        {:#04x}", device_desc.class_code());
		println!("  bDeviceSubClass     {:#04x}", device_desc.sub_class_code());
		println!("  bDeviceProtocol     {:#04x}", device_desc.protocol_code());
		println!("  bMaxPacketSize0      {:3}", device_desc.max_packet_size());
		println!("  idVendor          {vid:#06x} {vendor_name}",);
		println!("  idProduct         {pid:#06x} {product_name}",);
		println!(
		         "  bcdDevice          {:2}.{}{}",
		         device_desc.device_version().major(),
		         device_desc.device_version().minor(),
		         device_desc.device_version().sub_minor()
		);
		println!(
		         "  iManufacturer        {:3} {}",
		         device_desc.manufacturer_string_index().unwrap_or(0),
		         handle.as_mut().map_or(String::new(), |h| {
			         h.handle
			          .read_manufacturer_string(h.language, device_desc, h.timeout)
			          .unwrap_or_default()
		         })
		);
		println!(
		         "  iProduct             {:3} {}",
		         device_desc.product_string_index().unwrap_or(0),
		         handle.as_mut().map_or(String::new(), |h| {
			         h.handle
			          .read_product_string(h.language, device_desc, h.timeout)
			          .unwrap_or_default()
		         })
		);
		println!(
		         "  iSerialNumber        {:3} {}",
		         device_desc.serial_number_string_index().unwrap_or(0),
		         handle.as_mut().map_or(String::new(), |h| {
			         h.handle
			          .read_serial_number_string(h.language, device_desc, h.timeout)
			          .unwrap_or_default()
		         })
		);
		println!("  bNumConfigurations   {:3}", device_desc.num_configurations());
	}

	fn print_config<T: UsbContext>(config_desc: &ConfigDescriptor, handle: &mut Option<UsbDevice<T>>) {
		println!("  Config Descriptor:");
		println!("    bNumInterfaces       {:3}", config_desc.num_interfaces());
		println!("    bConfigurationValue  {:3}", config_desc.number());
		println!(
		         "    iConfiguration       {:3} {}",
		         config_desc.description_string_index().unwrap_or(0),
		         handle.as_mut().map_or(String::new(), |h| {
			         h.handle
			          .read_configuration_string(h.language, config_desc, h.timeout)
			          .unwrap_or_default()
		         })
		);
		println!("    bmAttributes:");
		println!("      Self Powered     {:>5}", config_desc.self_powered());
		println!("      Remote Wakeup    {:>5}", config_desc.remote_wakeup());
		println!("    bMaxPower           {:4}mW", config_desc.max_power());

		if !config_desc.extra().is_empty() {
			println!("    {:?}", config_desc.extra());
		} else {
			println!("    no extra data");
		}
	}

	fn print_interface<T: UsbContext>(interface_desc: &InterfaceDescriptor, handle: &mut Option<UsbDevice<T>>) {
		println!("    Interface Descriptor:");
		println!(
		         "      bInterfaceNumber     {:3}",
		         interface_desc.interface_number()
		);
		println!("      bAlternateSetting    {:3}", interface_desc.setting_number());
		println!("      bNumEndpoints        {:3}", interface_desc.num_endpoints());
		println!("      bInterfaceClass     {:#04x}", interface_desc.class_code());
		println!(
		         "      bInterfaceSubClass  {:#04x}",
		         interface_desc.sub_class_code()
		);
		println!(
		         "      bInterfaceProtocol  {:#04x}",
		         interface_desc.protocol_code()
		);
		println!(
		         "      iInterface           {:3} {}",
		         interface_desc.description_string_index().unwrap_or(0),
		         handle.as_mut().map_or(String::new(), |h| {
			         h.handle
			          .read_interface_string(h.language, interface_desc, h.timeout)
			          .unwrap_or_default()
		         })
		);

		if interface_desc.extra().is_empty() {
			println!("    {:?}", interface_desc.extra());
		} else {
			println!("    no extra data");
		}
	}

	fn print_endpoint(endpoint_desc: &EndpointDescriptor) {
		println!("      Endpoint Descriptor:");
		println!(
		         "        bEndpointAddress    {:#04x} EP {} {:?}",
		         endpoint_desc.address(),
		         endpoint_desc.number(),
		         endpoint_desc.direction()
		);
		println!("        bmAttributes:");
		println!(
		         "          Transfer Type          {:?}",
		         endpoint_desc.transfer_type()
		);
		println!("          Synch Type             {:?}", endpoint_desc.sync_type());
		println!(
		         "          Usage Type             {:?}",
		         endpoint_desc.usage_type()
		);
		println!(
		         "        wMaxPacketSize    {:#06x}",
		         endpoint_desc.max_packet_size()
		);
		println!("        bInterval            {:3}", endpoint_desc.interval());
	}

	fn get_speed(speed: Speed) -> &'static str {
		match speed {
			Speed::SuperPlus => "10000 Mbps",
			Speed::Super => "5000 Mbps",
			Speed::High => " 480 Mbps",
			Speed::Full => "  12 Mbps",
			Speed::Low => " 1.5 Mbps",
			_ => "(unknown)",
		}
	}
}
