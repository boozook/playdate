#[allow(unused)]
type Result<T = (), E = Box<dyn std::error::Error>> = std::result::Result<T, E>;


#[cfg(all(feature = "serde_json", feature = "toml"))]
pub fn build_json(toml: &str, link_script: Option<impl ToString>) -> Result<serde_json::Value> {
	let toml = build_toml(toml, link_script)?;
	serde_json::to_value(toml).map_err(Into::into)
}

#[cfg(all(feature = "serde_json", feature = "toml"))]
pub fn write_json_from_toml(toml: &str,
                            link_script: Option<impl ToString>,
                            to: impl std::io::Write,
                            pretty: bool)
                            -> Result<()> {
	let spec = build_json(toml, link_script)?;

	if pretty {
		serde_json::to_writer_pretty(to, &spec)
	} else {
		serde_json::to_writer(to, &spec)
	}.map_err(Into::into)
}


#[cfg(feature = "toml")]
pub fn build_toml(src: &str, link_script: Option<impl ToString>) -> Result<toml::Value> {
	let mut toml: toml::Value = toml::from_str(src)?;
	if let Some(src) = link_script {
		toml["link-script"] = toml::Value::String(src.to_string());
	}
	// else { toml.as_table_mut().and_then(|t| t.remove("link-script")); }
	Ok(toml)
}


#[cfg(feature = "toml_edit")]
/// Build using toml-edit
pub fn build_toml_edit(src: &str, link_script: Option<impl ToString>) -> Result<toml_edit::DocumentMut> {
	use toml_edit::DocumentMut;

	let mut toml = src.parse::<DocumentMut>()?;
	if let Some(src) = link_script {
		toml["link-script"] = toml_edit::value(src.to_string());
	}
	// else { toml.as_table_mut().and_then(|t| t.remove("link-script")); }
	Ok(toml)
}
