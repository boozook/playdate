use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use simulator::utils::consts::SDK_ENV_VAR;

use crate::device::query::Query;


pub fn parse() -> Cfg { Cfg::parse() }


#[derive(Parser, Debug)]
#[command(author, version, about, name = "pdtool")]
pub struct Cfg {
	#[command(subcommand)]
	pub cmd: Command,

	/// Standard output format.
	#[clap(long, global = true, default_value_t = Format::Human)]
	pub format: Format,
}


#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum Format {
	Human,
	Json,
}

impl std::fmt::Display for Format {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Format::Human => "human".fmt(f),
			Format::Json => "json".fmt(f),
		}
	}
}


#[derive(Subcommand, Debug)]
pub enum Command {
	/// Print list of connected active Playdate devices.
	List {
		#[arg(default_value_t = DeviceKind::Any)]
		kind: DeviceKind,
	},

	/// Mount a Playdate device if specified, otherwise mount all Playdates as possible.
	Mount {
		#[command(flatten)]
		query: Query,
		/// Wait for availability of mounted device's filesystem.
		#[arg(long, default_value_t = false)]
		wait: bool,
	},

	/// Unmount a Playdate device if specified, otherwise unmount all mounted Playdates.
	Unmount {
		/// Device spec
		#[command(flatten)]
		query: Query,
		/// Wait for device to be connected after unmounted.
		#[arg(long, default_value_t = false)]
		wait: bool,
	},

	/// Install given package to device if specified, otherwise use all devices as possible.
	///
	/// Workflow: switch to storage mode and mount if needed, write files, unmount if requested.
	Install(#[command(flatten)] Install),

	/// Install and run given package on the specified device or simulator.
	Run(#[command(flatten)] run::Run),

	/// Connect to device and proxy output to stdout.
	Read(#[command(flatten)] Query),

	/// Send command to specified device.
	// #[command(hide = true)]
	Send(#[command(flatten)] Send),

	/// Debug functions, only for development purposes.
	#[cfg(debug_assertions)]
	Debug(#[command(flatten)] Dbg),
}


#[derive(Clone, Debug, clap::Parser)]
pub struct Dbg {
	/// Command to send:
	#[clap(subcommand)]
	pub cmd: DbgCmd,

	/// Device selector.
	#[command(flatten)]
	pub query: Query,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum DbgCmd {
	/// Inspect device(s) state.
	Inspect,
	/// Probe powershell
	Probe,
	/// Retrieve sn of dev that behind the mounted volume.
	VolSn1 { vol: char },
	/// Retrieve sn of dev that behind the mounted volume.
	VolSn2 { vol: char },
	/// Eject device by mounted volume letter.
	Eject { vol: char },
}


#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum DeviceKind {
	Any,
	Data,
	Storage,
}

impl ToString for DeviceKind {
	fn to_string(&self) -> String {
		match self {
			DeviceKind::Any => "any",
			DeviceKind::Data => "data",
			DeviceKind::Storage => "storage",
		}.to_owned()
	}
}


#[derive(Clone, Debug, clap::Parser)]
#[command(author, version, about, long_about = None, name = "install")]
pub struct Install {
	/// Path to the PDX package.
	#[arg(value_name = "PACKAGE")]
	pub pdx: PathBuf,

	/// Allow to overwrite existing files.
	#[arg(long, default_value_t = false)]
	pub force: bool,

	#[command(flatten)]
	pub query: Query,
}


#[derive(Clone, Debug, clap::Parser)]
#[command(author, version, about, long_about = None, name = "send")]
pub struct Send {
	/// Command to send:
	#[clap(subcommand)]
	pub command: device::device::command::Command,

	/// Device selector.
	#[command(flatten)]
	pub query: Query,

	/// Read output from device after sending command.
	#[arg(long, default_value_t = false)]
	pub read: bool,
}


pub use run::*;
mod run {
	use std::borrow::Cow;

	use super::*;


	#[derive(Clone, Debug, clap::Parser)]
	#[command(author, version, about, long_about = None, name = "run")]
	pub struct Run {
		#[clap(subcommand)]
		pub destination: Destination,
	}


	#[derive(Clone, Debug, clap::Subcommand)]
	pub enum Destination {
		/// Install to the device.
		///
		/// Attention: <PACKAGE> parameter is a local path to pdx-package.
		/// But in case of '--no-install' given path will be interpreted as on-device relative to it's root path,
		/// e.g. "/Games/my-game.pdx".
		#[clap(visible_alias("dev"))]
		Device(Dev),

		/// Run with simulator.
		/// Playdate required to be installed.
		#[clap(visible_alias("sim"))]
		Simulator(Sim),
	}

	impl std::fmt::Display for Destination {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			let name: Cow<str> = match self {
				Destination::Device(Dev { install: Install { query, .. },
				                          .. }) => format!("device:{query}").into(),
				Destination::Simulator(_) => "simulator".into(),
			};
			name.fmt(f)
		}
	}


	#[derive(Clone, Debug, clap::Parser)]
	/// Simulator destination
	pub struct Sim {
		/// Path to the PDX package.
		#[arg(value_name = "PACKAGE")]
		pub pdx: PathBuf,

		/// Path to Playdate SDK
		#[arg(long, env = SDK_ENV_VAR, value_name = "DIRECTORY", value_hint = clap::ValueHint::DirPath)]
		pub sdk: Option<PathBuf>,
	}


	#[derive(Clone, Debug, clap::Parser)]
	/// Hardware destination
	pub struct Dev {
		#[command(flatten)]
		pub install: super::Install,

		/// Do not install pdx to the device.
		/// If set, <PACKAGE> path will be interpreted as on-device path of already installed package,
		/// relative to the root of device's fs partition.
		#[arg(long, name = "no-install", default_value_t = false)]
		pub no_install: bool,

		/// Do not wait & read the device's output after execution.
		/// Exits immediately after send 'run' command.
		#[arg(long, name = "no-read", default_value_t = false)]
		pub no_read: bool,
	}
}
