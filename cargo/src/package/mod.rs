use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::anyhow;
use anyhow::bail;
use cargo::core::PackageId;
use cargo::CargoResult;
use cargo::core::Package;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CrateType;
use cargo::core::profiles::DebugInfo;
use cargo::core::profiles::Profiles;
use cargo_util_schemas::manifest::TomlDebugInfo;
use clap_lex::OsStrExt;
use playdate::fs::soft_link_checked;
use playdate::layout::Layout;
use playdate::layout::Name;
use playdate::manifest::format::ManifestFmt;
use playdate::manifest::PackageSource;
use playdate::metadata::validation::Validate;
use playdate::metadata::validation::ValidateCrate;

use crate::assets::proto::AssetsArtifactsNew;
use crate::assets::proto::AssetsArtifact as AssetsArtifactNew;
use crate::build::BuildProduct;
use crate::config::Config;
use crate::layout::CrossTargetLayout;
use crate::layout::LayoutLockable;
use crate::layout::ForTargetLayout;
use crate::proc::logging::cmd_logged;
use crate::proc::reader::format::ArtifactProfile;
use crate::utils::cargo::meta_deps::RootNode;
use crate::utils::path::AsRelativeTo;

mod ar;


#[derive(Debug)]
pub struct Product {
	pub package_id: PackageId,
	pub crate_types: Vec<CrateType>,
	pub targets: Vec<CompileKind>,
	/// Build-product name
	pub name: String,
	/// Path of produced artifact - pdx-dir or zip-file
	pub path: PathBuf,
}


pub fn build_all(config: &'_ Config,
                 assets: AssetsArtifactsNew<'_, '_>,
                 products: Vec<BuildProduct<'_>>)
                 -> CargoResult<Vec<Product>> {
	let products: Vec<SuccessfulBuildProduct> = products.into_iter().flat_map(TryInto::try_into).collect();
	let mut targets = HashMap::<_, Vec<_>>::new();

	for product in products {
		let key = (product.package, product.name.to_owned());
		if let Some(products) = targets.get_mut(&key) {
			products.push(product);
		} else {
			targets.insert(key, vec![product]);
		}
	}

	let mut results = Vec::new();

	for ((package, _), mut products) in targets {
		let package_id = package.package_id();

		log::debug!(
		            "Looking for assets artifacts for ({}) {}::{} for {}:",
		            &products[0].src_ct,
		            package_id,
		            products[0].name,
		            match &products[0].ck {
			            CompileKind::Host => "host",
		               CompileKind::Target(ref kind) => kind.short_name(),
		            }
		);

		let (root, assets) = assets.iter()
		                           .find_map(|(r, arts)| {
			                           let unit = r.node().unit();
			                           (unit.package_id == package_id &&
			                            unit.platform == products[0].ck &&
			                            unit.target.crate_types.contains(&products[0].src_ct) &&
			                            unit.target.name == products[0].name)
			                                                        .then_some((r, Some(arts)))
		                           })
		                           .or_else(|| {
			                           assets.tree
			                                 .roots()
			                                 .iter()
			                                 .find(|r| {
				                                 let unit = r.node().unit();
				                                 unit.package_id == package_id &&
				                                 unit.platform == products[0].ck &&
				                                 unit.target.crate_types.contains(&products[0].src_ct) &&
				                                 unit.target.name == products[0].name
			                                 })
			                                 .map(|r| (r, None))
		                           })
		                           .ok_or_else(|| {
			                           anyhow!(
			                                   "Root not found for ({}) {}::{} for {}",
			                                   &products[0].src_ct,
			                                   package_id,
			                                   products[0].name,
			                                   match products[0].ck {
				                                   CompileKind::Host => "host",
			                                      CompileKind::Target(ref kind) => kind.short_name(),
			                                   }
			)
		                           })?;

		match products.len() {
			0 => unreachable!("impossible len=0"),
			1 => {
				let product = products.pop().unwrap();
				let result = package_single_target(config, product, root, assets)?;
				results.push(result);
			},
			_ => {
				let result = package_multi_target(config, package, products, root, assets)?;
				results.push(result);
			},
		}
	}

	Ok(results)
}


fn package_single_target<'p, 'art>(config: &Config,
                                   product: SuccessfulBuildProduct<'p>,
                                   root: &RootNode<'_>,
                                   assets: Option<impl Iterator<Item = &'art AssetsArtifactNew>>)
                                   -> CargoResult<Product> {
	let presentable_name = product.presentable_name();
	config.log().status(
	                    "Packaging",
	                    format!(
		"(single-target) {presentable_name} (source: {}, target: {})",
		product.src_ct, product.dst_ct
	),
	);


	if config.compile_options.build_config.force_rebuild {
		// TODO: clean entire product.layout.build() if force rebuild requested
	}


	if let Some(assets) = assets {
		for art in assets {
			log::debug!("Packaging assets {:?} {}", art.kind, product.presentable_name());

			prepare_assets(config, art, product.layout.build(), true, product.layout.root())?;
		}
	}

	// manifest:
	let cargo_target = (matches!(product.src_ct, CrateType::Bin) || product.example).then_some(&product.name)
	                                                                                .map(Cow::from);
	build_manifest(
	               config,
	               &product.layout,
	               product.package,
	               root.as_source(),
	               cargo_target,
	               product.example,
	)?;

	// finally call pdc and pack:
	let mut artifact = execute_pdc(config, &product.layout)?;
	if !config.no_info_meta {
		ar::add_info_meta(&artifact)?;
	}
	if config.zip {
		artifact = ar::build(artifact)?;
	}

	let result = Product { name: product.name,
	                       package_id: product.package.package_id(),
	                       crate_types: vec![product.src_ct.clone()],
	                       targets: vec![product.ck],
	                       path: artifact.to_path_buf() };


	Ok(result)
}


/// We can pack three targets in a package:
/// - unix-family's dylib
/// - windows's dll
/// - elf for hardware
///
/// So one executable and one or two dylibs.
///
/// __Can't mix macos dylib with linux dylib in a one package.__
fn package_multi_target<'p, 'art>(config: &Config,
                                  package: &'p Package,
                                  products: Vec<SuccessfulBuildProduct>,
                                  root: &RootNode<'_>,
                                  assets: Option<impl Iterator<Item = &'art AssetsArtifactNew>>)
                                  -> CargoResult<Product> {
	let src_cts = products.iter()
	                      .map(|p| format!("{}", p.src_ct))
	                      .collect::<Vec<_>>()
	                      .join(", ");
	let dst_cts = products.iter()
	                      .map(|p| format!("{}", p.dst_ct))
	                      .collect::<Vec<_>>()
	                      .join(", ");

	let cargo_targets = products.iter().fold(HashSet::new(), |mut set, product| {
		                                   set.insert(product.name.as_str());
		                                   set
	                                   });
	if cargo_targets.len() > 1 {
		// TODO: instead of this, group them by cargo-target - one or two for single cargo-target.
		let list = cargo_targets.into_iter().collect::<Vec<_>>().join(", ");
		let msg = "Multiple cargo-targets not supported:";
		if !config.compile_options.build_config.keep_going {
			bail!("{msg} [{list}]");
		} else {
			config.log()
			      .error(format!("{msg} [{list}] (sources: {src_cts}, targets: {dst_cts})",));
		}
	}

	config.log().status(
	                    "Packaging",
	                    format!(
		"(multi-target) {} (sources: {src_cts}, targets: {dst_cts})",
		products[0].presentable_name()
	),
	);

	// validation:
	if products.len() > 1 {
		let mut unix = 0;
		for product in products.iter() {
			let info = config.target_info(product.ck)?;
			info.cfg().iter().for_each(|cfg| {
				                 match cfg {
					                 cargo_platform::Cfg::Name(name) if name.as_str() == "unix" => {
					                    unix += 1;
				                    },
				                    _ => {},
				                 }
			                 });
		}

		if unix > 1 {
			let targets = products.iter()
			                      .map(|p| {
				                      match p.ck {
					                      CompileKind::Host => config.host_target,
				                         CompileKind::Target(target) => target,
				                      }.short_name()
				                      .to_string()
			                      })
			                      .collect::<Vec<_>>()
			                      .join(", ");
			let message = format!("Packaging more then two binaries for targets ({targets}) into one package can cause that some of them overwrite some other, so produced package can be broken.");
			config.log().warn(message);

			let message = "Package can contains three binaries: one dylib for unix-family, one dll for windows and one elf for hardware.";
			config.log().note(message);

			let error = "Can't mix two unix-family's dylibs into one package";
			if !config.compile_options.build_config.keep_going {
				bail!(error);
			} else {
				config.log().error(error);
			}
		}
	}

	// cross-target layout:
	let layout_target_name = Name::with_names(package.name().as_str(), products.first().map(|p| &p.name));
	let mut layout =
		CrossTargetLayout::new(config, package.package_id(), Some(layout_target_name))?.lock(config.workspace
		                                                                                           .gctx())?;
	crate::layout::Layout::prepare(&mut layout.as_mut())?;


	if config.compile_options.build_config.force_rebuild {
		// TODO: clean entire product.layout.build() if force rebuild requested
	}


	let mut dev = Default::default();
	for product in &products {
		log::debug!("Preparing binaries for packaging {}", product.presentable_name());
		assert_eq!(package, product.package, "package must be same");
		let dst = layout.build().join(product.path.file_name().expect("file_name"));
		soft_link_checked(&product.path, &dst, true, layout.as_inner().target())?;

		if product.example {
			dev = Some(product);
		}
	}


	// Then the same as for single-product package:
	if let Some(assets) = assets {
		let mut has_dev = false;
		for artifact in assets {
			log::debug!(
			            "Packaging assets {:?} {}",
			            artifact.kind,
			            artifact.package_id.name()
			);

			debug_assert_eq!(
			                 layout.as_ref().assets_layout(config).root(),
			                 artifact.layout.root(),
			                 "wrong layout root"
			);

			has_dev = has_dev || artifact.kind.is_dev();

			prepare_assets(config, artifact, layout.build(), true, layout.as_inner().target())?;
		}
		assert_eq!(dev.is_some(), has_dev);
	}

	// manifest:
	let cargo_target =
		(matches!(products[0].src_ct, CrateType::Bin) || products[0].example).then_some(products[0].name.as_str())
		                                                                     .map(Cow::from);
	build_manifest(
	               config,
	               &layout,
	               package,
	               root.as_source(),
	               cargo_target,
	               products[0].example,
	)?;

	// finally call pdc and pack:
	let mut artifact = execute_pdc(config, &layout)?;
	ar::add_info_meta(&artifact)?;
	if config.zip {
		artifact = ar::build(artifact)?;
	}

	// and back-link to product-layout:
	for product in &products {
		let link = product.layout.artifact();
		// TODO: if zip => try hard-link with fallback to soft-link
		soft_link_checked(&artifact, &link, true, product.layout.root())?;
	}

	let result = Product { package_id: package.package_id(),
	                       name: products[0].name.clone(),
	                       crate_types: products.iter().map(|p| p.src_ct.clone()).collect(),
	                       targets: products.iter().map(|p| p.ck).collect(),
	                       path: artifact.to_path_buf() };
	Ok(result)
}


fn build_manifest<Layout: playdate::layout::Layout>(config: &Config,
                                                    layout: &Layout,
                                                    package: &Package,
                                                    source: impl PackageSource,
                                                    cargo_target: Option<Cow<'_, str>>,
                                                    dev: bool)
                                                    -> CargoResult<()> {
	config.log().verbose(|mut log| {
		            let msg = format!("building package manifest for {}", package.package_id());
		            log.status("Manifest", msg);
	            });

	use ::playdate::metadata::validation::Problem;
	let log_problem = |pre: &str, problem: Problem| {
		let msg = format!("{pre}: {problem}");
		if problem.is_err() {
			config.log().error(msg);
		} else {
			config.log().warn(msg);
		}
	};
	let log_src_problem = |problem: Problem| log_problem("Manifest validation", problem);
	let log_meta_problem = |problem: Problem| log_problem("Metadata validation", problem);


	// validate the source:
	if let Some(target) = &cargo_target {
		source.validate_for(target)
			.into_iter()
			// .filter(Problem::is_err)
			.for_each(log_meta_problem);
	} else {
		source.validate()
			.into_iter()
			// .filter(Problem::is_err)
			.for_each(log_meta_problem);
	}

	let manifest = source.manifest_override_or_crate(cargo_target.as_deref(), dev);
	// validation, lints
	manifest.validate().into_iter().for_each(log_src_problem);

	std::fs::write(layout.manifest(), manifest.to_manifest_string()?)?;
	Ok(())
}


fn execute_pdc<'l, Layout: playdate::layout::Layout>(config: &Config,
                                                     layout: &'l Layout)
                                                     -> CargoResult<Cow<'l, Path>> {
	// TODO: Maybe use products[0].profile here too
	let (optimized, debuginfo) = match config.compile_options.build_config.requested_profile.as_ref() {
		"dev" => (false, true),
		"release" => (true, false),
		_ => {
			// in case of custom profile:
			let profiles = Profiles::new(
			                             &config.workspace,
			                             config.compile_options.build_config.requested_profile,
			)?;
			let profile = profiles.base_profile();
			let optimized = profile.opt_level.as_str() == "0";
			let debuginfo = match profile.debuginfo {
				DebugInfo::Resolved(TomlDebugInfo::None) => false,
				DebugInfo::Deferred(TomlDebugInfo::None) => false,
				DebugInfo::Resolved(_) | DebugInfo::Deferred(_) => true,
			};
			(optimized, debuginfo)
		},
	};

	let mut cmd = Command::new(config.sdk()?.pdc());
	if optimized || !debuginfo {
		cmd.arg("--strip");
	}
	let dst = layout.artifact();

	if dst.exists() {
		if dst.is_dir() {
			cargo_util::paths::remove_dir_all(&dst)?;
		} else {
			cargo_util::paths::remove_file(&dst)?;
		}
	}

	cmd.arg("-v").arg(layout.build().as_os_str()).arg(dst.as_os_str());
	cmd_logged(config, cmd)?.status()?.exit_ok()?;

	Ok(dst)
}


fn prepare_assets<Dst: AsRef<Path>>(config: &Config,
                                    assets: &AssetsArtifactNew,
                                    dst: Dst,
                                    overwrite: bool,
                                    root: impl AsRef<Path>)
                                    -> CargoResult<()> {
	// Choose what assets we will use:
	// - `assets.layout.build` points to pre-built assets, but that can fail
	// - `assets.layout.assets` points to original assets, so use it as fallback

	let dev = assets.kind.is_dev();

	let filter_hidden = |path: &PathBuf| {
		if let Some(filename) = path.file_name() {
			!filename.starts_with(".")
		} else {
			false
		}
	};

	let select_files_in = |build: &Path, assets: &Path| -> CargoResult<_> {
		let mut files: Vec<_> = build.read_dir()?
		                             .filter_map(|entry| entry.ok())
		                             .map(|entry| entry.path())
		                             .filter(filter_hidden)
		                             .collect();
		if files.is_empty() {
			log::debug!("No pre-built assets found, using original assets instead");
			files.extend(assets.read_dir()?
			                   .filter_map(|entry| entry.ok())
			                   .map(|entry| entry.path())
			                   .filter(filter_hidden));
		}
		Ok(files)
	};

	let link_assets = |files: Vec<PathBuf>| -> CargoResult<_> {
		for src in files {
			let dst_name = src.file_name()
			                  .ok_or_else(|| anyhow!("Missed file name in {}", src.display()))?;
			let dst = dst.as_ref().join(dst_name);
			soft_link_checked(&src, &dst, overwrite, root.as_ref())?;
			log::debug!("Asset {} prepared", src.as_relative_to_root(config).display());
		}
		Ok(())
	};

	let (assets_src, assets_build) = if dev {
		(assets.layout.assets_dev(), assets.layout.build_dev())
	} else {
		(assets.layout.assets(), assets.layout.build())
	};

	let files: Vec<_> = select_files_in(&assets_build, &assets_src)?;
	link_assets(files)?;

	Ok(())
}


#[allow(dead_code)]
struct SuccessfulBuildProduct<'cfg> {
	package: &'cfg Package,

	/// Crate-target ID
	name: String,

	src_ct: CrateType,
	dst_ct: CrateType,
	ck: CompileKind,
	profile: ArtifactProfile,

	path: PathBuf,
	layout: ForTargetLayout<PathBuf>,

	example: bool,
}

impl SuccessfulBuildProduct<'_> {
	pub fn presentable_name(&self) -> Cow<'_, str> {
		if self.package.name().as_str() == self.name.as_str() {
			Cow::from(self.name.as_str())
		} else {
			Cow::from(format!("{}:{}", self.package.name(), self.name))
		}
	}
}

impl<'cfg> TryFrom<BuildProduct<'cfg>> for SuccessfulBuildProduct<'cfg> {
	type Error = ();

	fn try_from(product: BuildProduct<'cfg>) -> Result<Self, Self::Error> {
		match product {
			BuildProduct::Success { package,
			                        name,
			                        src_ct,
			                        dst_ct,
			                        ck,
			                        profile,
			                        path,
			                        layout,
			                        example, } => {
				Ok(Self { package,
				          name,
				          src_ct,
				          dst_ct,
				          ck,
				          profile,
				          path,
				          layout,
				          example })
			},
			BuildProduct::Skip { .. } => Err(()),
		}
	}
}
