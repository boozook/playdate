use std::path::PathBuf;
use std::process::Command;

use anyhow::bail;
use cargo::CargoResult;
use cargo::core::Package;
use playdate::io::soft_link_checked;
use playdate::layout::Layout;

use crate::config::Config;
use crate::layout::PlaydateAssets;
use crate::proc::logging::cmd_logged;


pub fn build(config: &Config, package: &Package, layout: &PlaydateAssets<PathBuf>) -> CargoResult<()> {
	config.log()
	      .status("Compiling", format!("assets for {}", package.package_id()));

	if config.no_sdk {
		bail!("Build without Playdate SDK is not supported yet.");
	}

	let src = layout.assets();
	let build = layout.build();
	let dst = build.with_file_name(".build.pdx");

	soft_link_checked(&build, &dst, true, layout.dest())?;

	// prepare src for pdc:
	std::fs::write(&src.join("pdex.bin"), &[])?;

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
