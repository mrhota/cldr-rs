#![allow(dead_code)]

use std::{fs, path};
use std::io::Read;

use error::Result;

use serde;
use serde_json;
use bzip2::read::BzDecoder;

fn read_cldr_data<T: serde::Deserialize>(p: &path::Path, pointer: &str) -> Result<T> {
    let f = try!(fs::File::open(p));
    let mut decompressor = BzDecoder::new(f);
    let mut contents = String::new();
    try!(decompressor.read_to_string(&mut contents));
    let json: serde_json::value::Value = try!(serde_json::from_str(&contents));
    try!(serde_json::from_value(json.pointer(pointer)
                                    .cloned()
                                    .unwrap_or(serde_json::Value::Null)))
}
