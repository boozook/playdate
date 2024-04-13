use std::io::Write;

use futures_lite::future::block_on;


use futures_lite::StreamExt;
use nusb::transfer::RequestBuffer;
use nusb::{DeviceInfo, Interface};

use crate::device::Device;
use crate::error::Error;
use crate::serial::redirect_interface_to_stdout as redirect_serial_to_stdout;
use crate::usb::mode::DeviceMode;
use crate::usb::mode::Mode;
use crate::usb::BULK_IN;


#[cfg_attr(feature = "tracing", tracing::instrument(skip(interface)))]
pub fn read_interface(interface: &Interface,
                      buf_size: usize,
                      bufs: usize)
                      -> Result<impl futures_lite::stream::Stream<Item = Result<String, Error>>, Error> {
	let mut inp = interface.bulk_in_queue(BULK_IN);

	// preallocate buffers
	while inp.pending() < bufs {
		inp.submit(RequestBuffer::new(buf_size));
	}

	let stream = futures_lite::stream::poll_fn(move |ctx| {
		inp.poll_next(ctx)
		   .map(|out| -> Result<_, Error> {
			   let data = out.into_result()?;
			   let s = std::str::from_utf8(&data)?.to_owned();
			   inp.submit(RequestBuffer::reuse(data, buf_size));
			   Ok(s)
		   })
		   .map(Some)
	});

	Ok(stream)
}

#[cfg_attr(feature = "tracing", tracing::instrument(skip(interface, map)))]
pub fn read_while_map<F, T>(interface: &Interface,
                            buf_size: usize,
                            buffers: usize,
                            mut map: F)
                            -> Result<impl futures_lite::stream::Stream<Item = T>, Error>
	where F: FnMut(&[u8]) -> Option<T>
{
	let mut inp = interface.bulk_in_queue(BULK_IN);

	// preallocate buffers
	while inp.pending() < buffers {
		inp.submit(RequestBuffer::new(buf_size));
	}

	let stream = futures_lite::stream::poll_fn(move |ctx| {
		inp.poll_next(ctx).map(|out| -> Option<_> {
			                  match out.into_result() {
				                  Ok(data) => {
				                     let res = map(data.as_slice());
				                     if res.is_some() {
					                     inp.submit(RequestBuffer::reuse(data, buf_size));
				                     } else {
					                     trace!("cancel all IN queue, by predicate.");
					                     inp.cancel_all();
				                     }
				                     res
			                     },
			                     Err(err) => {
				                     trace!("cancel all IN queue, by err: {err}.");
				                     inp.cancel_all();
				                     None
			                     },
			                  }
		                  })
	});

	Ok(stream)
}


#[cfg_attr(feature = "tracing", tracing::instrument)]
pub fn read_once(device: DeviceInfo) -> Result<(String, Interface), Error> {
	let mode = device.mode();
	if !matches!(mode, Mode::Data) {
		return Err(Error::WrongState(mode));
	}


	let device = device.open()?;
	let inter = device.claim_interface(1)?;

	let stream = read_while_map(&inter, 256, 2, |data| {
		             match std::str::from_utf8(data) {
			             Ok(s) => {
			                if s.trim().is_empty() {
				                None
			                } else {
				                Some(s.to_owned())
			                }
		                },
		                Err(err) => {
			                error!("{err:?}");
			                None
		                },
		             }
	             })?.fold(String::new(), |acc, ref s| acc + s);
	let s = block_on(stream);
	Ok((s, inter))
}


#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn redirect_to_stdout(device: &mut Device) -> Result<(), Error> {
	let mode = device.mode();
	if !matches!(mode, Mode::Data) {
		return Err(Error::WrongState(mode));
	}

	device.open()?;
	redirect_interface_to_stdout(device.interface_mut()?).await?;

	Ok(())
}

#[cfg_attr(feature = "tracing", tracing::instrument)]
pub async fn redirect_interface_to_stdout(interface: &mut crate::interface::Interface) -> Result<(), Error> {
	match interface {
		crate::interface::Interface::Usb(interface) => {
			let mut stdout = std::io::stdout();
			let to_stdout = move |data: &[u8]| stdout.write_all(data).inspect_err(|err| error!("{err}")).ok();
			let stream = read_while_map(&interface.inner, 256, 2, to_stdout)?;
			if let Some(_) = stream.last().await {
				trace!("Read stream complete.");
			}
		},
		crate::interface::Interface::Serial(interface) => {
			interface.open()?;
			redirect_serial_to_stdout(interface).await?;
		},
	}
	Ok(())
}
