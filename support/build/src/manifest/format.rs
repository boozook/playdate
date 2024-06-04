pub use crate::metadata::format::Manifest;
use crate::metadata::source::ManifestSourceOptExt;


pub trait ManifestFmt {
	fn write_to<W: std::io::Write>(&self, to: &mut W) -> std::io::Result<()>
		where Self: ManifestSourceOptExt {
		let mut buf = String::new();
		self.write_to_fmt(&mut buf).map_err(std::io::Error::other)?;
		to.write_all(buf.as_bytes())
	}


	fn write_to_fmt<W: std::fmt::Write>(&self, to: &mut W) -> std::fmt::Result
		where Self: ManifestSourceOptExt {
		let data = self;

		let is_not_empty = |s: &&str| !s.trim().is_empty();

		{
			let mut write_fmt = |k: &str, v: &str| to.write_fmt(format_args!("{}={}\n", k.trim(), v.trim()));

			if let Some(s) = data.name().filter(is_not_empty) {
				write_fmt("name", s)?;
			}
			if let Some(s) = data.version().filter(is_not_empty) {
				write_fmt("version", s)?
			}
			if let Some(s) = data.author().filter(is_not_empty) {
				write_fmt("author", s)?
			}
			if let Some(s) = data.bundle_id().filter(is_not_empty) {
				write_fmt("bundleID", s)?
			}
			if let Some(s) = data.description().filter(is_not_empty) {
				write_fmt("description", s)?
			}
			if let Some(s) = data.image_path().filter(is_not_empty) {
				write_fmt("imagePath", s)?
			}
			if let Some(s) = data.launch_sound_path().filter(is_not_empty) {
				write_fmt("launchSoundPath", s)?
			}
			if let Some(s) = data.content_warning().filter(is_not_empty) {
				write_fmt("contentWarning", s)?
			}
			if let Some(s) = data.content_warning2().filter(is_not_empty) {
				write_fmt("contentWarning2", s)?
			}
		}

		if let Some(v) = data.build_number() {
			to.write_fmt(format_args!("{}={}\n", "buildNumber", v))?
		}

		if let Some(extra) = data.iter_extra() {
			for (key, value) in extra.into_iter() {
				let (key, value) = (key.as_ref(), value.as_ref());
				if is_not_empty(&key) && !value.is_empty() {
					to.write_fmt(format_args!("{}={}\n", key.trim(), value))?
				}
			}
		}

		Ok(())
	}


	fn to_manifest_string(&self) -> Result<String, std::fmt::Error>
		where Self: ManifestSourceOptExt {
		let mut buf = String::new();
		self.write_to_fmt(&mut buf)?;
		Ok(buf)
	}
}


impl<T: ManifestSourceOptExt> ManifestFmt for T {}


#[cfg(test)]
mod tests {
	use crate::metadata::format::Ext;
	use crate::metadata::format::ExtraFields;
	use super::ManifestFmt;
	use super::Manifest;


	#[test]
	fn fmt_empty() {
		let m = Manifest::<&str>::default();
		assert!(m.to_manifest_string().unwrap().is_empty());
	}

	#[test]
	fn fmt_full() {
		let m = Manifest::<&str> { name: "name".into(),
		                           version: "version".into(),
		                           author: "author".into(),
		                           bundle_id: "bundle_id".into(),
		                           description: "description".into(),
		                           image_path: "image_path".into(),
		                           launch_sound_path: "launch_sound_path".into(),
		                           content_warning: "content_warning".into(),
		                           content_warning2: "content_warning2".into(),
		                           build_number: 42.into() };
		let s = m.to_manifest_string().unwrap();
		assert_eq!(
		           "name=name\nversion=version\nauthor=author\nbundleID=bundle_id\ndescription=description\nimagePath=image_path\nlaunchSoundPath=launch_sound_path\ncontentWarning=content_warning\ncontentWarning2=content_warning2\nbuildNumber=42\n",
		           s
		);
	}

	#[test]
	fn fmt_ext() {
		let main = Manifest::<&str> { bundle_id: "bundle_id".into(),
		                              ..Default::default() };
		let mut extra = ExtraFields::new();
		extra.insert("foo".to_owned(), "bar".to_owned().into());

		let m = Ext::new(main, extra);
		let s = m.to_manifest_string().unwrap();
		assert_eq!("bundleID=bundle_id\nfoo=bar\n", s);
	}

	#[test]
	fn fmt_trim() {
		let main = Manifest::<&str> { name: "  name".into(),
		                              bundle_id: "bundle_id   ".into(),
		                              description: "   description   ".into(),
		                              ..Default::default() };
		let mut extra = ExtraFields::new();
		extra.insert("   foo    ".to_owned(), "  bar  ".to_owned().into());

		let m = Ext::new(main, extra);
		let s = m.to_manifest_string().unwrap();
		assert_eq!(
		           "name=name\nbundleID=bundle_id\ndescription=description\nfoo=bar\n",
		           s
		);
	}
}
