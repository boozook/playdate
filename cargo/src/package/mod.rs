use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::anyhow;
use anyhow::bail;
use cargo::CargoResult;
use cargo::core::Package;
use cargo::core::compiler::CompileKind;
use cargo::core::compiler::CrateType;
use cargo::core::profiles::DebugInfo;
use cargo::core::profiles::Profiles;
use cargo::util::toml::TomlDebugInfo;
use clap_lex::OsStrExt;
use playdate::io::soft_link_checked;
use playdate::layout::Layout;
use playdate::layout::Name;
use playdate::manifest::ManifestDataSource;
use playdate::manifest::format::Manifest;

use crate::assets::AssetsArtifact;
use crate::assets::AssetsArtifacts;
use crate::assets::playdate_metadata;
use crate::build::BuildProduct;
use crate::config::Config;
use crate::layout::CrossTargetLayout;
use crate::layout::LayoutLockable;
use crate::layout::ForTargetLayout;
use crate::proc::logging::cmd_logged;
use crate::proc::reader::format::ArtifactProfile;
use crate::utils::path::AsRelativeTo;

mod ar;


#[derive(Debug)]
pub struct Product<'p> {
	pub package: &'p Package,
	pub crate_types: Vec<CrateType>,
	pub targets: Vec<CompileKind>,
	pub name: String,
	pub path: PathBuf,
}


pub fn build_all<'b>(config: &'_ Config,
                     assets: AssetsArtifacts<'_>,
                     products: Vec<BuildProduct<'b>>)
                     -> CargoResult<Vec<Product<'b>>> {
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
		if products.len() == 1 {
			let assets = assets.get(package);
			let product = products.pop().unwrap();
			let result = package_single_target(config, product, assets)?;
			results.push(result);
		} else if products.len() > 1 {
			let assets = assets.get(package);
			let result = package_multi_target(config, package, products, assets)?;
			results.push(result);
		} else {
			unreachable!("impossible len=0")
		}
	}

	Ok(results)
}


fn package_single_target<'p>(config: &Config,
                             product: SuccessfulBuildProduct<'p>,
                             assets: Option<&AssetsArtifact>)
                             -> CargoResult<Product<'p>> {
	let presentable_name = product.presentable_name();
	config.log().status(
	                    "Packaging",
	                    format!(
		"(single-target) {presentable_name} (source: {}, target: {})",
		product.src_ct, product.dst_ct
	),
	);

	if let Some(assets) = assets {
		assert_eq!(assets.package, product.package);
		log::debug!("Preparing assets for packaging {}", product.presentable_name());

		prepare_assets(
		               config,
		               assets,
		               product.example,
		               product.layout.build(),
		               true,
		               product.layout.root(),
		)?;
	}

	// manifest:
	build_manifest(config, &product.layout, product.package, assets)?;

	// finally call pdc and pack:
	let mut artifact = execute_pdc(config, &product.layout)?;
	if !config.no_info_meta {
		ar::add_info_meta(&artifact)?;
	}
	if config.zip {
		artifact = ar::build(artifact)?;
	}

	let result = Product { name: product.name,
	                       package: product.package,
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
fn package_multi_target<'p>(config: &Config,
                            package: &'p Package,
                            products: Vec<SuccessfulBuildProduct>,
                            assets: Option<&AssetsArtifact>)
                            -> CargoResult<Product<'p>> {
	let src_cts = products.iter()
	                      .map(|p| format!("{}", p.src_ct))
	                      .collect::<Vec<_>>()
	                      .join(", ");
	let dst_cts = products.iter()
	                      .map(|p| format!("{}", p.dst_ct))
	                      .collect::<Vec<_>>()
	                      .join(", ");
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

			let message =
				format!("Package can contains three binaries: one dylib for unix-family, one dll for windows and one elf for hardware.");
			config.log().note(message);

			let error = format!("Can't mix two unix-family's dylibs into one package");
			if !config.compile_options.build_config.keep_going {
				bail!(error);
			} else {
				config.log().error(&error);
			}
		}
	}

	// cross-target layout:
	let layout_target_name = Name::with_names(package.name().as_str(), products.first().map(|p| &p.name));
	let mut layout =
		CrossTargetLayout::new(config, package, Some(layout_target_name))?.lock(config.workspace.config())?;
	if let Some(assets) = assets {
		debug_assert_eq!(
		                 layout.as_ref().assets_layout(config).root(),
		                 assets.layout.root(),
		                 "wrong layout root"
		);
	}
	crate::layout::Layout::prepare(&mut layout.as_mut())?;

	let mut has_dev = Default::default();
	for product in &products {
		log::debug!("Preparing binaries for packaging {}", product.presentable_name());
		assert_eq!(package, product.package, "package must be same");
		let dst = layout.build().join(product.path.file_name().expect("file_name"));
		soft_link_checked(&product.path, &dst, true, layout.as_inner().target())?;

		if product.example {
			has_dev = true;
		}
	}


	// Then the same as for single-product package:
	if let Some(assets) = assets {
		log::debug!("Preparing assets for packaging {}", assets.package.name());
		assert_eq!(package, assets.package, "package must be same");
		prepare_assets(
		               config,
		               assets,
		               has_dev,
		               layout.build(),
		               true,
		               layout.as_inner().target(),
		)?;
	}

	// manifest:
	build_manifest(config, &layout, package, assets)?;

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

	let result = Product { package,
	                       name: products[0].name.clone(),
	                       crate_types: products.iter().map(|p| p.src_ct.clone()).collect(),
	                       targets: products.iter().map(|p| p.ck).collect(),
	                       path: artifact.to_path_buf() };
	Ok(result)
}


fn build_manifest<'l, Layout: playdate::layout::Layout>(config: &Config,
                                                        layout: &'l Layout,
                                                        package: &Package,
                                                        assets: Option<&AssetsArtifact<'_>>)
                                                        -> CargoResult<()> {
	config.log().verbose(|mut log| {
		            let msg = format!("building package manifest for {}", package.package_id());
		            log.status("Manifest", msg);
	            });

	let manifest = if let Some(metadata) = assets.map(|a| a.metadata.as_ref()).flatten() {
		               Manifest::try_from_source(ManifestSource { package,
		                                                          metadata: metadata.into() })
	               } else {
		               let metadata = playdate_metadata(package);
		               Manifest::try_from_source(ManifestSource { package,
		                                                          metadata: metadata.as_ref() })
	               }.map_err(|err| anyhow!(err))?;
	std::fs::write(layout.manifest(), manifest.to_string())?;
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
				DebugInfo::Resolved(_) => true,
				DebugInfo::Deferred(_) => true,
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
                                    assets: &AssetsArtifact,
                                    dev: bool,
                                    dst: Dst,
                                    overwrite: bool,
                                    root: impl AsRef<Path>)
                                    -> CargoResult<()> {
	// Choose what assets we will use:
	// - `assets.layout.build` points to pre-built assets, but that can fail
	// - `assets.layout.assets` points to original assets, so use it as fallback

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


	// Main assets:
	let files: Vec<_> = select_files_in(&assets.layout.build(), &assets.layout.assets())?;
	link_assets(files)?;

	// Dev assets:
	if dev {
		let assets_dev = assets.layout.assets_dev();
		if assets_dev.exists() {
			let files: Vec<_> = select_files_in(&assets.layout.build_dev(), &assets_dev)?;
			link_assets(files)?;
		} else {
			// That's OK, dev-assets can be missing, we doesn't create dir without need.
			log::debug!(
			            "Asset (dev) not found at {}",
			            assets_dev.as_relative_to_root(config).display()
			);
		}
	}
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


struct ManifestSource<'cfg, 'm> {
	package: &'cfg Package,
	metadata: Option<&'m playdate::metadata::format::PlayDateMetadata<toml::Value>>,
}

impl ManifestDataSource for ManifestSource<'_, '_> {
	type Value = toml::Value;

	fn name(&self) -> &str { self.package.name().as_str() }
	fn authors(&self) -> &[String] { &self.package.manifest().metadata().authors }
	fn version(&self) -> Cow<str> { self.package.version().to_string().into() }
	fn description(&self) -> Option<&str> { self.package.manifest().metadata().description.as_deref() }
	fn metadata(&self) -> Option<&playdate::metadata::format::PlayDateMetadata<Self::Value>> { self.metadata }
}
