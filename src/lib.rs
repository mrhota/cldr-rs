#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

use std::path::Path;

extern crate serde;
extern crate serde_json;
extern crate bzip2;

pub mod util;
pub mod core;
pub mod error;
