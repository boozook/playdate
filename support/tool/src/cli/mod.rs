use clap::{Parser, Subcommand};

mod commands;
pub use commands::*;


pub fn parse() -> Cfg { Cfg::parse() }


#[derive(Parser, Debug)]
#[command(author, version, about, name = "pdtool")]
pub struct Cfg {
	// #[command(flatten)]
	// device: DeviceQuery,
	#[command(subcommand)]
	pub command: Command,
}


#[derive(Subcommand, Debug)]
pub enum Command {
	/// Print list of connected active Playdate devices.
	List,

	Mount(mount::Mount),

	Install(install::Install),

	Run(run::Run),

	/// Connect to device and proxy output to stdout.
	Read(read::Read),
}
