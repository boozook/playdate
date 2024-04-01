use std::fmt::Display;
use log::LevelFilter;
use env_logger::{Builder, Env};

use crate::config::Config;


pub fn init(verbose: u32) -> anyhow::Result<()> {
	let var = concat!(env!("CARGO_BIN_NAME"), "_LOG").to_uppercase()
	                                                 .replace("-", "_");
	let style = format!("{var}_STYLE");

	let env = Env::new().filter(var).write_style(style);
	let mut builder = Builder::new();
	builder.filter_level(LevelFilter::Off);
	if verbose < 3 {
		builder.filter(Some("cargo:"), LevelFilter::Off);
	}
	builder.parse_env(env);
	builder.format_timestamp(None);
	builder.format_indent(Some(12));
	builder.try_init()?;
	Ok(())
}


pub trait LogErr<T, E> {
	fn log_err(self) -> Self;
	fn log_err_cargo(self, config: &Config) -> Self;
}


impl<T, E: Display + std::fmt::Debug> LogErr<T, E> for Result<T, E> {
	fn log_err(self) -> Self { self.inspect_err(|err| ::log::error!("{err}")) }

	fn log_err_cargo(self, config: &Config) -> Self {
		self.inspect_err(|err| {
			    config.log().error(&err);
			    ::log::error!("{err}");
		    })
	}
}
