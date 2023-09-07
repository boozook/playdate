#![cfg(feature = "documentation")]

use std::collections::HashMap;

pub mod parser;
pub mod gen;


pub type DocsMap = HashMap<String, String>;
