use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Stdio;

use cargo::CargoResult;
use playdate::metadata::METADATA_FIELD;
use playdate::consts::SDK_ENV_VAR;

use crate::build::rustflags::Rustflags;
use crate::cli::target::PlaydateTarget;
use crate::config::Config;
use crate::proc;


pub fn new_or_init<'cfg>(config: &'cfg Config<'cfg>) -> CargoResult<()> {
	let cty_requested = config.args
	                          .iter()
	                          .find(|arg| arg.as_os_str() == OsStr::new("--lib"))
	                          .or_else(|| {
		                          config.args
		                                .iter()
		                                .find(|arg| arg.as_os_str() == OsStr::new("--bin"))
	                          })
	                          .map(|arg| arg.to_string_lossy().strip_prefix("--").map(ToString::to_string))
	                          .flatten();
	let mut cargo = proc::cargo_proxy_cmd(config, &config.cmd)?;
	if let Some(cty) = cty_requested {
		let cty = match cty.as_str() {
			"lib" => "library",
			"bin" => "executable binary",
			s => s,
		};
		config.log().status("Creating", format!("{cty} (requested)."));
	} else {
		cargo.arg("--lib");
		config.log().status("Creating", "library (default).");
	}

	config.log().verbose(|mut log| {
		            log.status("Cargo", proc::args_line_for_proc(&cargo));
	            });
	cargo.stderr(Stdio::inherit());
	cargo.stdout(Stdio::inherit());
	cargo.status()?.exit_ok()?;


	let path = config.create_path
	                 .as_deref()
	                 .ok_or(anyhow::anyhow!("Path not set"))?;
	let is_bin = path.join("src").join("main.rs").try_exists()?;

	// open manifest
	let manifest_path = path.join("Cargo.toml");
	let manifest_src = std::fs::read_to_string(&manifest_path)?;
	let mut manifest = manifest_src.parse::<toml_edit::Document>()?;


	// add metadata:
	if config.create_full_metadata {
		add_full_metadata(config, &mut manifest)?;
	} else {
		add_min_metadata(config, &mut manifest)?;
		reformat_metadata(config, &mut manifest)?;
	}


	// TODO: add dependencies
	let (deps_to_add, hl) = add_dependencies(config, &mut manifest)?;


	// sources:
	lib(config, &path, &mut manifest, hl)?;

	if is_bin {
		bin(config, &path, &mut manifest, hl)?;
	}


	// export manifest:
	let manifest = manifest.to_string();
	std::fs::write(manifest_path, manifest.trim_start())?;


	// cargo config:
	cargo_config(config, path.join(".cargo").join("config.toml"))?;


	// TODO: deps_to_add
	for dep in deps_to_add {
		// TODO call cargo add WITH PWD=path

		let mut cargo = proc::cargo(config.workspace.config().into())?;
		cargo.current_dir(path);
		cargo.arg("add");
		cargo.arg(dep.as_ref());
		cargo.stderr(Stdio::inherit());
		cargo.stdout(Stdio::inherit());
		cargo.status()?.exit_ok()?;
	}


	// ide configs:
	ide(config, path)?;


	Ok(())
}


/// Take uor metadata table, render it like a normal,
/// then add as trailing raw string to the end of file.
fn reformat_metadata(_config: &Config, manifest: &mut toml_edit::Document) -> CargoResult<()> {
	manifest["package"]["metadata"][METADATA_FIELD].as_inline_table_mut()
	                                               .map(|t| {
		                                               t.set_dotted(true);
		                                               t.fmt();
	                                               });


	let metadata = &manifest["package"]["metadata"][METADATA_FIELD];

	// TODO: Add comment https://sdk.play.date/Inside%20Playdate.html#pdxinfo
	//       Or better to playdate-build crate documentation.
	let mut raw_metadata_lines = vec![
		     Cow::Borrowed("\n"),
		     "# See more about playdate metadata:".into(),
		     "# https://github.com/boozook/playdate/blob/main/support/build/README.md#assets".into(),
		     format!("[package.metadata.{METADATA_FIELD}]").into(),
		];

	if let Some(table) = metadata.as_table_like() {
		let mut keys = Vec::new();
		for (key, value) in table.iter() {
			keys.push(key.to_owned());
			raw_metadata_lines.push(format!("{key} = {}", value.to_string()).into());
		}
	}

	// remove original our metadata table:
	manifest["package"]["metadata"].as_table_like_mut()
	                               .map(|table| table.remove(METADATA_FIELD));
	// remove original entire metadata table if it empty:
	if manifest["package"]["metadata"].as_table_like()
	                                  .into_iter()
	                                  .filter(|t| t.is_empty())
	                                  .next()
	                                  .is_some()
	{
		manifest["package"].as_table_like_mut()
		                   .map(|table| table.remove("metadata"));
	}

	let raw_metadata = raw_metadata_lines.join("\n");
	manifest.set_trailing(raw_metadata);

	Ok(())
}


fn add_min_metadata(_config: &Config<'_>, manifest: &mut toml_edit::Document) -> CargoResult<()> {
	use toml_edit::value;

	let name = manifest["package"]["name"].as_str()
	                                      .unwrap_or("hello-world")
	                                      .to_owned();
	let bundle_id = name.replace("_", ".")
	                    .replace("-", ".")
	                    .replace("..", ".")
	                    .replace("..", ".");
	let has_description = manifest["package"].as_table()
	                                         .map(|t| t.contains_key("description"))
	                                         .unwrap_or(false);

	let meta = &mut manifest["package"]["metadata"][METADATA_FIELD];

	meta["bundle-id"] = value(format!("com.{}", bundle_id.strip_prefix('.').unwrap_or(&bundle_id)));
	if !has_description {
		meta["description"] = value(format!("Description of {name} program."));
	}

	// options:
	meta["options"]["assets"]["dependencies"] = value(true);
	meta["options"]["assets"]["overwrite"] = value(true);

	Ok(())
}

fn add_full_metadata(_config: &Config<'_>, manifest: &mut toml_edit::Document) -> CargoResult<()> {
	use toml_edit::Document;

	let name = manifest["package"]["name"].as_str()
	                                      .unwrap_or("hello-world")
	                                      .to_owned();
	let bundle_id = name.replace("_", ".").replace("-", ".");
	let version = manifest["package"]["version"].as_str().unwrap_or("0.0");

	let author_default = "You, Inc";
	let author: Cow<str> = if manifest["package"].as_table()
	                                             .map(|t| t.contains_key("authors"))
	                                             .unwrap_or(false)
	{
		manifest["package"]["authors"].as_array()
		                              .into_iter()
		                              .flat_map(|arr| arr.get(0))
		                              .flat_map(|v| v.as_str().map(Into::into))
		                              .next()
		                              .unwrap_or(author_default.into())
	} else {
		author_default.into()
	};


	let description_default = format!("Description for the {name} program.");
	let description: Cow<str> = if manifest["package"].as_table()
	                                                  .map(|t| t.contains_key("description"))
	                                                  .unwrap_or(false)
	{
		manifest["package"]["description"].as_str()
		                                  .map(Into::into)
		                                  .unwrap_or_else(|| description_default.into())
	} else {
		description_default.into()
	};

	let toml = format!(
	                   include_str!("full-metadata.toml"),
	                   name = name,
	                   author = author,
	                   version = version,
	                   bundle_id = bundle_id,
	                   description = description
	);
	toml.parse::<Document>().expect("invalid doc");

	let raw_metadata = format!("\n\n{}\n", toml.trim());
	manifest.set_trailing(raw_metadata);

	Ok(())
}


fn cargo_add<'s>(config: &Config<'_>,
                 pwd: &Path,
                 manifest: &Path,
                 name: &str,
                 git: bool,
                 rename: Option<&str>,
                 features: Option<impl IntoIterator<Item = &'s str>>)
                 -> CargoResult<()> {
	let mut cargo = proc::cargo(config.workspace.config().into())?;
	cargo.current_dir(pwd);

	cargo.arg("add");
	cargo.arg(name);

	if let Some(name) = rename {
		cargo.arg("--rename");
		cargo.arg(name);
	}

	if let Some(features) = features {
		let features = features.into_iter().collect::<Vec<_>>().join(",");
		cargo.arg("--features");
		cargo.arg(features);
	}

	// git => --git="URL"

	cargo.arg("manifest-path");
	cargo.arg(manifest);

	cargo.stderr(Stdio::inherit());
	cargo.stdout(Stdio::inherit());
	cargo.status()?.exit_ok()?;
	Ok(())
}


fn add_dependencies<'cfg>(config: &'cfg Config<'_>,
                          manifest: &mut toml_edit::Document)
                          -> CargoResult<(Vec<Cow<'cfg, str>>, bool)> {
	use toml_edit::value;
	use crate::cli::deps::Dependency as Dep;
	use crate::cli::deps::DependencyName as Name;
	use crate::cli::deps::DependencySource as Src;

	fn add_dep(dep: &Dep<'_>, manifest: &mut toml_edit::Document) -> CargoResult<()> {
		match dep.source {
			Src::CratesIo => manifest["dependencies"][dep.name.to_string()] = value("*"),
			Src::Git => {
				let name = dep.name.to_string();
				let url = dep.git()
				             .ok_or(anyhow::anyhow!("Don't know git repository for {}", name))?;
				manifest["dependencies"][&name] = toml_edit::table();
				manifest["dependencies"][&name]["git"] = value(url);
				manifest["dependencies"][&name]["version"] = value("*");
			},
		}
		Ok(())
	}

	let mut add_pd = false;

	if config.create_deps_sys_only {
		let default = Dep { name: Name::Sys,
		                    source: Src::CratesIo };
		let dep = config.create_deps
		                .iter()
		                .find(|d| matches!(d.name, Name::Sys))
		                .unwrap_or(&default);
		add_dep(dep, manifest)?;
		Ok((vec![], add_pd))
	} else {
		let known = config.create_deps
		                  .iter()
		                  .filter(|d| !matches!(d.name, Name::Other(_)));
		for dep in known {
			add_pd = matches!(dep.name, Name::Playdate) || add_pd;
			add_dep(dep, manifest)?;
		}

		let others = config.create_deps.iter().filter_map(|d| {
			                                      if let Name::Other(dep) = &d.name {
				                                      Some(dep.to_owned())
			                                      } else {
				                                      None
			                                      }
		                                      });
		Ok((others.collect(), add_pd))
	}
}


/// 1. render rustflags to config
/// 2. add `[unstable]` section with required nightly features
fn cargo_config<P: AsRef<Path>>(config: &Config, path: P) -> CargoResult<()> {
	let mut doc = toml::Table::new();

	// target tables:
	let mut targets = toml::Table::new();

	// default
	if !config.create_full_config {
		// target[].rustflags:
		let rustflags: Vec<toml::Value> = Rustflags::rustflags_bin_playdate().into_iter()
		                                                                     .map(|s| toml::Value::String(s.to_string()))
		                                                                     .collect();

		let mut target_table = toml::Table::new();
		target_table.insert("rustflags".into(), rustflags.into());

		let ct = PlaydateTarget::device_compile_target();
		targets.insert(ct.short_name().into(), target_table.into());
	} else {
		let rustflags = Rustflags::try_default(config)?;
		for (kind, flags) in rustflags.targets()
		                              .map(|k| rustflags.get(k).map(|flags| (k, flags)))
		                              .flatten()
		{
			let rustflags: Vec<toml::Value> = flags.into_iter()
			                                       .map(|s| toml::Value::String(s.to_string()))
			                                       .collect();
			let mut target_table = toml::Table::new();
			target_table.insert("rustflags".into(), rustflags.into());

			let ct_name = match kind {
				cargo::core::compiler::CompileKind::Host => config.host_target.short_name(),
				cargo::core::compiler::CompileKind::Target(ct) => ct.short_name(),
			};
			targets.insert(ct_name.into(), target_table.into());
		}

		// also add env:
		let mut env = toml::Table::new();
		env.insert(
		           SDK_ENV_VAR.into(),
		           config.sdk()?.path().display().to_string().into(),
		);
		doc.insert("env".into(), env.into());
	}
	doc.insert("target".into(), targets.into());


	// NOTE: removed as workaround for issue:
	// https://github.com/boozook/playdate/issues/221
	// unstable features:
	// let mut unstable = toml::Table::new();
	// unstable.insert("unstable-options".into(), true.into());
	// unstable.insert("avoid-dev-deps".into(), true.into());
	// let check_cfg: Vec<toml::Value> = ["names", "values", "output"].into_iter()
	//                                                                .map(|s| toml::Value::String(s.to_string()))
	//                                                                .collect();
	// unstable.insert("check-cfg".into(), check_cfg.into());
	// doc.insert("unstable".into(), unstable.into());


	// export:
	if let Some(parent) = path.as_ref().parent() {
		if !parent.try_exists()? {
			std::fs::create_dir_all(parent)?;
		}
	}
	std::fs::write(path, doc.to_string())?;

	Ok(())
}


fn lib(config: &Config, root: &Path, manifest: &mut toml_edit::Document, hl: bool) -> CargoResult<()> {
	use toml_edit::Document;

	let toml = r#"
[lib]
path = "src/lib.rs"
crate-type = [
	"staticlib", # for hardware
	"dylib",     # for simulator
	"rlib",      # to link with bin
]
"#;
	manifest["lib"] = toml.parse::<Document>().expect("invalid doc")["lib"].clone();

	let path = root.join("src").join("lib.rs");
	let src = if config.create_deps_sys_only || !hl {
		&include_bytes!("lib-ll.rs")[..]
	} else {
		&include_bytes!("lib-hl.rs")[..]
	};
	std::fs::write(path, src)?;
	Ok(())
}

fn bin(config: &Config, root: &Path, manifest: &mut toml_edit::Document, hl: bool) -> CargoResult<()> {
	let path = root.join("src").join("main.rs");
	let name = manifest["package"]["name"].as_str()
	                                      .unwrap_or("hello-world")
	                                      .to_owned()
	                                      .replace("-", "_");
	let src = if config.create_deps_sys_only || !hl {
		format!(include_str!("bin-ll.rs"), crate_name = name)
	} else {
		format!(include_str!("bin-hl.rs"), crate_name = name)
	};
	std::fs::write(path, src)?;
	Ok(())
}


fn ide<P: AsRef<Path>>(config: &Config, path: P) -> CargoResult<()> {
	match config.ide {
		crate::cli::ide::Ide::None => return Ok(()),
		crate::cli::ide::Ide::Vscode => (),
	};

	let cfg_dir = path.as_ref().join(".vscode");
	if cfg_dir.try_exists().ok() == Some(true) {
		config.log().warn("IDE configuration directory already exists.");
		return Ok(());
	}

	std::fs::create_dir_all(&cfg_dir)?;

	let extensions = cfg_dir.join("extensions.json");
	std::fs::write(extensions, include_bytes!("ide/vsc/extensions.json"))?;

	Ok(())
}


#[cfg(test)]
mod tests {
	use toml_edit::Document;


	#[test]
	fn full_metadata_is_valid() {
		let toml = format!(
		                   include_str!("full-metadata.toml"),
		                   name = "name",
		                   author = "author",
		                   version = "version",
		                   bundle_id = "bundle-id",
		                   description = "description"
		);
		toml.parse::<Document>().expect("invalid doc");
	}
}
