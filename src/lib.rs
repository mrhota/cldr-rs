#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate bzip2;

mod util;
mod tests;

pub mod core;
pub mod error;
