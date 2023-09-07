use super::DeviceQuery;


/// Read the device or simulator output.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "cli", derive(clap::Parser))]
#[cfg_attr(feature = "cli", command(author, version, about, long_about = None, name = "read"))]
pub struct Read {
	#[cfg_attr(feature = "cli", command(flatten))]
	pub device: DeviceQuery,

	#[arg(long)]
	/// Set 'echo' mode for external shell.
	pub echo: Option<bool>,
}
