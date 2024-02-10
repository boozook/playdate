#![cfg(not(test))]
#![cfg(feature = "panic-handler")]
//! Global Panic Handler implementation. Depends on `panic-handler` feature.

use core::panic::PanicInfo;
use core::fmt::Write;
use arrayvec::ArrayString;
use super::proc::error;


#[panic_handler]
fn panic(#[allow(unused)] panic_info: &PanicInfo) -> ! {
	let mut output = ArrayString::<1024>::new();
	let payload = if let Some(payload) = panic_info.payload().downcast_ref::<&str>() {
		payload
	} else {
		""
	};

	let location = panic_info.location();
	let (module, line) = if let Some(location) = location {
		(location.file(), location.line())
	} else {
		("", 0)
	};

	let _ = if let Some(message) = panic_info.message() {
		write!(output, "PANIC: [{module}@{line}] \"{message}\" {payload}\0")
	} else {
		write!(output, "PANIC: [{module}@{line}] {payload}\0")
	};
	error(output.as_str());
}
