#![cfg(not(test))]
#![cfg(feature = "panic-handler")]
//! Global Panic Handler implementation. Depends on `panic-handler` feature.

use core::panic::PanicInfo;
use core::fmt::Write;
use arrayvec::ArrayString;
use super::proc::error;


#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
	let mut output = ArrayString::<1024>::new();
	let _ = write!(output, "{panic_info}\0");
	error(output.as_str());
}
