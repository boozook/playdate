use core::fmt::Display;
use core::str::FromStr;
use std::ffi::OsStr;
use crate::spec::*;


/// Playdate Target Kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TargetKind {
	/// Playdate Device.
	Device,
	/// Probably Playdate Simulator (any other target kind).
	Simulator,
}

impl Display for TargetKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Device => write!(f, "device"),
			Self::Simulator => write!(f, "simulator"),
		}
	}
}


// name/shorthand from/to

impl TargetKind {
	pub const DEV_OPTS: [&str; 4] = [OS, "device", "dev", "pddev"];
	pub const SIM_OPTS: [&str; 3] = ["simulator", "sim", "pdsim"];
}

impl FromStr for TargetKind {
	type Err = std::io::Error;
	fn from_str(s: &str) -> Result<Self, Self::Err> { Self::from_alias(s.trim()) }
}

impl TargetKind {
	pub fn is_alias(target: &str) -> bool { Self::from_alias(target).is_ok() }
	pub fn from_alias(target: &str) -> Result<Self, std::io::Error> {
		use std::io::{Error, ErrorKind};

		Self::DEV_OPTS.contains(&target)
		              .then_some(Self::Device)
		              .or_else(|| Self::SIM_OPTS.contains(&target).then_some(Self::Simulator))
		              .ok_or(Error::new(ErrorKind::InvalidData, target))
	}


	/// From rustc's spec short name
	pub fn from_target(name: &str) -> Self { Self::from_str(name).unwrap_or_else(|_| Self::from_triple(name)) }
}


// detect
impl TargetKind {
	/// Retrieve by cargo env vars.
	pub fn is_pd_from_cargo_env() -> Result<bool, std::env::VarError> {
		use std::env::var_os;
		use std::env::VarError::NotPresent;


		let is_pdos = || var_os("CARGO_CFG_TARGET_OS").is_some_and(|v| Self::is_pd_os(v));
		let is_panic = || var_os("CARGO_CFG_TARGET_VENDOR").is_some_and(|v| Self::is_pd_vendor(v));
		let arch = || var_os("CARGO_CFG_TARGET_ARCH").is_some_and(|v| v.eq_ignore_ascii_case(ARCH));
		let size = || var_os("CARGO_CFG_TARGET_POINTER_WIDTH").is_some_and(|v| v == "32");
		let abi =
			|| var_os("CARGO_CFG_TARGET_ABI").is_some_and(|v| ABI.iter().any(|abi| v.eq_ignore_ascii_case(abi)));
		let target = || {
			let target = var_os("TARGET").ok_or(NotPresent)?;
			Ok(target == LLVM_TARGET_HF || target == LLVM_TARGET_SF || Self::is_pd_triple(target))
		};

		Ok((arch() && size() && abi() && (is_pdos() || is_panic())) || target()?)
	}

	/// Retrieve by cargo env vars.
	pub fn from_cargo_env() -> Result<Self, std::env::VarError> {
		Self::is_pd_from_cargo_env().map(|is_pd| is_pd.then_some(Self::Device).unwrap_or(Self::Simulator))
	}


	pub fn is_pd_from_parts(os: Option<impl AsRef<OsStr>>,
	                        vendor: Option<impl AsRef<OsStr>>,
	                        arch: Option<impl AsRef<OsStr>>,
	                        abi: Option<impl AsRef<OsStr>>,
	                        size: Option<impl AsRef<OsStr>>,
	                        triple: Option<impl AsRef<OsStr>>)
	                        -> bool {
		(arch.is_some_and(|v| v.as_ref().eq_ignore_ascii_case(ARCH)) &&
		 size.is_some_and(|v| v.as_ref() == "32") &&
		 abi.is_some_and(|v| ABI.iter().any(|abi| v.as_ref().eq_ignore_ascii_case(abi))) &&
		 (os.is_some_and(|v| Self::is_pd_os(v)) || vendor.is_some_and(|v| Self::is_pd_vendor(v)))) ||
		triple.is_some_and(|v| v.as_ref() == LLVM_TARGET_HF || Self::is_pd_triple(v))
	}

	pub fn from_parts(os: Option<impl AsRef<OsStr>>,
	                  vendor: Option<impl AsRef<OsStr>>,
	                  arch: Option<impl AsRef<OsStr>>,
	                  abi: Option<impl AsRef<OsStr>>,
	                  size: Option<impl AsRef<OsStr>>,
	                  triple: Option<impl AsRef<OsStr>>)
	                  -> Self {
		Self::is_pd_from_parts(os, vendor, arch, abi, size, triple).then_some(Self::Device)
		                                                           .unwrap_or(Self::Simulator)
	}
	pub fn from_triple(target: impl AsRef<OsStr>) -> Self {
		Self::is_pd_triple(target).then_some(Self::Device)
		                          .unwrap_or(Self::Simulator)
	}

	pub fn is_pd_os(os: impl AsRef<OsStr>) -> bool {
		let os = os.as_ref();
		[OS].iter().chain(OS_ALT).any(|&v| os.eq_ignore_ascii_case(v))
	}

	pub fn is_pd_vendor(vendor: impl AsRef<OsStr>) -> bool { vendor.as_ref().eq_ignore_ascii_case(VENDOR) }


	pub fn is_pd_triple(target: impl AsRef<OsStr>) -> bool {
		let target = target.as_ref();

		[LLVM_TARGET_HF, LLVM_TARGET_SF].iter()
		                                .chain(TARGET_SHORTNAME_ALT)
		                                .any(|name| target.eq_ignore_ascii_case(name)) ||
		{
			// subslice search


			#[cfg(not(unix))]
			let target = target.to_string_lossy();
			#[cfg(unix)]
			use std::os::unix::prelude::OsStrExt;

			target.as_bytes().starts_with(b"thumbv7em") &&
			ABI.iter().any(|&abi| target.as_bytes().ends_with(abi.as_bytes())) &&
			([OS].iter()
			     .chain(OS_ALT)
			     .any(|&v| target.as_bytes().windows(v.len()).any(|p| p == v.as_bytes())) ||
			 target.as_bytes()
			       .windows(VENDOR.len())
			       .any(|part| part == VENDOR.as_bytes()))
		}
	}


	pub fn is_playdate(&self) -> bool { matches!(self, Self::Device) }
	pub fn is_simulator(&self) -> bool { matches!(self, Self::Simulator) }
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn is_variant() {
		assert!(TargetKind::Device.is_playdate());
		assert!(!TargetKind::Device.is_simulator());
		assert!(!TargetKind::Simulator.is_playdate());
		assert!(TargetKind::Simulator.is_simulator());
	}

	#[test]
	fn triple() {
		const TRUE: &[&str] = &[
		                        "thumbv7em-playdate-eabihf",
		                        "thumbv7em-panic-playdate-eabihf",
		                        "thumbv7em-pdos-eabihf",
		                        LLVM_TARGET_HF,
		                        "thumbv7em-playdate-eabi",
		                        "thumbv7em-panic-playdate-eabi",
		                        "thumbv7em-pdos-eabi",
		                        LLVM_TARGET_SF,
		];
		const FALSE: &[&str] = &[
		                         "aarch64-apple-darwin",
		                         "wasm32-unknown-unknown",
		                         "thumbv7em-pd-eabihf",
		];

		for target in TRUE {
			assert!(TargetKind::is_pd_triple(target));
			assert!(TargetKind::from_triple(target).is_playdate());
		}
		for target in FALSE {
			assert!(!TargetKind::is_pd_triple(target));
			assert!(TargetKind::from_triple(target).is_simulator());
		}
	}


	#[test]
	fn vendor() {
		let ok: &[&str] = &[&VENDOR.to_lowercase(), &VENDOR.to_uppercase()];
		let nope: &[&str] = &["playdate", "PLAYDATE", "pdos", "pd", "apple", "unknown", "none"];

		for v in ok {
			assert!(TargetKind::is_pd_vendor(v));
		}
		for v in nope {
			assert!(!TargetKind::is_pd_vendor(v));
		}
	}

	#[test]
	fn parts() {
		type Test = [[Option<&'static str>; 6]];

		const TRUE: &Test = &[
		                      [
			Some(OS),
			Some(VENDOR),
			Some(ARCH),
			Some(ABI[0]),
			Some("32"),
			Some(LLVM_TARGET_HF),
		],
		                      [
			Some(OS),
			Some(VENDOR),
			Some(ARCH),
			Some(ABI[1]),
			Some("32"),
			Some(LLVM_TARGET_SF),
		],
		                      [Some(OS), Some(VENDOR), Some(ARCH), Some(ABI[0]), Some("32"), None],
		                      [Some(OS), Some(VENDOR), Some(ARCH), Some(ABI[1]), Some("32"), None],
		                      [Some(OS), None, Some(ARCH), Some(ABI[0]), Some("32"), None],
		                      [Some(OS), None, Some(ARCH), Some(ABI[1]), Some("32"), None],
		                      [Some(OS_ALT[0]), None, Some(ARCH), Some(ABI[0]), Some("32"), None],
		                      [Some(OS_ALT[0]), None, Some(ARCH), Some(ABI[1]), Some("32"), None],
		                      [Some(OS_ALT[1]), None, Some(ARCH), Some(ABI[0]), Some("32"), None],
		                      [Some(OS_ALT[1]), None, Some(ARCH), Some(ABI[1]), Some("32"), None],
		                      [None, Some(VENDOR), Some(ARCH), Some(ABI[0]), Some("32"), None],
		                      [None, Some(VENDOR), Some(ARCH), Some(ABI[1]), Some("32"), None],
		                      [None, None, None, None, Some("32"), Some(LLVM_TARGET_HF)],
		                      [None, None, None, None, Some("32"), Some(LLVM_TARGET_SF)],
		                      [None, None, None, None, None, Some(LLVM_TARGET_HF)],
		                      [None, None, None, None, None, Some(LLVM_TARGET_SF)],
		                      [None, None, None, None, None, Some(TARGET_SHORTNAME_ALT[0])],
		                      [None, None, None, None, None, Some(TARGET_SHORTNAME_ALT[1])],
		                      [None, None, None, None, None, Some(TARGET_SHORTNAME_ALT[2])],
		                      [None, None, None, None, None, Some(TARGET_SHORTNAME_ALT[3])],
		];
		const FALSE: &Test = &[
		                       [None, None, Some(ARCH), None, Some("32"), None],
		                       [None, None, None, None, None, Some("thumbv7em-pd-eabihf")],
		                       [None, None, None, None, None, Some("thumbv7em-unknown-eabihf")],
		                       [None, None, None, None, None, Some("wasm32-unknown-unknown")],
		];

		for &[os, vendor, arch, abi, size, triple] in TRUE {
			assert!(TargetKind::is_pd_from_parts(os, vendor, arch, abi, size, triple));
			assert!(TargetKind::from_parts(os, vendor, arch, abi, size, triple).is_playdate());
		}
		for &[os, vendor, arch, abi, size, triple] in FALSE {
			assert!(!TargetKind::is_pd_from_parts(os, vendor, arch, abi, size, triple));
			assert!(TargetKind::from_parts(os, vendor, arch, abi, size, triple).is_simulator());
		}
	}


	#[test]
	fn from_to() {
		assert_eq!(
		           TargetKind::Device,
		           TargetKind::from_str(&TargetKind::Device.to_string()).unwrap()
		);
		assert_eq!(
		           TargetKind::Simulator,
		           TargetKind::from_str(&TargetKind::Simulator.to_string()).unwrap()
		);
	}

	#[test]
	fn to_str() {
		assert_eq!("device", TargetKind::Device.to_string());
		assert_eq!("simulator", TargetKind::Simulator.to_string());
	}

	#[test]
	fn str_opts() {
		for expected in [OS].iter().chain(["device", "dev"].iter()) {
			assert!(TargetKind::DEV_OPTS.contains(expected));
		}
		for expected in &["simulator", "sim"] {
			assert!(TargetKind::SIM_OPTS.contains(expected));
		}
	}

	#[test]
	fn from_str() {
		for name in TargetKind::DEV_OPTS {
			assert_eq!(TargetKind::from_str(name).unwrap(), TargetKind::Device);
		}

		for name in TargetKind::SIM_OPTS {
			assert_eq!(TargetKind::from_str(name).unwrap(), TargetKind::Simulator);
		}

		for name in ["other", "foo", "bar", "none", "unknown"] {
			assert!(TargetKind::from_str(name).is_err());
		}
	}

	#[test]
	/// From rustc' target shortname
	fn from_target() {
		for &name in [LLVM_TARGET_HF].iter()
		                             .chain(TARGET_SHORTNAME_ALT)
		                             .chain(&TargetKind::DEV_OPTS)
		{
			assert_eq!(TargetKind::from_target(name), TargetKind::Device);
		}

		for &name in ["other", "foo", "bar", "none", "unknown"].iter()
		                                                       .chain(&TargetKind::SIM_OPTS)
		{
			assert_eq!(TargetKind::from_target(name), TargetKind::Simulator);
		}
	}
}
