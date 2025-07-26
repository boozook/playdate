#![cfg(feature = "documentation")]

use std::collections::HashMap;

pub mod parser;
pub mod r#gen;


pub type DocsMap = HashMap<String, String>;
