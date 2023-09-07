use cargo::core::compiler::CompileKind;
use cargo::util::Rustc;


pub trait TargetSpec {
	fn target_spec(&self, kind: &CompileKind) -> anyhow::Result<format::TargetSpec>;
}


impl TargetSpec for Rustc {
	fn target_spec(&self, kind: &CompileKind) -> anyhow::Result<format::TargetSpec> {
		let mut proc = self.process_no_wrapper();

		match kind {
			CompileKind::Host => (),
			CompileKind::Target(target) => {
				proc.arg("--target").arg(target.rustc_target());
			},
		}
		proc.args(&["--print=target-spec-json", "-Zunstable-options"]);

		let extra_fingerprint = kind.fingerprint_hash();
		let (stdout, stderr) = self.cached_output(&proc, extra_fingerprint)?;
		if !stderr.trim().is_empty() {
			log::error!("{}", stderr);
		}

		let spec: format::TargetSpec = serde_json::from_str(&stdout)?;
		Ok(spec)
	}
}


pub mod format {
	#[derive(serde::Deserialize, Debug)]
	#[serde(rename_all = "kebab-case")]
	pub struct TargetSpec {
		pub dll_prefix: Option<String>,
		pub dll_suffix: Option<String>,
		pub staticlib_prefix: Option<String>,
		pub staticlib_suffix: Option<String>,

		pub target_family: Option<Vec<String>>,
	}
}
