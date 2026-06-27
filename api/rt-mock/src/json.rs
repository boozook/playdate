use crate::ffi::PlaydateJson;


pub static JSON: PlaydateJson = PlaydateJson { initEncoder: None,
                                               decode: None,
                                               decodeString: None };
