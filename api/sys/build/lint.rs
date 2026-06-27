use format_args as fmt;
use bindgen_cfg::{BIN_NAME, Bin};

use crate::cargo::warn;

pub static PKG_NAME: &str = env!("CARGO_PKG_NAME");


pub fn check_bindgen_unnecessary_inner() {
	if cfg!(feature = "bindgen") {
		warn(fmt!("Playdate bindgen has been built as dependency of the {PKG_NAME} by enabled feature 'bindgen'. You might want to disable that feature to significantly decrease build time. It's not necessary because the bindings for the SDK you're using are already bundled."));
	}
}

pub fn check_bindgen_inner_and_external(bin: &Bin) {
	use bindgen_cfg::Runner;

	let bin = bin.to_owned();
	let join = std::thread::spawn(move || {
		if let Some((_, ver)) = Runner::find_tool(&bin) {
			warn(fmt!("Playdate bindgen (v{ver}) found but also built as dependency of the {PKG_NAME} by enabled feature 'bindgen'. You might want to disable that feature to significantly decrease build time."));
		}
	});

	std::mem::forget(join);
}

#[track_caller]
pub fn panic_recover_hints_no_builtin(sdk: &str) -> ! {
	panic!(
	       "Builtin bindings for SDK {sdk} that covers requested feature-set not found. {}\t{}\t{}\t{}",
	       fmt!("Possible options to solve it:"),
	       fmt!("- update '{PKG_NAME}' crate with `cargo update`"),
	       fmt!("- get '{BIN_NAME}' with `cargo install playdate-bindgen`"),
	       fmt!("- enable 'bindgen' feature for the '{PKG_NAME}' crate.")
	);
}
