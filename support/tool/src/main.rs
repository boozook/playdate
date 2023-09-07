#![feature(exit_status_error)]
#![cfg(feature = "cli")]


#[macro_use]
extern crate log;
extern crate playdate_tool as tool;

use tool::{Error, cli, search};
use tool::cli::{install, mount, run};
use tool::cli::find_one_device;


fn main() -> Result<(), Error> {
	let cfg = cli::parse();
	env_logger::init();
	trace!("input: {cfg:#?}");

	start(cfg).map_err(|err| {
		          error!("{err}");
		          err
	          })
}

fn start(cfg: cli::Cfg) -> Result<(), Error> {
	match cfg.command {
		cli::Command::List => {
			search::list_connected_devices()?.into_iter()
			                                 .for_each(|device| println!("{device:?}"));
		},

		cli::Command::Mount(cfg) => mount::mount(&cfg)?.unmount_on_drop(false),

		cli::Command::Install(cfg) => {
			let mut path = install::install(&cfg)?;
			path.device.refresh().ok();
			info!(
			      "Installed to {}, on-device path: {}",
			      path.device,
			      path.path.display()
			);
		},

		cli::Command::Run(cfg) => run::run(cfg)?,

		cli::Command::Read(cfg) => {
			let device = find_one_device(cfg.device)?;
			debug!("device: {device:#?}");
			device.read_to_stdout(cfg.echo)?;
		},
	}

	Ok(())
}
