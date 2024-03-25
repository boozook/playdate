use std::backtrace::Backtrace;
use thiserror::Error;
use miette::Diagnostic;
use crate::usb::mode::Mode;


#[derive(Error, Debug, Diagnostic)]
pub enum Error {
	#[error(transparent)]
	#[diagnostic(code(my_lib::io_error))]
	Io {
		#[backtrace]
		#[from]
		source: std::io::Error,
	},

	#[error(transparent)]
	#[diagnostic()]
	Process {
		#[backtrace]
		#[from]
		source: std::process::ExitStatusError,
	},

	#[error(transparent)]
	#[diagnostic()]
	Transfer {
		#[backtrace]
		#[from]
		source: nusb::transfer::TransferError,
	},

	#[error(transparent)]
	#[diagnostic()]
	Borrow {
		#[backtrace]
		#[from]
		source: std::cell::BorrowMutError,
	},

	// #[error(transparent)]
	// UsbDrive {
	// 	#[backtrace]
	// 	#[from]
	// 	source: usbenum::error::Error,
	// },
	#[error("Awaiting device timeout `{device}`.")]
	#[diagnostic()]
	DeviceTimeout {
		#[backtrace]
		backtrace: Backtrace,
		device: crate::device::Device,
	},

	#[error("Awaiting {what} timeout.")]
	Timeout {
		#[backtrace]
		backtrace: Backtrace,
		what: String,
	},

	#[error(transparent)]
	#[diagnostic()]
	Utf {
		#[backtrace]
		#[from]
		source: std::str::Utf8Error,
	},

	#[error(transparent)]
	Json {
		#[backtrace]
		#[from]
		source: serde_json::Error,
	},

	#[cfg(target_os = "macos")]
	#[error(transparent)]
	Plist {
		#[backtrace]
		#[from]
		source: plist::Error,
	},

	#[cfg(target_os = "linux")]
	#[error(transparent)]
	Lfs {
		#[backtrace]
		#[from]
		source: lfs_core::Error,
	},

	#[cfg(target_os = "windows")]
	#[error(transparent)]
	WinApi {
		#[backtrace]
		#[from]
		source: windows::core::Error,
	},

	#[error("Chain of errors ending with: {source}")]
	#[diagnostic()]
	Chain {
		#[backtrace]
		#[diagnostic(transparent)]
		source: Box<Error>,
		#[related]
		#[diagnostic(transparent)]
		others: Vec<Error>,
	},

	#[error(transparent)]
	#[diagnostic(transparent)]
	DeviceSerial {
		#[backtrace]
		#[from]
		source: crate::device::serial::DeviceSerialFormatError,
	},

	#[error(transparent)]
	#[diagnostic()]
	SerialPort {
		#[backtrace]
		#[from]
		source: serialport::Error,
	},

	#[diagnostic()]
	#[error("Device not found.")]
	/// Device discovery error.
	NotFound(#[backtrace] Backtrace),

	#[diagnostic()]
	#[error("Interface not ready.")]
	/// Interface error.
	NotReady(#[backtrace] Backtrace),

	#[error("Device in the wrong state `{0:?}`.")]
	WrongState(Mode),

	#[error("Mount point not found for {0}.")]
	MountNotFound(String),
	// #[error("data store disconnected")]
	// Disconnect(#[from] io::Error),
	// #[error("the data for key `{0}` is not available")]
	// Redaction(String),
	// #[error("invalid header (expected {expected:?}, found {found:?})")]
	// InvalidHeader { expected: String, found: String },
	// #[error("unknown error")]
	// Unknown,
}


impl Error {
	#[track_caller]
	pub fn usb_timeout(device: crate::device::Device) -> Self {
		Self::DeviceTimeout { device,
		                      backtrace: Backtrace::capture() }
	}

	#[track_caller]
	pub fn timeout<S: ToString>(what: S) -> Self {
		Self::Timeout { what: what.to_string(),
		                backtrace: Backtrace::capture() }
	}

	#[track_caller]
	pub fn not_found() -> Self { Self::NotFound(Backtrace::capture()) }
	#[track_caller]
	pub fn not_ready() -> Self { Self::NotReady(Backtrace::capture()) }

	#[track_caller]
	pub fn chain<I, A, B>(err: A, others: I) -> Self
		where I: IntoIterator<Item = B>,
		      A: Into<Error>,
		      B: Into<Error> {
		Self::Chain { source: Box::new(err.into()),
		              others: others.into_iter().map(Into::into).collect() }
	}
}


unsafe impl Sync for Error {}
