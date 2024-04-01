use futures::{FutureExt, Stream, StreamExt, TryFutureExt};
use futures_lite::stream;

use crate::device::command::Command;
use crate::{device, usb, interface};
use crate::error::Error;
use device::query::Query;
use interface::r#async::Out;


type Result<T = (), E = Error> = std::result::Result<T, E>;


/// Fails if can't map specified port to device in case of query is a port name/path.
/// Use [[send_to_interfaces]] instead if device mapping not needed.
#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn send_to_devs(query: Query,
                          cmd: Command,
                          read: bool)
                          -> Result<impl Stream<Item = Result<device::Device>>> {
	let devices = usb::discover::devices_data_for(query).await?;

	if devices.is_empty() {
		return Err(Error::not_found());
	}

	let devices = devices.into_iter().flat_map(|mut dev| {
		                                 dev.open().inspect_err(|err| error!("{err}")).ok()?;
		                                 Some(dev)
	                                 });
	let stream = stream::iter(devices).flat_map_unordered(None, move |mut dev| {
		                                  let cmd = cmd.clone();
		                                  async move {
			                                  match dev.interface_mut().inspect_err(|err| error!("{err}")) {
				                                  Ok(interface) => {
				                                     if read {
					                                     interface.send_cmd(cmd).await?;
					                                     usb::io::redirect_interface_to_stdout(interface).await?;
				                                     } else {
					                                     interface.send_cmd(cmd).await?;
				                                     }
				                                     Ok(())
			                                     },
			                                     Err(err) => Err(err),
			                                  }?;
			                                  Ok::<_, Error>(dev)
		                                  }.into_stream()
		                                  .boxed_local()
	                                  });
	Ok(stream)
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
pub async fn send_to_interfaces(query: Query,
                                cmd: Command)
                                -> Result<impl Stream<Item = Result<interface::Interface>>> {
	usb::discover::for_each_data_interface(query, move |interface| {
		let cmd = cmd.clone();
		async move {
			interface.send_cmd(cmd.clone())
			         .inspect_err(|err| error!("{err}"))
			         .await?;
			Ok::<_, Error>(interface)
		}
	}).await
}
