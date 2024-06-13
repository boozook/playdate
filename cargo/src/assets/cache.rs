use std::collections::BTreeMap;
use std::path::PathBuf;

use cargo::CargoResult;
use playdate::assets::plan::BuildPlan;

pub struct PlanCache {
	pub difference: Difference,
	pub serialized: Option<String>,
	pub path: PathBuf,
}


#[derive(Debug, Clone, Copy, serde::Serialize)]
pub enum Difference {
	Same,
	Different,
	/// There is not cache file.
	Missing,
}

impl Difference {
	pub fn is_same(&self) -> bool { matches!(self, Self::Same) }
}


#[must_use = "Cached plan must be used"]
pub fn plan_cache(path: PathBuf, plan: &BuildPlan<'_, '_>) -> CargoResult<PlanCache> {
	let mut serializable = plan.iter_flatten_meta().collect::<Vec<_>>();
	serializable.sort();

	#[derive(serde::Serialize)]
	struct SerializablePlan<'t> {
		items: &'t [(playdate::assets::plan::MappingKind, PathBuf, (PathBuf, Option<std::time::SystemTime>))],
		env: &'t BTreeMap<String, String>,
	}

	let serializable = SerializablePlan { items: &serializable,
	                                      env: plan.used_env_vars() };
	let json = serde_json::to_string(&serializable)?;

	let difference = if path.try_exists()? {
		if std::fs::read_to_string(&path)? == json {
			log::debug!("Cached plan is the same");
			Difference::Same
		} else {
			log::debug!("Cache mismatch, need diff & rebuild");
			Difference::Different
		}
	} else {
		log::debug!("Cache mismatch, full rebuilding");
		Difference::Missing
	};

	let serialized = (!difference.is_same()).then_some(json);

	Ok(PlanCache { path,
	               difference,
	               serialized })
}
