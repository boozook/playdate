use std::borrow::Cow;
use std::fmt::Display;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;


#[derive(Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Subcommand))]
#[cfg_attr(feature = "clap", command(name = "COMMAND"))]
pub enum Command {
	/// Run custom pdx.
	Run {
		/// On-device path to the PDX package,
		/// e.g. `/Games/MyGame.pdx` or `/System/Settings.pdx`
		path: String,
	},

	/// Run system built-in pdx,
	#[cfg_attr(feature = "clap", command(name = "run-sys"))]
	RunSystem {
		/// System built-in application,
		#[cfg_attr(feature = "clap", arg(value_name = "NAME"))]
		path: SystemPath,
	},

	/// Reboot into data segment USB disk
	Datadisk,

	/// Hibernate, semi-deep sleep mode.
	#[cfg_attr(feature = "clap", command(visible_alias = "sleep"))]
	Hibernate,

	/// Turn console echo on or off.
	Echo {
		// #[cfg_attr(feature = "clap", arg(default_value_t = true))]
		// value: bool,
		#[cfg_attr(feature = "clap", arg(default_value_t = Switch::On))]
		value: Switch,
	},

	/// Request the device serial number.
	#[cfg_attr(feature = "clap", command(visible_alias = "sn"))]
	SerialNumber,

	/// Request the device version info.
	#[cfg_attr(feature = "clap", command(visible_alias = "V"))]
	Version,

	/// Simulate a button press.
	///
	/// +a/-a/a for down/up/both
	#[cfg_attr(feature = "clap", command(visible_alias = "btn"))]
	Button {
		/// Button to press or release.
		#[cfg_attr(feature = "clap", clap(subcommand))]
		button: Button,
	},

	/// Send a message to a message handler in the current running program.
	#[cfg_attr(feature = "clap", command(visible_alias = "msg"))]
	Message {
		/// Message to send.
		message: String,
	},

	/// Send custom command.
	#[cfg_attr(feature = "clap", command(visible_alias = "!"))]
	Custom {
		/// Command to send.
		cmd: String,
	},
}


#[derive(Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
pub enum PdxPath {
	System { path: SystemPath },
	User { path: PathBuf },
}


#[derive(Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
pub enum SystemPath {
	/// Launcher application, Home.
	///
	/// `/System/Launcher.pdx`.
	Launcher,
	/// Settings application.
	///
	/// `/System/Settings.pdx`.
	Settings,
	/// Playdate Catalog application.
	///
	/// `/System/Catalog.pdx`.
	Catalog,
}

impl SystemPath {
	pub fn as_path(&self) -> &Path {
		match self {
			Self::Launcher => Path::new("/System/Launcher.pdx"),
			Self::Settings => Path::new("/System/Settings.pdx"),
			Self::Catalog => Path::new("/System/Catalog.pdx"),
		}
	}
}


impl Command {
	pub fn as_str(&self) -> Cow<'_, str> {
		match self {
			Command::Run { path } => format!("run {path}").into(),
			Command::RunSystem { path } => format!("run {}", path.as_path().display()).into(),
			Command::Datadisk => "datadisk".into(),
			Command::Hibernate => "hibernate".into(),
			Command::Echo { value: Switch::On } => "echo on".into(),
			Command::Echo { value: Switch::Off } => "echo off".into(),
			Command::SerialNumber => "serialread".into(),
			Command::Version => "version".into(),
			Command::Button { button } => format!("btn {}", button.as_btn_str()).into(),
			Command::Message { message } => format!("msg {message}").into(),
			Command::Custom { cmd } => format!("{cmd}").into(),
		}
	}
}


impl Display for Command {
	#[inline]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.as_str().fmt(f) }
}

impl Command {
	pub fn with_break(&self) -> String {
		let cmd = self.as_str();
		let mut line = String::with_capacity(cmd.len() + 2);
		line.push('\n'); // extra break to ensure that the command starts from new line.
		line.push_str(&cmd);
		line.push('\n');
		line
	}

	pub fn with_break_to<W: Write>(&self, mut writer: W) -> std::io::Result<()> { writeln!(writer, "{self}") }
}


#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "clap", clap(name = "BOOL"))]
pub enum Switch {
	/// Turn echo on.
	/// [aliases: true]
	#[cfg_attr(feature = "clap", value(alias = "true"))]
	On,
	/// Turn echo off.
	/// [aliases: false]
	#[cfg_attr(feature = "clap", value(alias = "false"))]
	Off,
}

impl From<bool> for Switch {
	fn from(value: bool) -> Self { if value { Switch::On } else { Switch::Off } }
}

impl Into<bool> for Switch {
	fn into(self) -> bool {
		match self {
			Switch::On => true,
			Switch::Off => false,
		}
	}
}

impl std::fmt::Display for Switch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let value = match self {
			Self::On => "on",
			Self::Off => "off",
		};
		write!(f, "{value}")
	}
}


#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "clap", derive(clap::Subcommand))]
#[cfg_attr(feature = "clap", command(name = "BTN"))]
pub enum Button {
	A {
		#[cfg_attr(feature = "clap", arg(required = false, default_value_t = ButtonAction::Both))]
		action: ButtonAction,
	},
	B {
		#[cfg_attr(feature = "clap", arg(required = false, default_value_t = ButtonAction::Both))]
		action: ButtonAction,
	},
}

impl std::fmt::Display for Button {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Button::A { action } => write!(f, "{action}a"),
			Button::B { action } => write!(f, "{action}b"),
		}
	}
}

impl Button {
	pub fn as_btn_str(&self) -> String {
		match self {
			Button::A { action } => format!("{}a", action.as_btn_prefix()),
			Button::B { action } => format!("{}b", action.as_btn_prefix()),
		}
	}
}


#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "clap", derive(clap::ValueEnum))]
#[cfg_attr(feature = "clap", clap(name = "BTN"))]
pub enum ButtonAction {
	#[cfg_attr(feature = "clap", value(alias = "-"))]
	Down,
	#[cfg_attr(feature = "clap", value(alias = "+"))]
	Up,
	#[cfg_attr(feature = "clap", value(alias = "+-"), value(alias = "±"))]
	Both,
}

impl Default for ButtonAction {
	fn default() -> Self { Self::Both }
}

impl std::fmt::Display for ButtonAction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let value = match self {
			Self::Down => "-",
			Self::Up => "+",
			Self::Both => "±",
		};
		write!(f, "{value}")
	}
}

impl ButtonAction {
	pub fn as_btn_prefix(&self) -> &'static str {
		match self {
			Self::Down => "+",
			Self::Up => "-",
			Self::Both => "",
		}
	}
}
