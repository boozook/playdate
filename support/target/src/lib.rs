#![cfg_attr(unix, feature(substr_range))]

mod built;

mod kind;
pub use kind::TargetKind;


pub mod spec {
	pub mod build;

	/// Crate and spec version.
	pub const VERSION: &str = env!("CARGO_PKG_VERSION");

	mod src {
		#![cfg(any(feature = "spec", all(feature = "serde_json", feature = "toml")))]

		/// Source text of target specification in json format for rustc.
		pub const SPEC_JSON: &str = include_str!(env!("PD_SPEC_PATH"));

		/// Canonical filename of the [target specification](SPEC_JSON).
		pub use crate::built::FILENAME as SPEC_FILENAME_JSON;

		/// Source text of target specification in toml format, respects the schema.
		pub const SPEC_TOML: &str = include_str!(env!("PD_SPECSRC_PATH"));

		/// Source text of target specification in toml format (pretty-printed), respects the schema.
		#[cfg(feature = "pretty")]
		pub const SPEC_TOML_PRETTY: &str = include_str!(env!("PD_SPECSRC_PRETTY_PATH"));
	}
	#[cfg(any(feature = "spec", all(feature = "serde_json", feature = "toml")))]
	pub use src::*;

	/// Source code of linker-script used in target specification,
	pub const LINKER_SCRIPT: &str = include_str!("spec/link-map.ld");

	/// Canonical shortname of base rustc/llvm' target.
	pub const LLVM_TARGET_HF: &str = "thumbv7em-none-eabihf";
	pub const LLVM_TARGET_SF: &str = "thumbv7em-none-eabi";


	/// Name of the builtin custom spec.
	pub const TARGET_SHORTNAME: &str = "thumbv7em-playdate-eabihf";
	/// Alternative shortnames of the target spec.
	/// _May be important for some shitty things like build-scripts which parses cargo's env `TARGET` to guess and make decision about target._
	pub const TARGET_SHORTNAME_ALT: &[&str] = &[
	                                            TARGET_SHORTNAME,
	                                            "thumbv7em-panic-playdate-eabihf",
	                                            "thumbv7em-pdos-eabihf",
	                                            "thumbv7em-panic-pdos-eabihf",
	                                            "thumbv7em-playdateos-eabihf",
	                                            "thumbv7em-panic-playdateos-eabihf",
	                                            "thumbv7em-playdate-eabi",
	                                            "thumbv7em-panic-playdate-eabi",
	                                            "thumbv7em-pdos-eabi",
	                                            "thumbv7em-panic-pdos-eabi",
	                                            "thumbv7em-playdateos-eabi",
	                                            "thumbv7em-panic-playdateos-eabi",
	];

	pub const VENDOR: &str = "panic";
	/// OS name, also brand.
	pub const OS: &str = "playdate";
	/// Alternative OS names.
	pub const OS_ALT: &[&str] = &["playdateos", "pdos"];

	pub const ARCH: &str = "arm";
	pub const ABI: &[&str] = &["eabihf", "eabi"];
	pub const KIND: &str = "thumb";
	pub const CPU: &str = "cortex-m7";
	pub const FLOAT_ABI: &str = "hard";
	pub const FPU: &str = "fpv5-sp-d16"; // vfp4d16sp
	pub const FPU_TARGET_FEATURE: &str = "-fp64";
	pub const RELOCATION_MODEL: &str = "pie";
	pub const RELOCATION_MODEL_ALT: &[&str] = &["pic"];
	pub const ENTRY: &str = "eventHandlerShim";
}


pub mod os {
	use core::str::FromStr;


	/// Task kind of PD-RTOS.
	#[derive(Debug, Clone, Copy)]
	pub enum Task {
		/// Main task, for runloop callbacks.
		/// Also __network__ callbacks are here too.
		GameTask,
		/// Separated higher-priority task, __async__, __small__.
		/// Used for filter/sample callbacks.
		///
		/// Callbacks must be lightweight & fast as possible.
		/// So no allocations, no IO, no FS.
		Audio,
		/// System internals, incl. network.
		/// Scoreboard callbacks run directly in this task.
		System,
	}

	impl Task {
		/// FreeRTOS's software-defined task-stack mapping, in bytes.
		pub const fn max_stack_frame(self) -> u16 {
			match self {
				Self::GameTask => 10240_u16,
				Self::Audio => 4096,
				Self::System => 6144,
			}
		}

		pub const ALL: [Self; 3] = [Self::GameTask, Self::Audio, Self::System];
		pub const ALL_AS_STR: [&str; 3] = [
		                                   Self::GameTask.as_str(),
		                                   Self::Audio.as_str(),
		                                   Self::System.as_str(),
		];

		/// Name of the task.
		pub const fn as_str(&self) -> &'static str {
			match self {
				Self::GameTask => "main",
				Self::Audio => "audio",
				Self::System => "system",
			}
		}
	}

	impl FromStr for Task {
		type Err = &'static str;

		fn from_str(name: &str) -> Result<Self, Self::Err> {
			name.eq_ignore_ascii_case("main")
			    .then_some(Self::GameTask)
			    .or_else(|| name.eq_ignore_ascii_case("audio").then_some(Self::Audio))
			    .or_else(|| name.eq_ignore_ascii_case("system").then_some(Self::System))
			    .ok_or_else(|| "expected one of [main, audio, system]")
		}
	}
}


// pub mod package { /* TODO */ }
