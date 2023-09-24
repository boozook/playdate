//! Shared consts between tests and test-crates
#![allow(dead_code)]

pub const CARGO_PLAYDATE_TEST_VALUE_ENV: &'static str = "CARGO_PLAYDATE_TEST_VALUE";
pub const CARGO_PLAYDATE_TEST_VALUE: Option<&'static str> = option_env!("CARGO_PLAYDATE_TEST_VALUE");
pub const CARGO_PLAYDATE_TEST_VALUE_PREFIX: &'static str = "TEST";

// For assets / package tests
pub const CARGO_PLAYDATE_TEST_DEV_ASSETS_DIR_ENV: &'static str = "CARGO_PLAYDATE_TEST_DEV_ASSETS_DIR";
pub const CARGO_PLAYDATE_TEST_DEV_ASSETS_DIR: Option<&'static str> =
	option_env!("CARGO_PLAYDATE_TEST_DEV_ASSETS_DIR");
