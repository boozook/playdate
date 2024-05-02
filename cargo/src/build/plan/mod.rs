use cargo::CargoResult;
use cargo::core::PackageId;
use cargo::util::command_prelude::CompileMode;
use crate::cli::cmd::Cmd;
use crate::config::Config;
use crate::proc::args_line_for_proc;
use crate::proc::cargo_proxy_cmd;
use self::format::TargetKind;

pub mod format;


pub fn build_plan(cfg: &Config) -> CargoResult<format::BuildPlan> {
	let config = cfg.workspace.config();
	let mut cargo = cargo_proxy_cmd(cfg, &Cmd::Build)?;

	if !cfg.compile_options.build_config.build_plan {
		cargo.args(["--build-plan", "-Zunstable-options"]);
	}

	cfg.log()
	   .verbose(|mut log| log.status("Cargo", args_line_for_proc(&cargo)));

	let output = cargo.output()?;
	if !output.status.success() {
		config.shell().err().write_all(&output.stderr)?;
		output.status.exit_ok()?;
	}

	let stdout = std::str::from_utf8(&output.stdout)?;

	// parse only last line of output:
	let line = stdout.lines()
	                 .find(|s| {
		                 let s = s.trim();
		                 !s.is_empty() && s.starts_with('{')
	                 })
	                 .unwrap_or("{}");

	let value: format::BuildPlan = serde_json::de::from_str(line)?;
	Ok(value)
}


impl format::BuildPlan {
	pub fn build_package_invocations<'plan: 'i, 'p: 'i, 'i>(
		&'plan self,
		package: &'p PackageId)
		-> impl Iterator<Item = &'plan format::Invocation> + 'i {
		self.invocations
		    .iter()
		    .filter(move |item| {
			    item.package_name == package.name().as_str() && package.version() == &item.package_version
		    })
		    .filter(|item| item.compile_mode == CompileMode::Build)
	}
}


#[allow(dead_code)]
pub enum TargetKindWild {
	Lib,
	Bin,
	Test,
	Bench,
	ExampleLib,
	ExampleBin,
	CustomBuild,
}

impl PartialEq<TargetKind> for TargetKindWild {
	fn eq(&self, other: &TargetKind) -> bool {
		match self {
			TargetKindWild::Lib => matches!(other, TargetKind::Lib(_)),
			TargetKindWild::Bin => matches!(other, TargetKind::Bin),
			TargetKindWild::Test => matches!(other, TargetKind::Test),
			TargetKindWild::Bench => matches!(other, TargetKind::Bench),
			TargetKindWild::ExampleLib => matches!(other, TargetKind::Example),
			TargetKindWild::ExampleBin => matches!(other, TargetKind::Example),
			TargetKindWild::CustomBuild => matches!(other, TargetKind::CustomBuild),
		}
	}
}
