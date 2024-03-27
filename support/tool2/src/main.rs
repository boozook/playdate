#[cfg(feature = "tracing")]
#[macro_use]
extern crate tracing;

#[cfg(not(feature = "tracing"))]
#[macro_use]
extern crate log;

extern crate device as pddev;

use std::path::PathBuf;

use futures::{FutureExt, StreamExt, TryFutureExt};
use pddev::device::query::DeviceQueryValue;
use pddev::device::serial::SerialNumber;
use pddev::error::Error;
use pddev::device::query::DeviceQuery;
use pddev::*;


use miette::IntoDiagnostic;
use report::AsReport;


mod cli;
mod report;


#[cfg(all(feature = "tracing", not(feature = "console-subscriber")))]
fn enable_tracing() {
	use tracing::Level;
	use tracing_subscriber::fmt::Subscriber;

	let subscriber = Subscriber::builder().compact()
	                                      .with_file(true)
	                                      .with_target(false)
	                                      .with_line_number(true)
	                                      .without_time()
	                                      .with_level(true)
	                                      .with_thread_ids(false)
	                                      .with_thread_names(true)
	                                      .with_max_level(Level::TRACE)
	                                      .finish();
	tracing::subscriber::set_global_default(subscriber).unwrap();
}

#[cfg(all(feature = "tracing", feature = "console-subscriber"))]
fn enable_tracing() {
	use tracing::Level;
	use console_subscriber::ConsoleLayer;
	use tracing_subscriber::prelude::*;

	let console_layer = ConsoleLayer::builder().with_default_env().spawn();
	let fmt = tracing_subscriber::fmt::layer().with_file(true)
	                                          .with_target(false)
	                                          .with_line_number(true)
	                                          .without_time()
	                                          .with_level(true)
	                                          .with_thread_ids(true)
	                                          .with_thread_names(true)
	                                          .with_filter(tracing::level_filters::LevelFilter::from(Level::TRACE));


	tracing_subscriber::registry().with(console_layer)
	                              .with(fmt)
	                              .init();
}


#[tokio::main]
async fn main() -> miette::Result<()> {
	#[cfg(feature = "tracing")]
	enable_tracing();
	#[cfg(not(feature = "tracing"))]
	{
		// std::env::set_var("RUST_LOG", "trace,nusb=warn,mio_serial=info,mio=info");
		std::env::set_var("RUST_LOG", "trace");
		env_logger::Builder::from_env(env_logger::Env::default()).format_indent(Some(3))
		                                                         .format_module_path(false)
		                                                         .format_target(true)
		                                                         .format_timestamp(None)
		                                                         .init();
	}

	let cfg = cli::parse();


	debug!("cmd: {:?}", cfg.cmd);

	match cfg.cmd {
		cli::Command::List { kind } => list(cfg.format, kind).await,
		cli::Command::Read(query) => read(query).await,
		cli::Command::Mount { query, wait } => mount(query, wait, cfg.format).await,
		cli::Command::Unmount { query, wait } => unmount(query, wait, cfg.format).await,
		cli::Command::Install(cli::Install { pdx, query, force }) => install(query, pdx, force, cfg.format).await,
		cli::Command::Run(cli::Run { destination:
		                                cli::Destination::Device(cli::DeviceDestination { install:
		                                                                                     cli::Install { pdx,
		                                                                                                    query,
		                                                                                                    force, },
		                                                                                  no_install,
		                                                                                  no_read, }), }) => {
			run_dev(query, pdx, no_install, no_read, force, cfg.format).await
		},
		cli::Command::Run(cli::Run { destination:
		                                cli::Destination::Simulator(cli::SimDestination { pdx, sdk }), }) => {
			run_sim(pdx, sdk, cfg.format).await
		},
		cli::Command::Send(cli::Send { command, query, read }) => send(query, command, read, cfg.format).await,

		cli::Command::Debug => {
			use mount::volume::volumes_for_map;

			volumes_for_map(usb::discover::devices()?).await?
			                                          .into_iter()
			                                          .map(|(dev, vol)| (dev, vol.map(|v| v.path().to_path_buf())))
			                                          .for_each(|(mut dev, path)| {
				                                          dev.debug_inspect();
				                                          println!("vol: {path:?}");
			                                          });
			Ok(())
		},
	}.into_diagnostic()
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn run_dev(query: DeviceQuery,
                 pdx: PathBuf,
                 no_install: bool,
                 no_read: bool,
                 force: bool,
                 format: cli::Format)
                 -> Result<(), error::Error> {
	let devs = run::run_on_device(query, pdx, no_install, no_read, force).await?
	                                                                     .into_iter()
	                                                                     .enumerate();
	if matches!(format, cli::Format::Json) {
		print!("[");
	}

	for (i, dev) in devs {
		let repr = dev.as_report_short();
		match format {
			cli::Format::Human => println!("{}", repr.to_printable_line()),
			cli::Format::Json => {
				if i > 0 {
					println!(",");
				}
				serde_json::to_string(&repr).map(|s| println!("{s},"))
				                            .map_err(|err| error!("{err}"))
				                            .ok();
			},
		}
	}

	if matches!(format, cli::Format::Json) {
		println!("]");
	}

	Ok(())
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn run_sim(pdx: PathBuf, sdk: Option<PathBuf>, _format: cli::Format) -> Result<(), error::Error> {
	run::run_with_sim(pdx, sdk).await
	                           .inspect(|_| trace!("sim execution is done"))
}


/// `mount_and_install` with report.
#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn install(query: DeviceQuery,
                 path: PathBuf,
                 force: bool,
                 format: cli::Format)
                 -> Result<(), error::Error> {
	if matches!(format, cli::Format::Json) {
		print!("[");
	}
	install::mount_and_install(query, &path, force).await?
	                                               .filter_map(|res| {
		                                               async { res.map_err(|err| error!("{err}")).ok() }
	                                               })
	                                               .enumerate()
	                                               .for_each_concurrent(4, |(i, installed)| {
		                                               let (drive, installed) = installed.into_parts();
		                                               trace!("installed: {installed}");
		                                               async move {
			                                               let repr = drive.as_report_short();
			                                               match format {
				                                               cli::Format::Human => {
				                                                  println!("{}", repr.to_printable_line())
			                                                  },
			                                                  cli::Format::Json => {
				                                                  if i > 0 {
					                                                  println!(",");
				                                                  }
				                                                  serde_json::to_string(&repr).map(|s| println!("{s},"))
				                                                                       .map_err(|err| error!("{err}"))
				                                                                       .ok();
			                                                  },
			                                               }
		                                               }
	                                               })
	                                               .await;
	if matches!(format, cli::Format::Json) {
		println!("]");
	}
	Ok(())
}


/// [[mount::mount_and]] with report.
#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn mount(query: DeviceQuery, wait: bool, format: cli::Format) -> Result<(), error::Error> {
	if matches!(format, cli::Format::Json) {
		print!("[");
	}
	mount::mount_and(query, wait).await?
	                             .enumerate()
	                             .for_each_concurrent(4, |(i, res)| {
		                             async move {
			                             match res {
				                             Ok(drive) => {
				                                let repr = drive.as_report_short();
				                                match format {
					                                cli::Format::Human => println!("{}", repr.to_printable_line()),
				                                   cli::Format::Json => {
					                                   if i > 0 {
						                                   println!(",");
					                                   }
					                                   serde_json::to_string(&repr).map(|s| println!("{s},"))
					                                                               .map_err(|err| error!("{err}"))
					                                                               .ok();
				                                   },
				                                }
			                                },
			                                Err(err) => error!("{err}"),
			                             }
		                             }
	                             })
	                             .await;
	if matches!(format, cli::Format::Json) {
		println!("]");
	}
	Ok(())
}


/// [[mount::unmount_and]] with report.
#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn unmount(query: DeviceQuery, wait: bool, format: cli::Format) -> Result<(), error::Error> {
	let results: Vec<_> = mount::unmount_and(query, wait).await?.collect().await;
	for (i, res) in results.into_iter().enumerate() {
		match res {
			Ok(dev) => {
				let repr = dev.as_report_short();
				match format {
					cli::Format::Human => println!("{}", repr.to_printable_line()),
					cli::Format::Json => {
						if i > 0 {
							println!(",");
						}
						serde_json::to_string(&repr).map(|s| println!("{s},"))
						                            .map_err(|err| error!("{err}"))
						                            .ok();
					},
				};
			},
			Err(err) => error!("{err}"),
		}
	}
	Ok(())
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn send(query: DeviceQuery,
              command: device::command::Command,
              read: bool,
              _format: cli::Format)
              -> Result<(), error::Error> {
	let senders = send::send_to_interfaces(query, command).await?;

	senders.for_each_concurrent(None, |res| {
		       async move {
			       if read {
				       match res {
					       Ok(mut interface) => usb::io::redirect_interface_to_stdout(&mut interface)
							                      .inspect_ok(|_| trace!("Read interface done.")).await,
				          Err(err) => Err(err),
				       }
			       } else {
				       res.map(|_| ())
			       }
		       }.inspect_err(|err| error!("{err}"))
		       .map(|_| ())
	       })
	       .await;
	Ok(())
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn read(query: DeviceQuery) -> Result<(), error::Error> {
	let by_dev = |mut device: device::Device| -> Result<_, Error> {
		device.info().serial_number().map(|s| trace!("reading {s}"));
		let fut = async move { usb::io::redirect_to_stdout(&mut device).await };
		Ok(fut)
	};

	let by_sn = |sn: SerialNumber| -> Result<_, Error> {
		let device = usb::discover::device(&sn)?;
		by_dev(device)
	};


	let by_port = |port: String| -> Result<_, Error> {
		let fut = async move {
			if let Err(err) = serial::dev_with_port(&port).map_ok(by_dev).await??.await {
				warn!("Unable to map specified port {port} to device: {err}");
				serial::redirect_to_stdout(port).await?;
			}
			Ok(())
		};
		Ok(fut)
	};

	match query.value {
		Some(DeviceQueryValue::Serial(sn)) => by_sn(sn)?.await,
		Some(DeviceQueryValue::Path(port)) => by_port(port.to_string_lossy().to_string())?.await,
		Some(DeviceQueryValue::Com(port)) => by_port(format!("COM{port}"))?.await,
		None => {
			let mut devices: Vec<_> = usb::discover::devices_data()?.collect();
			match devices.len() {
				1 => by_dev(devices.remove(0))?.await,
				0 => Err(Error::not_found()),
				len => {
					error!("Read multiple devices not supported, plz connect exact one or specify its serial number. Found {len} devices.");
					Err(Error::not_found())
				},
			}
		},
	}
}


#[cfg_attr(feature = "tracing", tracing::instrument())]
async fn list(format: cli::Format, kind: cli::DeviceKind) -> Result<(), error::Error> {
	use mount::volume::volumes_for_map;

	let devices = match kind {
		              cli::DeviceKind::Any => volumes_for_map(usb::discover::devices()?).await?,
	                 cli::DeviceKind::Storage => volumes_for_map(usb::discover::devices_storage()?).await?,
	                 cli::DeviceKind::Data => usb::discover::devices_data()?.map(|dev| (dev, None)).collect(),
	              }.into_iter()
	              .map(|(dev, vol)| (dev, vol.map(|v| v.path().to_path_buf())));

	match format {
		cli::Format::Human => {
			for (mut dev, vol) in devices {
				if !dev.is_ready() {
					dev.open().ok();
				}

				let vol = vol.map(|v| v.into());
				let repr = report::DevInfo::new(&dev, vol);
				println!("{}", repr.to_printable_line());
				dev.close();
			}
		},
		cli::Format::Json => {
			print!("[");
			let devices: Vec<_> = devices.collect();
			let len = devices.len();
			for (i, (mut dev, vol)) in devices.into_iter().enumerate() {
				if !dev.is_ready() {
					dev.open().ok();
				}

				let vol = vol.map(|v| v.into());
				let repr = report::DevInfo::new(&dev, vol);
				let repr = serde_json::to_string(&repr)?;
				println!("{repr}");
				dev.close();

				if i != len - 1 {
					print!(", ");
				}
			}
			println!("]");
		},
	}

	Ok(())
}
