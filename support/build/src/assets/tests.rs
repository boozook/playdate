#![cfg(test)]
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::config::Env;
use crate::assets::resolver::Expr;
use crate::metadata::format::PlayDateMetadataAssets;
use super::*;

use resolver::unixish_path_pattern;
use resolver::Match;
use toml::Value;


mod plan {
	use super::*;
	use std::env::temp_dir;


	fn crate_root() -> PathBuf { PathBuf::from(env!("CARGO_MANIFEST_DIR")) }


	fn prepared_tmp(test_name: &str) -> (PathBuf, PathBuf, [&'static str; 4], Env) {
		let temp = temp_dir().join(env!("CARGO_PKG_NAME"))
		                     .join(env!("CARGO_PKG_VERSION"))
		                     .join(test_name);

		let sub = temp.join("dir");

		if !temp.exists() {
			println!("creating temp dir: {temp:?}")
		} else {
			println!("temp dir: {temp:?}")
		}
		std::fs::create_dir_all(&temp).unwrap();
		std::fs::create_dir_all(&sub).unwrap();

		// add temp files
		let files = ["foo.txt", "bar.txt", "dir/baz.txt", "dir/boo.txt"];
		for name in files {
			std::fs::write(temp.join(name), []).unwrap();
		}

		let env = {
			let mut env = Env::default().unwrap();
			env.vars.insert("TMP".into(), temp.to_string_lossy().into_owned());
			env.vars.insert("SUB".into(), sub.to_string_lossy().into_owned());
			env
		};

		(temp, sub, files, env)
	}


	mod list {
		use super::*;


		mod as_is {
			use super::*;


			#[test]
			fn local_exact() {
				let env = Env::default().unwrap();
				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();

				let exprs = tests.iter().map(|s| s.to_string()).collect();
				let assets = PlayDateMetadataAssets::List::<toml::Value>(exprs);

				let plan = build_plan(&env, &assets, &opts, root).unwrap();

				for pair in plan.as_inner() {
					assert!(matches!(
						pair,
						Mapping::AsIs(_, (Expr::Original(left), Expr::Original(right)))
						if right == "true" && tests.contains(left.as_str())
					));
				}
			}


			#[test]
			fn resolve_local_abs() {
				let env = {
					let mut env = Env::default().unwrap();
					env.vars.insert(
					                "SRC_ABS".into(),
					                concat!(env!("CARGO_MANIFEST_DIR"), "/src").into(),
					);
					env
				};

				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				let tests: HashMap<_, _> = {
					let man_abs = PathBuf::from("Cargo.toml").canonicalize()
					                                         .unwrap()
					                                         .to_string_lossy()
					                                         .to_string();
					let lib_abs = PathBuf::from("src/lib.rs").canonicalize()
					                                         .unwrap()
					                                         .to_string_lossy()
					                                         .to_string();
					vec![
					     ("${CARGO_MANIFEST_DIR}/Cargo.toml", man_abs),
					     ("${SRC_ABS}/lib.rs", lib_abs),
					].into_iter()
					.collect()
				};

				let exprs = tests.keys().map(|s| s.to_string()).collect();
				let assets = PlayDateMetadataAssets::List::<toml::Value>(exprs);

				let plan = build_plan(&env, &assets, &opts, root).unwrap();

				for pair in plan.as_inner() {
					assert!(matches!(
						pair,
						Mapping::AsIs(matched, (Expr::Modified{original, actual}, Expr::Original(right)))
						if right == "true"
						&& tests[original.as_str()] == actual.as_ref()
						&& matched.source() == Path::new(&tests[original.as_str()]).canonicalize().unwrap()
					));
				}
			}


			#[test]
			fn resolve_local() {
				let env = {
					let mut env = Env::default().unwrap();
					env.vars.insert("SRC".into(), "src".into());
					env
				};

				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				let tests: HashMap<_, _> = { vec![("${SRC}/lib.rs", "src/lib.rs"),].into_iter().collect() };

				let exprs = tests.keys().map(|s| s.to_string()).collect();
				let assets = PlayDateMetadataAssets::List::<toml::Value>(exprs);

				let plan = build_plan(&env, &assets, &opts, root).unwrap();

				for pair in plan.as_inner() {
					if let Mapping::AsIs(matched, (Expr::Modified { original, actual }, Expr::Original(right))) = pair
					{
						assert_eq!("true", right);
						assert_eq!(tests[original.as_str()], actual.as_ref());
						assert_eq!(
						           matched.source().canonicalize().unwrap(),
						           Path::new(&tests[original.as_str()]).canonicalize().unwrap()
						);
						assert_eq!(matched.target(), Path::new(&tests[original.as_str()]));
					} else {
						panic!("pair is not matching: {pair:#?}");
					}
				}
			}


			#[test]
			#[cfg_attr(windows, should_panic)]
			fn resolve_exact_external_abs() {
				let (temp, sub, _files, env) = prepared_tmp("as_is-resolve_external");

				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());


				// tests:

				let tests: HashMap<_, _> = {
					vec![
					     ("${TMP}/foo.txt", (temp.join("foo.txt"), "foo.txt")),
					     ("${TMP}/bar.txt", (temp.join("bar.txt"), "bar.txt")),
					     ("${SUB}/baz.txt", (sub.join("baz.txt"), "baz.txt")),
					     ("${TMP}/dir/boo.txt", (sub.join("boo.txt"), "boo.txt")),
					].into_iter()
					.collect()
				};

				let exprs = tests.keys().map(|s| s.to_string()).collect();
				let assets = PlayDateMetadataAssets::List::<toml::Value>(exprs);

				let plan = build_plan(&env, &assets, &opts, root).unwrap();

				// check targets len
				{
					let targets = plan.targets().collect::<Vec<_>>();
					let expected = tests.values().map(|(_, name)| name).collect::<Vec<_>>();
					assert_eq!(expected.len(), targets.len());
				}

				// full check
				for pair in plan.as_inner() {
					if let Mapping::AsIs(matched, (Expr::Modified { original, actual }, Expr::Original(right))) = pair
					{
						assert_eq!("true", right);
						assert_eq!(tests[original.as_str()].0.to_string_lossy(), actual.as_ref());
						assert_eq!(matched.source(), tests[original.as_str()].0);
						assert_eq!(matched.target().to_string_lossy(), tests[original.as_str()].1);
					} else {
						panic!("pair is not matching: {pair:#?}");
					}
				}
			}


			#[test]
			#[cfg_attr(windows, should_panic)]
			fn resolve_glob_external_many() {
				let (_, _, files, env) = prepared_tmp("as_is-resolve_external_many");

				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				let exprs = ["${TMP}/*.txt", "${SUB}/*.txt"];

				let assets =
					PlayDateMetadataAssets::List::<toml::Value>(exprs.iter().map(|s| s.to_string()).collect());

				let plan = build_plan(&env, &assets, &opts, root).unwrap();

				// check targets len
				{
					let targets = plan.targets().collect::<Vec<_>>();
					assert_eq!(files.len(), targets.len());
				}

				// full check
				for pair in plan.as_inner() {
					if let Mapping::AsIs(matched, (Expr::Modified { original, actual }, Expr::Original(right))) = pair
					{
						assert!(exprs.contains(&original.as_str()));
						assert!(Path::new(actual.as_ref()).is_absolute());
						assert_eq!("true", right);

						if let Match::Pair { source, target } = matched {
							// target is just filename:
							assert_eq!(1, target.components().count());
							assert_eq!(target.file_name(), source.file_name());
						} else {
							panic!("pair.matched is not matching: {matched:#?}");
						}
					} else {
						panic!("pair is not matching: {pair:#?}");
					}
				}
			}
		}
	}


	mod map {
		use super::*;


		mod as_is {
			use super::*;


			#[test]
			fn local_exact() {
				let env = Env::default().unwrap();
				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();

				let exprs = tests.iter()
				                 .map(|s| (s.to_string(), Value::Boolean(true)))
				                 .collect();
				let assets = PlayDateMetadataAssets::Map::<Value>(exprs);

				let plan = build_plan(&env, &assets, &opts, root).unwrap();

				for pair in plan.as_inner() {
					if let Mapping::AsIs(matched, (Expr::Original(left), Expr::Original(right))) = pair {
						assert_eq!("true", right);
						assert!(tests.contains(left.as_str()));
						assert_eq!(
						           left.as_str(),
						           unixish_path_pattern(matched.target().to_string_lossy().as_ref())
						);
					} else {
						panic!("pair is not matching: {pair:#?}");
					}
				}
			}


			#[test]
			fn local_exact_target() {
				let env = Env::default().unwrap();
				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				// left hand of rule:
				let targets = ["trg", "/trg", "//trg"];
				// right hand of rule:
				let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();
				// latest because there is no to files into one target, so "into" will be used

				for trg in targets {
					let stripped_trg = &trg.replace('/', "").trim().to_owned();

					let exprs = tests.iter()
					                 .map(|s| (trg.to_string(), Value::String(s.to_string())))
					                 .collect();
					let assets = PlayDateMetadataAssets::Map::<Value>(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						if let Mapping::AsIs(
						                     Match::Pair { source, target },
						                     (Expr::Original(left), Expr::Original(right)),
						) = pair
						{
							assert_eq!(left, stripped_trg);
							assert!(tests.contains(right.as_str()));
							assert_eq!(source, Path::new(right));
							assert_eq!(target, Path::new(stripped_trg));
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}
			}
		}


		mod one_into {
			use super::*;


			#[test]
			fn local_exact_target() {
				let env = Env::default().unwrap();
				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				// left hand of rule:
				let targets = ["trg/", "trg//", "/trg/", "//trg/"];
				let targets_rel = ["trg/", "trg//"]; // non-abs targets
				// right hand of rule:
				let tests: HashSet<_> = vec!["Cargo.toml", "src/lib.rs"].into_iter().collect();

				for trg in targets {
					let exprs = tests.iter()
					                 .map(|s| (trg.to_string(), toml::Value::String(s.to_string())))
					                 .collect();
					let assets = PlayDateMetadataAssets::Map::<toml::Value>(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						if let Mapping::Into(
						                     Match::Pair { source, target },
						                     (Expr::Original(left), Expr::Original(right)),
						) = pair
						{
							assert_eq!(left, target.to_string_lossy().as_ref());
							assert!(targets_rel.contains(&left.as_str()));
							assert!(tests.contains(right.as_str()));
							assert_eq!(source, Path::new(right));
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}
			}
		}


		mod many_into {
			use super::*;

			#[test]
			#[cfg_attr(windows, should_panic)]
			fn glob_local_target() {
				let env = Env::default().unwrap();
				let opts = AssetsOptions::default();

				let root = crate_root();
				let root = Some(root.as_path());

				// left hand of rule:
				let targets = ["/trg/", "//trg/", "/trg", "trg"];
				let targets_rel = ["trg/", "trg"]; // non-abs targets
				// right hand of rule:
				let tests: HashSet<_> = vec!["Cargo.tom*", "src/lib.*"].into_iter().collect();
				// latest because there is no to files into one target, so "into" will be used

				for trg in targets {
					let exprs = tests.iter()
					                 .map(|s| (trg.to_string(), toml::Value::String(s.to_string())))
					                 .collect();
					let assets = PlayDateMetadataAssets::Map::<toml::Value>(exprs);

					let plan = build_plan(&env, &assets, &opts, root).unwrap();

					for pair in plan.as_inner() {
						if let Mapping::ManyInto { sources,
						                           target,
						                           #[cfg(feature = "assets-report")]
						                           excluded,
						                           exprs: (Expr::Original(left), Expr::Original(right)), } = pair
						{
							assert!(targets_rel.contains(&target.to_string_lossy().as_ref()));
							assert_eq!(&target.to_string_lossy(), left);

							assert_eq!(1, sources.len());
							assert!(tests.contains(right.as_str()));

							#[cfg(feature = "assets-report")]
							assert_eq!(0, excluded.len());
						} else {
							panic!("pair is not matching: {pair:#?}");
						}
					}
				}
			}
		}
	}
}
