use std::borrow::Cow;
use std::cell::RefMut;
use std::fmt::Display;
use std::ops::DerefMut;

use cargo::core::Shell;
use cargo::util::machine_message::Message;
use anstyle::AnsiColor as Color;

use crate::config::Config;
use crate::logger::LogErr;
use crate::proc::reader::format::Artifact;
use crate::proc::reader::format::CargoMessage;
use crate::proc::reader::format::CompilerMessage;


pub struct CargoLogger<T: DerefMut<Target = Shell>>(T, bool);

// impl CargoLogger<'_> {
impl<S: DerefMut<Target = Shell>> CargoLogger<S> {
	/// Shortcut to right-align and color green a status message.
	pub fn status<T, U>(&mut self, status: T, message: U)
		where T: Display,
		      U: Display {
		self.0.status(status, message).log_err().ok();
	}

	/// Shortcut to right-align a status message.
	pub fn status_with_color<T, U>(&mut self, status: T, message: U, color: Color)
		where T: Display,
		      U: Display {
		self.0
		    .status_with_color(status, message, &color.on_default())
		    .log_err()
		    .ok();
	}

	/// Prints a red 'error' status message.
	pub fn status_err<U>(&mut self, message: U)
		where U: Display {
		if self.1 {
			let msg = CargoMessage::CompilerMessage { message: CompilerMessage { rendered: message.to_string(),
			                                                                     level: "error".to_owned(),
			                                                                     code: None,
			                                                                     spans: vec![] } };
			self.0.print_json(&msg)
		} else {
			self.0
			    .status_with_color("Error", message, &Color::Red.on_default())
		}.log_err()
		.ok();
	}

	/// Runs the callback only if we are in verbose mode.
	pub fn verbose<F>(&mut self, mut callback: F)
		where F: FnMut(CargoLogger<&mut Shell>) {
		self.0
		    .verbose(|shell| {
			    callback(CargoLogger(shell, self.1));
			    Ok(())
		    })
		    .log_err()
		    .ok();
	}

	/// Prints a red 'error' message.
	pub fn error<T: Display>(&mut self, message: T) {
		if self.1 {
			let msg = CargoMessage::CompilerMessage { message: CompilerMessage { rendered: message.to_string(),
			                                                                     level: "error".to_owned(),
			                                                                     code: None,
			                                                                     spans: vec![] } };
			self.0.print_json(&msg)
		} else {
			self.0.error(message)
		}.log_err()
		.ok();
	}

	/// Prints an amber 'warning' message.
	pub fn warn<T: Display>(&mut self, message: T) {
		if self.1 {
			let msg = CargoMessage::CompilerMessage { message: CompilerMessage { rendered: message.to_string(),
			                                                                     level: "warning".to_owned(),
			                                                                     code: None,
			                                                                     spans: vec![] } };
			self.0.print_json(&msg)
		} else {
			self.0.warn(message)
		}.log_err()
		.ok();
	}

	/// Prints a cyan 'note' message.
	pub fn note<T: Display>(&mut self, message: T) { self.0.note(message).log_err().ok(); }

	// pub fn print_json<T: Message>(&mut self, message: T)  {
	// 	if self.1 { self.0.print_json(&message) } else { Ok(()) }.log_err().ok();
	// 	self
	// }

	pub fn print_cargo_message<T: Message>(&mut self, message: T) {
		if self.1 {
			self.0
			    .out()
			    .write_all(message.to_json_string().as_bytes())
			    .log_err()
			    .ok();
		}
	}

	pub fn build_finished<T>(&mut self, success: bool, message: Option<T>)
		where T: Display {
		if self.1 {
			self.print_cargo_message(&CargoMessage::BuildFinished { success })
		} else {
			let msg: Cow<str> = message.map(|s| s.to_string().into()).unwrap_or("build".into());
			self.status("Finished", msg)
		}
	}

	pub fn compiler_artifact(&mut self, artifact: Artifact) { self.print_cargo_message(artifact) }
}


impl Config<'_> {
	#[must_use]
	pub fn log(&self) -> CargoLogger<RefMut<'_, Shell>> {
		CargoLogger(
		            self.workspace.gctx().shell(),
		            self.compile_options.build_config.emit_json(),
		)
	}

	pub fn log_extra_verbose<F>(&self, mut callback: F)
		where F: FnMut(CargoLogger<RefMut<'_, Shell>>) {
		if self.workspace.gctx().extra_verbose() {
			callback(self.log())
		}
	}
}
