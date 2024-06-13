use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use anyhow::bail;
use cargo::core::PackageId;
use cargo::CargoResult;
use playdate::fs::soft_link_checked;
use playdate::layout::Layout;

use crate::config::Config;
use crate::layout::PlaydateAssets;
use crate::proc::logging::cmd_logged;

use super::Kind;


pub fn build(config: &Config,
             package_id: &PackageId,
             layout: &PlaydateAssets<PathBuf>,
             kind: Kind)
             -> CargoResult<()> {
	let (src, build) = match kind {
		Kind::Package => {
			let src = layout.assets();
			let build = layout.build();
			(src, build)
		},
		Kind::Dev => {
			let src = layout.assets_dev();
			let build = layout.build_dev();
			std::fs::create_dir(&build).ok();
			(src, build)
		},
	};

	build_in(config, package_id, &src, &build, &layout.dest())
}

fn build_in(config: &Config, package_id: &PackageId, src: &Path, build: &Path, root: &Path) -> CargoResult<()> {
	config.log()
	      .status("Compiling", format!("assets for {}", package_id));

	if config.no_sdk {
		bail!("Build without Playdate SDK is not supported yet.");
	}

	let prefix = src.file_name()
	                .map(|s| format!(".{}", s.to_string_lossy()))
	                .unwrap_or_default();
	let dst = build.with_file_name(format!("{prefix}.build.pdx"));

	soft_link_checked(build, &dst, true, root)?;

	// prepare src for pdc:
	std::fs::write(src.join("pdex.bin"), [])?;

	let mut cmd = Command::new(config.sdk()?.pdc());
	if config.skip_unknown {
		cmd.arg("--skip-unknown");
	}
	cmd.arg("-v").arg(src.as_os_str()).arg(dst.as_os_str());
	let status = cmd_logged(config, cmd)?.status()?;

	// remove temp files before possible failure:
	fs_extra::remove_items(&[
		src.join("pdex.bin"),
		build.join("pdex.bin"),
		build.join("pdxinfo"),
	])?;
	status.exit_ok()?;
	Ok(())
}
