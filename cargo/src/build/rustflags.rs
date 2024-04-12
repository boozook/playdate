use std::borrow::Cow;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Add;
use anyhow::Context;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CompileTarget;
use cargo::util;
use playdate::compile::RUSTFLAGS_BIN_PLAYDATE;
use playdate::compile::RUSTFLAGS_LIB_HOST;
use playdate::compile::RUSTFLAGS_LIB_PLAYDATE;
use playdate::consts::DEVICE_TARGET;

use crate::config::Config;
use crate::logger::LogErr;


pub struct Rustflags {
	flags: BTreeMap<CompileKind, Vec<Cow<'static, str>>>,
}


impl Rustflags {
	pub const fn rustflags_lib_host() -> &'static [&'static str] { RUSTFLAGS_LIB_HOST }
	pub const fn rustflags_lib_playdate() -> &'static [&'static str] { RUSTFLAGS_LIB_PLAYDATE }
	pub const fn rustflags_bin_playdate() -> &'static [&'static str] { RUSTFLAGS_BIN_PLAYDATE }


	pub fn try_default(config: &Config) -> anyhow::Result<Self> {
		let host = config.host_target;
		let device_target = CompileTarget::new(DEVICE_TARGET).expect("invalid target");

		let prevent_unwinding = || -> Option<Cow<str>> {
			if config.prevent_unwinding {
				Some("-Cpanic=abort".into())
			} else {
				None
			}
		};
		let bins_targeted = true; // TODO: determine from config
		let rustflags_device = if !bins_targeted {
			Self::rustflags_lib_playdate().into_iter()
			                              .map(|s| Cow::from(*s))
			                              .chain(prevent_unwinding().into_iter())
			                              .collect()
		} else {
			let sdk = config.sdk()
			                .log_err_cargo(config)
			                .with_context(|| "Playdate Sdk needed to build binaries")?;

			if config.no_gcc {
				// export LINK MAP:
				let target_dir = config.workspace
				                       .config()
				                       .target_dir()
				                       .unwrap_or_default()
				                       .unwrap_or_else(|| util::Filesystem::new("target".into()))
				                       .into_path_unlocked()
				                       .canonicalize()?;
				let map = target_dir.join("pd.x");
				if !map.exists() {
					std::fs::write(&map, build::compile::LINK_MAP_BIN_SRC)?;
				}
				let link_map = format!("-Clink-arg=-T{}", map.display());
				Self::rustflags_bin_playdate().into_iter()
				                              .map(|s| Cow::from(*s))
				                              .chain([link_map.into()])
				                              .chain(prevent_unwinding().into_iter())
				                              .collect()
			} else {
				let arm = config.gcc()
				                .log_err_cargo(config)
				                .with_context(|| "ARM GNU toolchain needed to build binaries")?;

				let link_map = format!("-Clink-arg=-T{}", sdk.build_support().link_map().display());
				let lib_search_paths = arm.lib_search_paths_for_playdate()
				                          .or_else(|_| arm.lib_search_paths_default())?;
				Self::rustflags_bin_playdate().into_iter()
				                              .map(|s| Cow::from(*s))
				                              .chain([link_map.into()])
				                              .chain(lib_search_paths.into_iter().map(|s| {
					                              "-L".to_owned().add(s.to_string_lossy().as_ref()).into()
				                              }))
				                              .chain(prevent_unwinding().into_iter())
				                              .collect()
			}
		};


		let mut flags: BTreeMap<CompileKind, Vec<_>> = BTreeMap::new();
		let rustflags_lib_host = || {
			Self::rustflags_lib_host().into_iter()
			                          .map(|s| Cow::from(*s))
			                          .chain(prevent_unwinding().into_iter())
			                          .collect()
		};
		flags.insert(CompileKind::Host, rustflags_lib_host());
		flags.insert(CompileKind::Target(host), rustflags_lib_host());
		flags.insert(CompileKind::Target(device_target), rustflags_device);


		let existing = [&device_target, &config.host_target].into_iter()
		                                                    .map(|ck| {
			                                                    config.workspace
			                                                          .config()
			                                                          .target_cfg_triple(ck.short_name())
			                                                          .ok()
			                                                          .and_then(|tcfg| tcfg.rustflags)
			                                                          .map(|flags| (ck, flags))
		                                                    })
		                                                    .flatten()
		                                                    .collect::<HashMap<_, _>>();
		// remove conflicting flags:
		let mut drained = Vec::new();
		for target in [&device_target, &config.host_target] {
			if let Some(existing) = existing.get(&target) {
				if let Some(flags) = flags.get_mut(&CompileKind::Target(*target)) {
					drained.extend(flags.extract_if(|flag| {
						                    existing.val
						                            .as_slice()
						                            .iter()
						                            .find(|f| f.as_str() == flag.as_ref())
						                            .is_some()
					                    }));
					if existing.val
					           .as_slice()
					           .iter()
					           .find(|f| f.contains("target-cpu"))
					           .is_some()
					{
						drained.extend(flags.extract_if(|flag| flag.starts_with("-Ctarget-cpu=")));
					}
				}
			}
		}
		log::debug!("removed duplicates flags: {drained:#?}");
		flags.extract_if(|_, flags| flags.is_empty()).count();
		Ok(Self { flags })
	}


	/// Render RUSTFLAGS to args for cargo:
	/// ```text
	/// --config='target.{HOST}.rustflags="-Ctarget-cpu=native"'
	/// --config='target.{DEVICE}.rustflags="-C target-cpu=cortex-m7"'
	/// ```
	/// Uses [cli-config](https://github.com/rust-lang/cargo/issues/6699)
	pub fn rustflags_to_args(&self, targets: &[&CompileTarget], host_target: &CompileTarget) -> Vec<String> {
		// this to catch backslashes and then escape it:
		let re = regex::Regex::new(r"([^\\])\\([^\\])").unwrap();
		let fmt = |t, args: &[Cow<'_, str>]| {
			let args: Vec<_> = args.into_iter()
			                       .map(|s| (s, re.replace_all(&s, r"$1\\$2")))
			                       .map(|(src, esc)| {
				                       if src.as_ref() != esc.as_ref() {
					                       log::trace!("replace: {src}");
					                       log::trace!("     to: {esc}");
				                       }
				                       esc
			                       })
			                       .collect();
			format!("--config=target.{t}.rustflags=[\"{}\"]", args.join("\", \""))
		};

		if targets.is_empty() {
			if let Some(args) = self.flags.get(&CompileKind::Host) {
				let t = host_target.rustc_target();
				return vec![fmt(t.as_str(), args)];
			}
		} else {
			let mut result = Vec::with_capacity(targets.len());
			for target in targets.iter() {
				if let Some(args) = self.flags.get(&CompileKind::Target(**target)) {
					let t = target.rustc_target();
					result.push(fmt(t.as_str(), args));
				}
			}
			return result;
		}
		Vec::with_capacity(0)
	}


	pub fn rustflags_to_args_from(&self, cfg: &Config) -> Vec<String> {
		let targets = cfg.compile_options
		                 .build_config
		                 .requested_kinds
		                 .iter()
		                 .map(|kind| {
			                 match kind {
				                 CompileKind::Host => &cfg.host_target,
			                    CompileKind::Target(target) => target,
			                 }
		                 })
		                 .collect::<Vec<_>>();
		self.rustflags_to_args(&targets[..], &cfg.host_target)
	}


	pub fn targets(&self) -> impl Iterator<Item = &CompileKind> { self.flags.keys() }

	pub fn get(&self, kind: &CompileKind) -> Option<&[Cow<'_, str>]> {
		if let Some(vec) = self.flags.get(kind) {
			Some(&vec[..])
		} else {
			None
		}
	}
}
