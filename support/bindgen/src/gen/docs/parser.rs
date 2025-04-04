//! Playdate SDK C-API docs parser
//!
//! Util that parses `Inside Playdate with C.html` file and produces map with
//! keys like `sound.effect.bitCrusher.setAmountModulator`
//! and values with doc in markdown format.
//!
//! Used for generating doc-comments for bindings.


use std::io::{Error as IoError, ErrorKind};
use std::borrow::BorrowMut;
use markup5ever_rcdom::NodeData;
use markup5ever_rcdom::RcDom;

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeSink;
use markup5ever_rcdom::Handle;
use markup5ever_rcdom::SerializableHandle;
use utils::toolchain::sdk::Sdk;

use crate::Result;
use super::DocsMap;


pub fn parse(sdk: &Sdk) -> Result<DocsMap> {
	let path = sdk.path().join("Inside Playdate with C.html");
	parse_file(&path).map_err(Into::into)
}


pub fn parse_file(path: &std::path::Path) -> Result<DocsMap, IoError> {
	if !path.try_exists()? {
		return Err(IoError::new(ErrorKind::NotFound, path.display().to_string()));
	}

	let mut results = DocsMap::new();
	let dom = parse_document(RcDom::default(), Default::default()).from_utf8()
	                                                              .from_file(path)?
	                                                              .finish();
	if !dom.errors.is_empty() {
		eprintln!("errors: {:#?}", dom.errors);
	}

	walk(&dom.document, &mut results);

	#[cfg(feature = "log")]
	for (k, _) in &results {
		println!("Doc found for {k}");
	}

	Ok(results)
}


// TODO: optimize, use Cow instead of String.
fn walk(handle: &Handle, results: &mut DocsMap) {
	let node = handle;
	let mut found = None;
	match node.data {
		NodeData::Element { ref name, ref attrs, .. } => {
			found = if name.local == *"div" {
				let attrs = attrs.borrow();
				let attr = attrs.iter()
				                .find(|attr| attr.name.local == *"id" && attr.value.starts_with("f-"));
				attr.map(|attr| {
					    attr.value
					        .strip_prefix("f-")
					        .expect("prefix 'f-' must be there")
					        .to_string()
				    })
			} else {
				None
			};

			if let Some(_key) = found.as_ref() {
				// Changing: this-div . div_class="title" is a cpp-code fn-path+definition
				//                  to `<code data-lang="c">`
				// TODO: also fix links like `<a href="#f-sound.source">SoundSource</a>`
				let mut children = node.children.borrow_mut();
				let title = children.iter_mut().find(|child| {
					                               match &child.data {
						                               NodeData::Element { name, attrs, .. } => {
						                                  name.local == *"div" &&
						                                  attrs.borrow()
						                                       .iter()
						                                       .find(|attr| {
							                                       attr.name.local == *"class" &&
							                                       attr.value.contains("title")
						                                       })
						                                       .is_some()
					                                  },
					                                  _ => false,
					                               }
				                               });
				if let Some(title) = title {
					let mut data = {
						match &title.data {
							NodeData::Element { name,
							                    attrs,
							                    template_contents,
							                    mathml_annotation_xml_integration_point, } => {
								let mut code = name.clone();
								code.borrow_mut().local = html5ever::LocalName::from("code");
								NodeData::Element { name: code,
								                    attrs: attrs.clone(),
								                    template_contents: template_contents.clone(),
								                    mathml_annotation_xml_integration_point:
									                    *mathml_annotation_xml_integration_point }.into()
							},
							_ => None,
						}
					};

					if let Some(data) = data.take() {
						unsafe {
							std::rc::Rc::get_mut_unchecked(title).data = data;
						}
					}
				}
			}
		},

		_ => {},
	}

	for child in node.children.borrow().iter().filter(|child| {
		                                          match child.data {
			                                          NodeData::Text { .. } | NodeData::Element { .. } => true,
		                                             _ => false,
		                                          }
	                                          })
	{
		walk(child, results);
	}

	if let Some(key) = found {
		let document: SerializableHandle = node.clone().into();
		let mut render = Vec::new();
		html5ever::serialize(&mut render, &document, Default::default()).ok()
		                                                                .expect("serialization failed");
		let html = std::str::from_utf8(&render).unwrap();
		let mut node = html2md::parser::safe_parse_html(html.to_owned()).expect("parsing failed");
		if let Some(node) = node.children.first_mut() {
			if node.tag_name == Some(html2md::structs::NodeType::Code) &&
			   node.attributes
			       .as_ref()
			       .filter(|a| a.get_class().map(String::as_str) == Some("title"))
			       .is_some()
			{
				node.children.clear();
			}
		}

		let mut md = html2md::to_md::to_md(node);
		md = md.strip_prefix("```\n```\n")
		       .map(ToString::to_string)
		       .unwrap_or(md);
		if !md.trim().is_empty() {
			results.insert(key, md);
		}
	}
}
