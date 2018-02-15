#![feature(proc_macro)]

pub mod core;
pub mod error;
mod tests;
mod util;

extern crate bzip2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
